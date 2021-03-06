/* Groups everything into easily usable lib
   Copyright (C) 2020-2021 Sean DiGirolamo

This file is part of Shellbird.

Shellbird is free software; you can redistribute it and/or modify it
under the terms of the GNU General Public License as published by the
Free Software Foundation; either version 3, or (at your option) any
later version.

Shellbird is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License
along with Shellbird; see the file COPYING.  If not see
<http://www.gnu.org/licenses/>.  */

extern crate mpd;
extern crate termion;
extern crate signal_hook;
extern crate home;
extern crate unicode_width;
extern crate unicode_truncate;
extern crate json;

pub mod event;
pub mod components;
pub mod music;
pub mod layout_config;
pub mod signals;
pub mod color;
pub mod playlist;
pub mod styles;
pub mod command_line;
pub mod mode;
pub mod screen;

use std::error::Error;
use std::io::{self, Stdout, Write, BufRead, BufReader};
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::fs::File;

use mpd::Song;

use termion::raw::RawTerminal;
use termion::{clear, cursor};
use termion::input::TermRead;

use event::*;
use styles::StyleTree;
use music::{mpd_sender, mpd_listener};
use command_line::CommandLine;
use components::{Components, Component};
use screen::Screen;

pub struct GlobalState {
    pub style_tree: Option<StyleTree>,
    pub library: Vec<Song>,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            style_tree: None,
            library: Vec::new(),
        }
    }
}

pub struct Shellbird<'a> {
    genres_path: Option<String>,
    sbrc_path: Option<String>,
    layout_path: Option<String>,
    mpd_ip: &'a str,
    mpd_port: &'a str,
    debug: bool,
}

impl<'a> Shellbird<'a> {
    pub fn new(
        genres_path: Option<String>,
        sbrc_path: Option<String>,
        layout_path: Option<String>,
        mpd_ip: &'a str,
        mpd_port: &'a str,
        debug: bool,
    )  -> Shellbird<'a> {
        Shellbird {
            genres_path,
            sbrc_path,
            layout_path,
            mpd_ip,
            mpd_port,
            debug,
        }
    }

    pub fn run(
        &mut self,
        mut stdout: RawTerminal<Stdout>
    ) -> Result<(), Box<dyn Error>> {
        let mut state = GlobalState::new();

        let mut components = self.init_components();

        let mut screen = Screen::new("Default");

        let (tx, rx) = mpsc::channel();

        let mut command_line = CommandLine::new(tx.clone());

        let mpd_tx =  mpd_sender::init_mpd_sender_thread(
            self.mpd_ip,
            self.mpd_port,
            tx.clone()
        );

        self.init_stdin_thread(tx.clone());
        self.run_sbrc(tx.clone());
        mpd_listener::init_mpd_listener_thread(self.mpd_ip, self.mpd_port, tx.clone());
        signals::init_listener(tx.clone());

        if let Some(path) = &self.genres_path {
            styles::load_style_tree_async(&path, tx.clone());
        }

        print!("{}{}", cursor::Hide, clear::All);

        loop {
            command_line.draw();

            stdout.flush().unwrap();

            let e = rx.recv()?;

            if self.debug {
                eprintln!("{:?}", e);
            }

            match e {
                Event::BindKey(key, e) => command_line.bind(key, e.to_event()),
                Event::ToComponent(name, e) => {
                    if let Some(c) = components.get_mut(&name) {
                        c.handle(&state, &e, tx.clone());
                    }
                },
                Event::ToApp(e) => match e {
                    AppEvent::Quit => break,
                    AppEvent::Error(s) => eprintln!("{}", s),
                    AppEvent::ClearScreen => print!("{}", clear::All),
                    AppEvent::DrawScreen =>
                        tx.send(
                            spawn_draw_screen_event(&screen, &components)
                        ).unwrap(),
                    AppEvent::LostMpdConnection => {
                        state.library = Vec::new();
                        tx.send(Event::ToAllComponents(
                            ComponentEvent::LostMpdConnection
                        )).unwrap();
                    },
                    AppEvent::Database(tracks) => {
                        if let Some(tree) = &mut state.style_tree {
                            tree.set_tracks(tracks.clone());
                        }

                        state.library = tracks.clone();

                        tx.send(Event::ToAllComponents(
                                ComponentEvent::Database(tracks))
                        ).unwrap();
                    },
                    AppEvent::SwitchScreen(name) => {
                        screen.set(&name);
                        tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                    },
                    AppEvent::StyleTreeLoaded(tree) => {
                        state.style_tree = tree;
                        tx.send(Event::ToAllComponents(
                            ComponentEvent::UpdateRootStyleMenu
                        )).unwrap();
                    },
                    AppEvent::Resize =>
                        tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap(),
                },
                Event::ToCommandLine(e) => command_line.handle(&e, tx.clone()),
                Event::ToScreen(e) => match e {
                    ScreenEvent::FocusNext => {
                        screen.focus_next(&mut components);
                        tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                    },
                    ScreenEvent::FocusPrev => {
                        screen.focus_prev(&mut components);
                        tx.send(Event::ToApp(AppEvent::DrawScreen)).unwrap();
                    },
                    ScreenEvent::NeedsRedraw(name) => {
                        if screen.contains(&name, &components) {
                            tx.send(
                                Event::ToApp(AppEvent::DrawScreen)
                            ).unwrap();
                        }
                    },
                },
                Event::ToAllComponents(e) => {
                    for c in components.values_mut() {
                        c.handle(&state, &e, tx.clone())
                    }
                },
                Event::ToFocus(e) => {
                    let focus = screen.focus(&components).to_string();
                    if let Some(c) = components.get_mut(&focus) {
                        c.handle(&state, &e, tx.clone());
                    }
                },
                Event::ToMpd(e) => mpd_tx.send(e).unwrap(),
                _ => (),
            }
        }

        write!(stdout, "{}{}{}", clear::All, cursor::Goto(1,1), cursor::Show).unwrap();

        Ok(())
    }

    fn init_components(&self) -> HashMap<String, Components> {
        if let Some(path) = &self.layout_path {
            match layout_config::load(&path) {
                Ok(map) => map,
                _ => HashMap::new(),
            }
        } else {
            HashMap::new()
        }
    }

    fn init_stdin_thread(&self, tx: mpsc::Sender<Event>) {
        thread::spawn(move || {
            let stdin = io::stdin();
            for key in stdin.keys() {
                if let Ok(key) = key {
                    tx.send(
                        Event::ToCommandLine(CommandLineEvent::Input(key))
                    ).unwrap();
                }
            }
        });
    }

    fn run_sbrc(&self, tx: mpsc::Sender<Event>) {
        if let Some(path) = &self.sbrc_path {
            let sbrc = File::open(path).unwrap();
            let reader = BufReader::new(sbrc);

            for (i, line) in reader.lines().enumerate() {
                let line = line.unwrap();
                match command_line::run_headless(&line, tx.clone()) {
                    Ok(_) => (),
                    _ => tx.send(
                        Event::ToCommandLine(
                            CommandLineEvent::SbrcError(i + 1, line.to_string())
                        )
                    ).unwrap(),
                }
            }
        } else {
            tx.send(
                Event::ToCommandLine(CommandLineEvent::SbrcNotFound)
            ).unwrap();
        }
    }

}
fn spawn_draw_screen_event(
    screen: &Screen,
    components: &HashMap<String, Components>,
) -> Event {
    let (w, h) = termion::terminal_size().unwrap();
    let h = h - 1;
    if let Some(_) = components.get(screen.name()) {
        Event::ToComponent(
            screen.to_string(),
            ComponentEvent::Draw(1, 1, w, h, screen.focus(components)),
        )
    } else {
        Event::ToApp(AppEvent::ClearScreen)
    }
}
