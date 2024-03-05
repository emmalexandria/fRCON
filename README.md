# fRCON
*fancy+RCON*

A generic RCON client written in Rust. Implements game specific parsing of responses and highlighting of commands. Currently only implemented for Minecraft.

I cannot overstate how much the work for communicating over RCON in this project was done by [rust-rcon](https://github.com/panicbit/rust-rcon/blob/master/src/packet.rs), whose code for representing, serialising, and deserialising packets was shamelessly stolen by me. 

## Caveats
### Minecraft
Currently, the main caveat is that the fix for [this bug](https://bugs.mojang.com/browse/MC-154617) is not implemented. This means that the stop command won't work on Minecraft versions earlier than 20w16a. This was an intentional decision.

## Project goals
- Parse command responses and display them nicely
- Provide a modern shell experience
- Look pretty 
