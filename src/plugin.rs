
// Imports
use bevy_app::prelude::*;
use half::f16;
use crate::types::*;
use crate::systems::*;

/// A structure that acts as the main plugin for all the reflections and systems of the mirth_engine_counters package.
///
/// Will enable said reflections and systems depending on which features have been activated
/// using the "cargo --features" command.  By default, reflections and systems are turned on.
pub struct CountersPlugin {}
impl Plugin for CountersPlugin {
    fn build(&self, app: &mut App) {

        // Reflecting Ticker Types
        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i8, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i16, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i32, f32>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i8, f64>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i16, f64>>();

        #[cfg(feature = "ticker_reflect")]
        app.register_type::<Ticker<i32, f64>>();

        // Ticker Systems
        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i8, f16>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i16, f16>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i32, f16>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i8, f32>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i16, f32>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i32, f32>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i8, f64>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i16, f64>);

        #[cfg(feature = "ticker_systems")]
        app.add_systems(First, tick_tickers::<i32, f64>);
    }
}
