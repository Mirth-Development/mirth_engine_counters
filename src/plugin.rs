
// Imports
use bevy::prelude::*;
use crate::types::*;
use crate::systems::*;

/// Structure that acts as the main plugin for all of the time structures, adding this
/// as a plugin would implement the various timer structure definitions and Bevy systems.
pub struct TimeStructures {}
impl Plugin for TimeStructures {
    fn build(&self, app: &mut App) {

        // ############################### TICKER FEATURES ###################################### //
        // Types
        #[cfg(feature = "ticker_type_i8_f32")]
        app.register_type::<Ticker<i8, f32>>();

        #[cfg(feature = "ticker_type_i16_f32")]
        app.register_type::<Ticker<i16, f32>>();

        #[cfg(feature = "ticker_type_i32_f32")]
        app.register_type::<Ticker<i32, f32>>();

        #[cfg(feature = "ticker_type_i8_f64")]
        app.register_type::<Ticker<i8, f64>>();

        #[cfg(feature = "ticker_type_i16_f64")]
        app.register_type::<Ticker<i16, f64>>();

        #[cfg(feature = "ticker_type_i32_f64")]
        app.register_type::<Ticker<i32, f64>>();

        // Systems
        #[cfg(feature = "ticker_systems_i8_f32")]
        app.add_systems(First, ticker_ticking::<i8, f32>);

        #[cfg(feature = "ticker_systems_i16_f32")]
        app.add_systems(First, ticker_ticking::<i16, f32>);

        #[cfg(feature = "ticker_systems_i32_f32")]
        app.add_systems(First, ticker_ticking::<i32, f32>);

        #[cfg(feature = "ticker_systems_i8_f64")]
        app.add_systems(First, ticker_ticking::<i8, f64>);

        #[cfg(feature = "ticker_systems_i16_f64")]
        app.add_systems(First, ticker_ticking::<i16, f64>);

        #[cfg(feature = "ticker_systems_i32_f64")]
        app.add_systems(First, ticker_ticking::<i32, f64>);
        // ###################################################################################### //



        // ############################### CHRONOLOG FEATURES ################################### //
        // Types

        // Systems

        // ###################################################################################### //
        // Text
        // app.register_type::<Chronolog>();
        // app.add_systems(First, chronolog_ticking);
    }
}
