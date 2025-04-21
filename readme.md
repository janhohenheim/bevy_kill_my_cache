# Bevy Kill My Cache

[![crates.io](https://img.shields.io/crates/v/bevy_kill_my_cache)](https://crates.io/crates/bevy_kill_my_cache)
[![docs.rs](https://docs.rs/bevy_kill_my_cache/badge.svg)](https://docs.rs/bevy_kill_my_cache)

This plugin adds thousands of components to your app, killing your cache. Why? Because [that makes the scheduler 100 times faster](https://discord.com/channels/691052431525675048/749335865876021248/1363976430085603338). Weird, huh?

## Usage

Just add the plugin, that's it:

```rust
use bevy::prelude::*;
use bevy_kill_my_cache::KillMyCachePlugin;

App::new()
  // rest of your setup code
  .add_plugins(KillMyCachePlugin)
  // that's it!
```
