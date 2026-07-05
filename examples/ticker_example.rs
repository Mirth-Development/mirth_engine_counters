
// Imports
use bevy_app::prelude::*;
use bevy_app::ScheduleRunnerPlugin;
use bevy_ecs::prelude::*;
use bevy_time::TimePlugin;
use half::f16;
use std::time::Duration;
use mirth_engine_tickers::*;
#[cfg(feature = "testing_tools")]
use mirth_engine_testing_tools::*;

fn main() {
    App::new()

        // 1.0 / 60.0 == Loop 60 Times Per 1 Second
        // Change the calculation to w/e you want for testing.
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0 / 60.0)))
        .add_plugins(TimePlugin)
        .add_plugins(TickersPlugin{})

        // Comment this add_system line to prevent the master ticker from spawning.
        .add_systems(Startup, spawn_custom_ticker)

        // Don't touch this unless you want the tickers to NOT print their information.
        .add_systems(Update, print_ticker_information)
        .run();
}

fn spawn_custom_ticker(mut commands: Commands) {

    // Change the fields to whatever you want to test any kind of ticker.
    // Remember to declare mutability if you want to make use of methods that change the ticker's
    // fields, or you can make use of the copy constructors.
    let ticker: Ticker<i8, f16> = Ticker::new(
        0,
        5,
        100,
        1.0,
        false,
        true,
        true,
        TickerBehavior::Looper,
    );

    commands.spawn(ticker);
}

fn print_ticker_information(
    tickers_i8_f16:   Query<&Ticker<i8, f16>>,
    tickers_i16_f16:  Query<&Ticker<i16, f16>>,
    tickers_i32_f16:  Query<&Ticker<i32, f16>>,

    tickers_i8_f32:   Query<&Ticker<i8, f32>>,
    tickers_i16_f32:  Query<&Ticker<i16, f32>>,
    tickers_i32_f32:  Query<&Ticker<i32, f32>>,

    tickers_i8_f64:   Query<&Ticker<i8, f64>>,
    tickers_i16_f64:  Query<&Ticker<i16, f64>>,
    tickers_i32_f64:  Query<&Ticker<i32, f64>>,
) {
    macro_rules! print_queries {
        ($($query:expr),* $(,)?) => {
            $(
                for ticker in &$query {
                    ticker.print_information();
                    println!();
                }
            )*
        };
    }

    print_queries!(
        tickers_i8_f16,
        tickers_i16_f16,
        tickers_i32_f16,

        tickers_i8_f32,
        tickers_i16_f32,
        tickers_i32_f32,

        tickers_i8_f64,
        tickers_i16_f64,
        tickers_i32_f64,
    );
}
