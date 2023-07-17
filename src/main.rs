use bevy::prelude::*;
mod hello_plugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, hello_plugin::HelloPlugin))
        .run();
}
