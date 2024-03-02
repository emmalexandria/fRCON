# mcrscon
A Rust reimplementation of [mcrcon](https://github.com/Tiiffi/mcrcon), made for fun and profit. I cannot overstate how the work for communicating over RCON in this project was done by [rust-rcon](https://github.com/panicbit/rust-rcon/blob/master/src/packet.rs), whose code for representing, serialising, and deserialising packets was shamelessly stolen by me. 

I have never worked on an interactive shell before, and I'm starting to think that my implementation is hugely overcomplicating things. I'd greatly appreciate help on the how to approach the creation of a low complexity but more complicated than dumb shell. 

## What works:
Multiple commands can be sent with a wait parameter with an optional silent mode. There is an interactive shell with full command history, and parsing of the unknown or incomplete command error to make it look nicer. 

## What doesn't:

- **Multi-line shell input** is completely nonfunctional
  - Backspacing causes duplicate prompts
  - Seeking with arrow keys doesn't work past the last line
  - Every new line added progressively moves the prompt up a single line

## Caveats
Currently, the main caveat is that the fix for [this bug](https://bugs.mojang.com/browse/MC-154617) is not implemented. This means that the stop command won't work on Minecraft versions earlier than 20w16a. This was an intentional decision.

## Project goals
- Parse multiple reasons for command failure and display them more nicely as errors
- Provide a highly readable command history
- Look pretty 