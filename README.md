# Claimed Respawns Bot

This is a side-project mainly created to get used to little bigger projects than small examples in Rust. It uses excellent [discord-rs](https://github.com/SpaceManiac/discord-rs/) library for connectivity to Discord servers.

## Purpose

This is a bot created for helping with managing spawns in a popular MMORPG game [Tibia](https://secure.tibia.com/mmorpg/free-multiplayer-online-role-playing-game.php). It helps with managing contention in the hunting areas in the game - people can use Discord to 'claim' the respawn, which basically is a way of saying "I'm currently there" so other people won't waste time checking if this particular hunting area is taken or not.

## Installation:

[discord-rs](https://github.com/SpaceManiac/discord-rs/) needs POSIX-compatible environment currently so you may need to set up the one if you are developing on Windows.

Then the project can be easily built using Cargo:

```
cargo build
```

As far as I know the project works in both stable and nightly toolchains - but I mainly worked on nightly.
