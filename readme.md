# Bevy Kill My Cache

[![crates.io](https://img.shields.io/crates/v/bevy_kill_my_cache)](https://crates.io/crates/bevy_kill_my_cache)
[![docs.rs](https://docs.rs/bevy_kill_my_cache/badge.svg)](https://docs.rs/bevy_kill_my_cache)

Did you know that the Bevy scheduler gets faster the more archetypes you have? Did you know that you can exploit this to get [up to 1000x more performance](https://discord.com/channels/691052431525675048/749335865876021248/1363976430085603338) in a [highly specialized benchmark](https://github.com/bevyengine/bevy/blob/main/examples/stress_tests/many_components.rs)? Do you want to absolutely destroy your cache to get some scheduler speedup in your game?
Oh boy do I have what you need.
bevy_kill_my_cache comes to the rescue! By using the power of science and technology, we create 1000 new components and insert them at random into your entities! Wow! Isn't that amazing? All you have to do is `.add_plugins(KillMyCachePlugin)` and you unlock all the secret performance locked away in your App.

> Please don't use this for real


## Usage

Just add the plugin, that's it:

```rust
use bevy::prelude::*;
use bevy_kill_my_cache::KillMyCachePlugin;

App::new()
  // rest of your setup code
  .add_plugins(KillMyCachePlugin);
  // that's it!
```

## Compatibility

| bevy        | bevy_kill_my_cache |
|-------------|--------------------|
| 0.17        | 0.3                |
| 0.16        | 0.2                |
| 0.15        | 0.1                |
