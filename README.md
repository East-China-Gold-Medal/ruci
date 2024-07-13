# Rust Unified Configuration Interface (RUCI) 
## Introduction
A **(WIP)** project that hosts an OpenWRT UCI configuration interface on Rust.  
## Components
+ libUCI binding (bindgen)  
+ libc getlogin() binding (MUSL libc)
+ RESTful interface
+ *TBD*
## Build Notes
+ Use OpenWRT SDK for OpenWRT-specific toolchain
+ OpenWRT SDK **WILL** discard symbol table from dynamic library. You must build *libuci* from OpenWRT SDK **BY YOUR OWN**. 
## Credits
+ [UCI from OpenWRT](https://git.openwrt.org/?p=project/uci.git)
+ [Icons from SVG Repo](https://www.svgrepo.com)
+ [Default background image: Pastel Hills by Lionel](https://store.kde.org/p/1031414)
