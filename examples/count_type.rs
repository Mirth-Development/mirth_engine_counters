
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

        .add_systems(Startup, spawn_custom_ticker)

        .add_systems(Update, print_count_information)

        .run();
}

fn spawn_custom_count(mut commands: Commands) {

    let mut count: Count<i8> = Count::new(
        0,
        5,
        100,
        true,
        true
    );

    commands.spawn(count);
}

fn print_count_information(
    counts_i8:   Query<&Count<i8>>,
    counts_i16:  Query<&Count<i16>>,
    counts_i32:  Query<&Count<i32>>,
    counts_f16:  Query<&Count<f16>>,
    counts_f32:  Query<&Count<f32>>,
    counts_f64:  Query<&Count<f64>>,
) {
    macro_rules! print_queries {
        ($($query:expr),* $(,)?) => {
            $(
                for count in &$query {
                    count.print_information();
                    println!();
                }
            )*
        };
    }

    print_queries!(
        counts_i8,
        counts_i16,
        counts_i32,
        counts_f16,
        counts_f32,
        counts_f64,
    );
}
