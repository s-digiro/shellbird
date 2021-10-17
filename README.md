# shellbird
Powerful and customizable mpd client with support for genre-subgenre tree

	usage: shellbird <path/to/genres.txt>

Note: An mpd server must be installed and running for this project to work. This
is not a full blown music player. It is a client for interacting with mpd.
[mpd](https://www.musicpd.org/)

## Building
	cargo build

## Installation
	cargo install --path ./

## Configuration

### Sbrc
Configuration is done in sbrc. sbrc location can be specified as a commandline
argument. Otherwise, it will be sourced from ~/.config/shellbird/sbrc, ~/.sbrc,
or /etc/shellbird/sbrc, in that order. If sbrc is not found, then it simply will not be
loaded. If this happens, a message will be displayed in the statusline alerting
the user.

Sbrc is meant to contain a set of commands that will be run automatically at
startup. The most obvious use  for this is  to execute a series of bind
commands. This allows the user to set keybinds. Another possible use is to set
the startup screen using switchscreen. Be creative!

An example is in the root directory, called sbrc.

### layout
Shellbirds layout is defined in layout.json. Similarly to sbrc, it can be
specified as a commandline argument, otherwise it is sourced from
~/.config/shellbird/layout.json, ~/.sblayout.json, or /etc/shellbird/layout.json
in that order.

An example can be found in the root directory of this project called layout.json

### Genre Tree
Genres are read from a file. This too can be specified as a commandline
argument, otherwise it is sourced from ~/.config/shellbird/genres.txt,
~/.sbgenres.txt or /etc/shellbird/genres.txt, in that order.

An example can be found in the root directory of this project called genres.txt

## Usage
* Now Playing Screen: 1
* Queue Screen: 2
* Playlists Screen: 3
* Library Screen: 4
* Genre Tree Screen: 5
* Menu Next: j
* Menu Prev: k
* Go to top of menu: gg
* Go to bottom of menu: G
* Search: /
* Focus Next: l
* Focus Prev: h
* Toggle Pause: p
* Clear Queue: c
* Select Highlighted: Space
	- What this actually does depends on the context.
	- In a queue menu, it will play from the queue
	- In most other menus, such as tag menus, it will add all tracks under that option to the queue
	- Enter command: : (Currently the only command is :pause)

Note: These are all temporary controls. Eventually controls will be fully configurable in a vim-like rc file

## Commands
A commandline can be brought up with the ':' key. Some available commands are:
* `echo <message>`: Prints /<message/> in status bar
* `q`: Quit application
* `switchscreen <number>`: Switch screens to the one indexed by \<number\>
	* Note`: In the future, screens will probably be named rather than numbered
* `focusnext`: switch focus to the next component on the current screen
* `focusprev`: switch focus to the previous component on the current screen
* `next`: send a next event to the focused component. On a menu this will advance to the next item in the menu
* `prev`: send a prev event to the focused component. On a menu this will go back to the previous item in the menu
* `select`: send a select even to the focused component. On a menu, this usually means adding selected items to the queue. On a queue menu, this means play from the currently selected item.
* `top`: Go to top of focused menu
* `bot`: Go to bottom of focused menu
* `search <term>`: search for given \<term\> and go to it in focused menu. Always case insensitive.
* `goto <number>`: Go to line number \<number\> in focused menu
* `pause`: toggles music pause/playing state
* `clear`: clears playback queue
* `bind <key sequence> <command>`: binds \<key sequence\> to send off \<command\>
	* Example: `bind ss goto 3` will cause inputing 'ss' in normal mode to go to the 4th item in a menu

## To Do:
- [x] Rework Event Enum
- [x] Commandline
- [x] keybinds
- [x] CommandLineEvent
- [x] Track Menu with Style Menu parent
- [x] Pass StyleTree to handlers
- [x] All entry in certain menus
- [x] Tag Menu with StyleMenu parent
- [x] All option on tagmenu
- [ ] component alignment
- [ ] Track display formatting in menus
- [ ] statusline
- [x] RC
- [x] Json Reading
- [ ] Restore cursor after application exits. Termion seems to be bugged and isn't doing it right.
- [x] Screen Map
- [ ] Redraw Event

### To Do Components
- [ ] SearchResultsMenu
- [ ] TextBox
- [ ] Button
- [ ] Text
- [ ] Seekbar
