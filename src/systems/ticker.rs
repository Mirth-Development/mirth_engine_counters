
use bevy::prelude::*;
use crate::ticker::{TickerPrecision, TickerValue};
use crate::types::Ticker;

/// Will loop through queried tickers to initiate their ticking.
pub fn ticker_ticking<V: TickerValue, P: TickerPrecision>(
    time: Res<Time>,
    mut tickers: Query<&mut Ticker<V, P>>,
) {

    // P::from_f64 will either keep time.delta_secs_f64 as an f64 value or convert it to f32
    // depending on the type of ticker_ticking system being used.
    let delta_in_seconds: P = P::from_f64(time.delta_secs_f64());

    for mut ticker in tickers.iter_mut() {
        ticker.tick(delta_in_seconds);
    }
}
