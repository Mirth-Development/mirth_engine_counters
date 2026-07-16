
// Imports
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use half::f16;
use mirth_engine_counters::*;
#[cfg(feature = "testing_tools")]
use mirth_engine_testing_tools::*;

fn main() {
    App::new()
        .add_plugins(CountersPlugin{})

        .add_systems(Startup, spawn_custom_count)

        .run();
}

fn spawn_custom_count(mut commands: Commands) {

    let mut count: Count<i8> = Count::new(
        10,
        0,
        100,
        20,
        true,
        true
    );
}
