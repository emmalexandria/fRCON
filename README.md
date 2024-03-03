# mcrscon
An RCON client for minecraft written in Rust, made for fun and profit. Includes nicely formatted command responses. Does not yet include command history support, but it is planned.

I cannot overstate how much the work for communicating over RCON in this project was done by [rust-rcon](https://github.com/panicbit/rust-rcon/blob/master/src/packet.rs), whose code for representing, serialising, and deserialising packets was shamelessly stolen by me. 

## Caveats
Currently, the main caveat is that the fix for [this bug](https://bugs.mojang.com/browse/MC-154617) is not implemented. This means that the stop command won't work on Minecraft versions earlier than 20w16a. This was an intentional decision.

## Project goals
- Parse multiple reasons for command failure and display them more nicely as errors
- Provide a modern shell experience
- Provide a highly readable command history
- Look pretty 
