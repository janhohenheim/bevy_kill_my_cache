#![warn(missing_docs)]
#![doc = include_str!("../readme.md")]

use bevy::prelude::*;
use paste::paste;
use rand::Rng as _;

/// A plugin that kills your cache, but makes the Bevy scheduler go brrr
pub struct KillMyCachePlugin;

impl Plugin for KillMyCachePlugin {
    fn build(&self, app: &mut App) {
        create_1000_components!(app, KillMyCache);
    }
}

macro_rules! create_component {
    ($app:expr,$a:ident) => {{
        paste! {
            #[derive(Component)]
            #[allow(non_camel_case_types)]
            pub struct $a;

            #[allow(non_snake_case)]
            fn [<add_ $a>](t: Trigger<OnAdd, Transform>, mut commands: Commands) {
                if rand::thread_rng().gen_bool(0.001) {
                    commands.entity(t.target()).insert($a);
                }
            }

            $app.add_observer([<add_ $a>]);
        }
    }};
}
use create_component;

macro_rules! create_5_components {
    ($app:expr, $a:ident) => {
        paste! {
            create_component!($app, [<$a _1>]);
            create_component!($app, [<$a _2>]);
            create_component!($app, [<$a _3>]);
            create_component!($app, [<$a _4>]);
            create_component!($app, [<$a _5>]);
        }
    };
}
use create_5_components;
macro_rules! create_25_components {
    ($app:expr, $a:ident) => {
        paste! {
            create_5_components!($app, [<$a _1>]);
            create_5_components!($app, [<$a _2>]);
            create_5_components!($app, [<$a _3>]);
            create_5_components!($app, [<$a _4>]);
            create_5_components!($app, [<$a _5>]);
        }
    };
}
use create_25_components;
macro_rules! create_100_components {
    ($app:expr, $a:ident) => {
        paste! {
            create_25_components!($app, [<$a _1>]);
            create_25_components!($app, [<$a _2>]);
            create_25_components!($app, [<$a _3>]);
            create_25_components!($app, [<$a _4>]);
        }
    };
}
use create_100_components;

macro_rules! create_500_components {
    ($app:expr, $a:ident) => {
        paste! {
            create_100_components!($app, [<$a _1>]);
            create_100_components!($app, [<$a _2>]);
            create_100_components!($app, [<$a _3>]);
            create_100_components!($app, [<$a _4>]);
            create_100_components!($app, [<$a _5>]);
        }
    };
}
use create_500_components;

macro_rules! create_1000_components {
    ($app:expr, $a:ident) => {
        paste! {
            create_500_components!($app, [<$a _1>]);
            create_500_components!($app, [<$a _2>]);
        }
    };
}
use create_1000_components;
