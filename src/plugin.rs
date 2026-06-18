
// Imports
use bevy::prelude::*;
use crate::types::*;
use crate::systems::*;

/// Structure that acts as the main plugin for all of the time structures, adding this
/// as a plugin would implement the various timer structure definitions and Bevy systems.
pub struct TimeStructures {}
impl Plugin for TimeStructures {
    fn build(&self, app: &mut App) {

        // Ticker Type Registration + Systems
        app.register_type::<Ticker<i8, f32>>();
        app.register_type::<Ticker<i16, f32>>();
        app.register_type::<Ticker<i32, f32>>();
        app.register_type::<Ticker<i8, f64>>();
        app.register_type::<Ticker<i16, f64>>();
        app.register_type::<Ticker<i32, f64>>();
        app.add_systems(First, ticker_ticking::<i8, f32>);
        app.add_systems(First, ticker_ticking::<i16, f32>);
        app.add_systems(First, ticker_ticking::<i32, f32>);
        app.add_systems(First, ticker_ticking::<i8, f64>);
        app.add_systems(First, ticker_ticking::<i16, f64>);
        app.add_systems(First, ticker_ticking::<i32, f64>);

        // Text
        // app.register_type::<Chronolog>();
        // app.add_systems(First, chronolog_ticking);
    }
}
