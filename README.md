# shellbird
Powerful and customizable mpd client with support for genre-subgenre tree

	usage: shellbird

Note: An mpd server must be installed and running for this project to work. This
is not a full blown music player. It is a client for interacting with mpd.
[mpd](https://www.musicpd.org/)

## Building
	cargo build

## Installation
	cargo install --path ./

## Configuration
As of right now, there is no configuration. This project is in very early stages
of development. Eventually, it will have a vim-like configuration file, and a
layout file that defines the interface of the program in json format.

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

## Genre Tree
Genres are read from a file. An example is in the root of the project, genres.txt.
Right now the path to the file is hardcoded, on my machine, but soon it will be configurable


## To Do:
* Commandline
* keybinds
* splitter alignment
* statusline
* RC
* Json Reading

### To Do Components
* SearchResultsMenu
* TextBox
* Button
* Text
