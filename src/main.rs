extern crate clap_v3 as clap;

use std::io;
use std::path::Path;

use shellbird::Shellbird;

use termion::raw::IntoRawMode;

use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Sean D. <s.digirolamo218@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    genres: Option<String>,
    sbrc: Option<String>,
    layout: Option<String>,
    #[clap(short)]
    debug: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    let sbrc_path = get_sbrc(opts.sbrc);
    let layout_path = get_layout_path(opts.layout);
    let genres_path = get_genre_path(opts.genres);

    let stdout = io::stdout().into_raw_mode().unwrap();

    Shellbird::new(
        genres_path,
        sbrc_path,
        layout_path,
        "127.0.0.1",
        "6600",
        opts.debug,
    ).run(stdout)?;

    Ok(())
}


fn get_sbrc(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/sbrc");
            home
        };

        let homedir = {
            home.push(".sbrc");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/sbrc");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}

fn get_layout_path(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/layout.json");
            home
        };

        let homedir = {
            home.push(".sblayout.json");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/layout.json");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}

fn get_genre_path(path_override: Option<String>) -> Option<String> {
    if let Some(path) = path_override {
        return Some(path)
    }

    if let Some(mut home) = home::home_dir() {
        let free_desktop = {
            let mut home = home.clone();
            home.push(".config/shellbird/genres.txt");
            home
        };

        let homedir = {
            home.push(".sbgenres.txt");
            home
        };

        if free_desktop.as_path().exists() {
            return Some(free_desktop.to_str().unwrap().to_string())
        } else if homedir.as_path().exists() {
            return Some(homedir.to_str().unwrap().to_string())
        }
    }

    let default = Path::new("/etc/shellbird/genres.txt");

    if default.exists() {
        return Some(default.to_str().unwrap().to_string())
    }

    None
}
