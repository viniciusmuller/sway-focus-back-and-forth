# sway-focus-back-and-forth
Implements back-and-forth movement between the current and the previous focused
windows.

It also can be seen as a fix to this [sway's issue](https://github.com/swaywm/sway/issues/3974).

# How It Works
The `server` program connects to the sway's IPC socket and keeps track of the
current and previous focused windows. It also opens an unix socket at
`/run/user/1000/sway_focus_back_and_forth.sock`, which the `client` program
connects to and sends a *"jump back sir!"* request. Then it happens.

# How To Use It
After opening sway, you need to have the `server` program running. Then whenever you want to
jump back, just run the `client` program and it will do the trick.

# Building
Just run `cargo build`, no external dependencies required.
