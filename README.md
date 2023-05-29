# Hyperland-Mouse-Capture-Workaround
This program forces the mouse to stay inside specified windows in hyprland.
## How to use
```
hyprmousecapture [list of program class names]
```
e.g.
```
hyprmousecapture "league of legends"
```
To get the class name of a window you can use `hyprctl clients` to get a list of all windows
## Download
https://github.com/fobekyo/Hyperland-Mouse-Capture-Workaround/releases/download/0.1.0/hyprmousecapture
## Compile Instructions
Have a rust toolchain installed.
run `cargo build --release`
