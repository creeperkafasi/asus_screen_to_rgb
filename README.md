
# Asus Screen To RGB - A program to set the average screen color as your laptop keyboard color

## Requirements
- Rust for building
- [asusctl](https://gitlab.com/asus-linux/asusctl) for the rgb control

## Usage
```
Command Options:

-d / --delay <milliseconds>   : Time between changing colors in milliseconds (default: 50)
-i / --ignore-black <bool>    : Ignore black pixels when calculating average (default: false)
-b / --brightness <number>    : Brightness multiplier (default: 1)
-m / --mode <mode>            : Led mode; static, breathe, pulse (default: static)
```

## TODO
- A new name
- Better error handling
- More control over the asusctl args
- *(Maybe)* Add asusctl as a crate directly so it is not required 
