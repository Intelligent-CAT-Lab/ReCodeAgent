# TED Editor

An attempt to build a text editor from scratch in c.

[Kilo](https://github.com/antirez/kilo) was used as a reference for handling escape codes, things of that nature.

### TODO:

- [x] Build buffer
- [x] Render buffer
- [x] Load file
- []  Safe exit (ask if buffer dirty)
- [-] Save buffer to file
- [-] Show/update changed status
- [-] Inserts/editing
  - [] Delete line on backspace (linked list of gapbufs might make this easier)
- [-] Additional navigation 
- [] Refactoring
- [] Extras
  - [] Undo/Redo
  - [] Autosave backup (like vim)
  - [] Syntax highlight

### What it looks like so far:
![Alt text](screenshot.png "Ted")
