# fRCON
*fancy+RCON*

A generic RCON client written in Rust. Uses the power of [Reedline](https://github.com/nushell/reedline) to offer a more complete shell experience when connected to your server. 

![Basic list command](https://vhs.charm.sh/vhs-3kzKyotX3ucq2NWyKayHfU.gif)
## Features
- Basic first word command highlighting
- Formatting of various command responses and common errors
- First word autocompletion

These features are currently (only partially) implemented for Minecraft. There are no technical blocks (to my knowledge) to implementing these features for more games, or to complete the Minecraft implementation. It is a slow process however. It requires setting up a game server, and then slowly trial and erroring your way through the command list, adding special code to format responses as needed. Once the Minecraft implementation is complete, I plan to start work on a generic implementation for Source games. 

## More GIFS
![Print basic error response](https://vhs.charm.sh/vhs-2ZI97q9oDb78UYZ9sNo7T8.gif)
![Basic autocompletion](https://vhs.charm.sh/vhs-7ac8eXDWfeDDkQIfoGN66W.gif)

## Project goals
- Parse command responses and display them nicely
- Provide a modern shell experience
- Look pretty 


## Caveats
### Minecraft
Currently, the main caveat is that the fix for [this bug](https://bugs.mojang.com/browse/MC-154617) is not implemented. This means that the stop command won't work on Minecraft versions earlier than 20w16a. This was an intentional decision.

# Credit
I cannot overstate how much the work for communicating over RCON in this project was done by [rust-rcon](https://github.com/panicbit/rust-rcon/blob/master/src/packet.rs), whose code for representing, serialising, and deserialising packets was shamelessly stolen by me. 
