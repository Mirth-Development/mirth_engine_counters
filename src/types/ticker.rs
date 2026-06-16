
// Imports
use bevy::prelude::*;
use mirth_engine_testing_tools::check_if_value_is_within_range;

/// By themselves, tickers can be used to create simple timers.  Although they are best used in conjunction
/// as an inner element to a greater time structure to create some wicked tickety-tocking.
///
/// All fields of Ticker have getters, and only digit has no setter.
///
/// # TICKING LOOPS AT [`LOOP_POINT`]
/// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value using to_zero().
/// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
/// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
/// **If you're okay with values from [`i8::MIN`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
/// Otherwise, I recommend the Chronolog structure.
#[derive(Component, Reflect, Debug)]
pub struct Ticker {
    start_value:                i8,
    current_value:              i8,
    end_value:                  i8,
    interval:                   f32,
    accrued_delta:              f32,
    is_paused:                  bool,
    is_looping:                 bool,
    is_ticking_up:              bool,
    is_handling_frame_spikes:   bool,
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            start_value:                0,
            current_value:              0,
            end_value:                  100,
            interval:                   1.0,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up:              true,
            is_handling_frame_spikes:   true,
        }
    }
}

impl Ticker {

    ///
    pub fn new(
        start_value: i8,
        current_value: i8,
        end_value: i8,
        interval: f32,
        is_paused: bool,
        is_looping: bool,
        is_ticking_up: bool,
        is_handling_frame_spikes: bool,
    ) -> Self {
        Self {
            start_value,
            current_value,
            end_value,
            interval,
            accrued_delta: 0.0,
            is_paused,
            is_looping,
            is_ticking_up,
            is_handling_frame_spikes,
        }
    }

    ///
    pub fn new_onetime_with_frame_spike_handling(
        starting_value: i8,
        end_value: i8,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_onetime_without_frame_spike_handling(
        starting_value: i8,
        end_value: i8,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 false,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }

    ///
    pub fn new_looper_with_frame_spike_handling(
        starting_value: i8,
        end_value: i8,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   true,
        }
    }

    ///
    pub fn new_looper_without_frame_spike_handling(
        starting_value: i8,
        end_value: i8,
        interval: f32,
        is_ticking_up: bool,
    ) -> Self {
        Self {
            start_value:                starting_value,
            current_value:              starting_value,
            end_value,
            interval,
            accrued_delta:              0.0,
            is_paused:                  false,
            is_looping:                 true,
            is_ticking_up,
            is_handling_frame_spikes:   false,
        }
    }

    /// Returns the start_value of a Ticker.
    ///
    /// start_value can change through other methods, so don't treat it as a consistent value.
    pub fn get_start_value(&self) -> i8 {
        self.start_value
    }

    /// Returns the current_value of a Ticker.
    pub fn get_current_value(&self) -> i8 {
        self.current_value
    }

    /// Returns the end_value of a Ticker.
    ///
    /// end_value can change through other methods, so don't treat it as a consistent value.
    pub fn get_end_value(&self) -> i8 {
        self.start_value
    }

    /// Returns the interval of a Ticker.
    ///
    /// The interval value can change through other methods, so don't treat it as a consistent value.
    /// Also, it's important to remember that the interval is what dictates how long in seconds that it takes
    /// for current_value to increase or decrease; direction depends on is_ticking_up.
    pub fn get_interval(&self) -> f32 {
        self.interval
    }

    /// Returns the difference between current_value and start_value.
    ///
    /// Will only return positive numbers.
    pub fn get_difference_from_start(&self) -> i16 {
        let min: i16 = self.current_value.min(self.start_value) as i16;
        let max: i16 = self.current_value.max(self.start_value) as i16;
        max - min
    }

    /// Returns the difference between current_value and end_value.
    ///
    /// Will only return positive numbers.
    pub fn get_difference_from_end(&self) -> i16 {
        let min: i16 = self.current_value.min(self.end_value) as i16;
        let max: i16 = self.current_value.max(self.end_value) as i16;
        max - min
    }

    /// Returns the difference between start_value and end_value.
    ///
    /// Will only return positive numbers.
    pub fn get_difference_from_start_to_end(&self) -> i16 {
        let min: i16 = self.start_value.min(self.end_value) as i16;
        let max: i16 = self.start_value.max(self.end_value) as i16;
        max - min
    }

    /// Returns the digit in the ones-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// #### Small Note
    /// It is not possible for the ones digit to be dropped, hence the reason why this method has no
    /// "with_drop_accounting" version.  There is always a ones-place.
    pub fn get_ones_digit(&self) -> i8 {
        self.current_value.abs() % 10
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// Use the "get_tens_digit_with_drop_accounting" version of this method to differentiate between
    /// digit drops and the 0 digit.
    pub fn get_tens_digit(&self) -> i8 {
        (self.current_value.abs() / 10) % 10
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value.
    ///
    /// Will return a 0 if the digit doesn't exist.
    ///
    /// Use the "get_hundreds_digit_with_drop_accounting" version of this method to differentiate
    /// between digit drops and the 0 digit.
    pub fn get_hundreds_digit(&self) -> i8 {
        (self.current_value.abs() / 100) % 10
    }

    /// Returns the digit in the tens-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// If current_value is 6, then -1 would be returned.  The flipside would be if current_value is
    /// 103, then 0 would be returned.  The potential for -1 to be returned allows for differentiation
    /// between a digit dropping and if a digit is simply 0 at a given time.
    pub fn get_tens_digit_with_dda(&self) -> i8 {

        if self.current_value.abs() < 10 {
            (self.current_value.abs() / 10) % 10
        }
        else {
            -1
        }
    }

    /// Returns the digit in the hundreds-place of current_value.
    ///
    /// Will always return a positive value if the digit exists.
    ///
    /// Will return -1 if the digit is NOT being used.
    ///
    /// ### What is DDA?
    /// DDA stands for Digit Drop Accounting.  It can be used to determine if a digit has been dropped
    /// from a number or if a digit just happens to be 0.
    ///
    /// ### Example
    /// If current_value is 17, then -1 would be returned.  The flipside would be if current_value is
    /// 1032 (not realistic for i8, just an example), then 0 would be returned.  The potential for -1
    /// to be returned allows for differentiation between a digit dropping and if a digit is simply 0 at
    /// a given time.
    pub fn get_hundreds_digit_with_dda(&self) -> i8 {

        if self.current_value.abs() < 100 {
            (self.current_value.abs() / 100) % 10
        }
        else {
            -1
        }
    }

    /// Returns true if the current_value and the start_value are equal to one another, false otherwise.
    pub fn is_equal_to_start_value(&self) -> bool {
        self.current_value == self.start_value
    }

    /// Returns true if the current_value and the end_value are equal to one another, false otherwise.
    pub fn is_equal_to_end_value(&self) -> bool {
        self.current_value == self.end_value
    }

    /// Pauses a ticker's ticking.
    ///
    /// This prevents the .tick method from doing any calculations.
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Unpauses a ticker's ticking.
    ///
    /// This allows the .tick method to resume its calculations.
    pub fn unpause(&mut self) {
        self.is_paused = false;
    }

    /// Causes the ticker's current_value to count up.
    ///
    /// Will allow calculated ticks inside the .tick method to add to current_value, rather than subtract.
    pub fn tick_up(&mut self) {
        self.is_ticking_up = true;
    }

    /// Causes the ticker's current_value to count down.
    ///
    /// Will allow calculated ticks inside the .tick method to subtract from current_value, rather than add.
    pub fn tick_down(&mut self) {
        self.is_ticking_up = false;
    }

    /// Will set the current_value to be equal to the start_value and the digit field of the Ticker
    /// will be changed according to the new ones-place value that is seen after current_value's reset.
    ///
    /// Digit is always to reflect current_value's ones-place.
    pub fn reset(&mut self) {
        self.current_value = self.start_value;
    }

    /// Adds to the start_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    pub fn add_to_start_value(&mut self, value: i8) {
        self.start_value = self.start_value.saturating_add(value);
    }

    /// Adds to the current_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within start_value to end_value (inclusive).
    pub fn add_to_current_value(&mut self, value: i8) {
        let min = self.start_value.min(self.end_value);
        let max = self.start_value.max(self.end_value);
        self.current_value = self.current_value.saturating_add(value).clamp(min, max);
    }

    /// Adds to the end_value of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// Will not let the result of summing cause overflow or wrapping; results will always be within [`i8::MIN`] to [`i8::MAX`] (inclusive).
    pub fn add_to_end_value(&mut self, value: i8) {
        self.end_value = self.end_value.saturating_add(value);
    }

    /// Adds to the interval of the ticker by the passed value.  Can take in negatives for subtraction.
    ///
    /// # IMPORTANT
    /// Interval can never be 0 or a negative number (or go past f32::MAX), the reasoning for this is that it would cause the
    /// .tick method to create crazy values. If your goal is to slow time or an accumulation to the point that it reverses it,
    /// I suggest you flip the tick direction using .tick_up or .tick_down depending on which direction you want the counting to flip to.
    pub fn add_to_interval(&mut self, value: f32) {
        self.interval = (self.interval + value).clamp(f32::MIN_POSITIVE, f32::MAX);
    }

    /// Used to advance a ticker.  Takes in a time.delta() call off the time resource (Res<Time>) that Bevy provides.
    ///
    /// If you're making a custom ticking system and have stripped out the ticking systems provided
    /// in the systems of this plugin, then please note that you must run this each frame for time to move normally.
    ///
    /// # TICKING LOOPS AT [`LOOP_POINT`]
    /// Tickers don't stop ticking.  Once the next tick addition hits [`LOOP_POINT`] it will zero out current_value.
    /// **This is crucial to understand.** Not recognizing that ticking loops on these structures will make for poor usage of them.
    /// Tickers are a building block element to making larger time structures or for highly compartmentalized timer usage.
    /// **If you're okay with values from [`TICKER_MIN_VALUE`] to [`TICKER_MAX_VALUE`] for your timers, then feel free to go ham with Tickers.**
    /// Otherwise, I recommend the Chronolog structure.
    pub fn tick(&mut self, delta: f32) {

        // PAUSE STATUS
        // If paused, go no further as we don't need to calculate the new current_value since the Ticker is frozen.
        if self.is_paused {
            return;
        }

        // DELTA ACCUMULATION
        // Add to the accrued delta so that we can later determine if we've gone over the interval value and need to fire another tick.
        self.accrued_delta += delta;

        // TICK COLLECTION (TC)
        // Acquiring the amount of tick fires that occurred within the given frame based on if
        // the Ticker is set to handle frame spikes.
        let ticks = match self.is_handling_frame_spikes {

            // TC FOR HANDLING FRAME SPIKES
            // When frame spike handling is active, all ticks that accumulated during a large
            // delta are collected at once.  The remainder after division is kept in accrued_delta
            // so that partial progress toward the next tick is not lost between frames.
            true => {
                let tocks = (self.accrued_delta / self.interval) as i8;
                self.accrued_delta %= self.interval;    // Carrying remainder over to keep ticking accuracy.
                tocks
            },

            // TC FOR ~NOT~ HANDLING FRAME SPIKES
            // When frame spike handling is inactive, only one tick is allowed to fire per call
            // regardless of how large the delta was.  One interval is subtracted from accrued_delta
            // rather than resetting to zero so that the timer remains accurate over time — any
            // leftover time beyond the single tick carries into the next frame naturally.
            false => match self.accrued_delta >= self.interval {
                true => {
                    self.accrued_delta -= self.interval;
                    1
                },
                false => 0,
            },
        };

        // TICK FIRE TO CHANGE CURRENT_VALUE
        // Will only ever tick fire if the accrued delta pushed ticks beyond the interval value.
        // This check ensures we aren't needlessly firing for every frame, rather we are firing
        // based on if we've passed over the interval threshold using our constant accrual.
        if ticks > 0 {

            // TICK FIRE DIRECTION
            // Increase or decrease current_value's new host based on if the Ticker is counting up or down.
            let new_value = match self.is_ticking_up {
                true  => self.current_value.saturating_add(ticks),
                false => self.current_value.saturating_sub(ticks),
            };

            // DETERMINING CURRENT_VALUE'S BOUNDARIES
            // Since start_value and end_value can be either negative or positive at any given moment,
            // we must throw both values against one another to determine whose greater/lesser than
            // the other so that we can properly clamp down current_value to its allowed range.
            let min = self.start_value.min(self.end_value);
            let max = self.start_value.max(self.end_value);

            // RESET DETERMINATION + CURRENT_VALUE ASSIGNMENT
            // Will change current_value's assignment using new_value based on if the Ticker is set to loop or not.
            match self.is_looping {

                // LOOPING IS ACTIVE
                // Assign current_value to its new host and then reset it to the Ticker's start_value
                // if either of its boundaries -- start_value and end_value -- are hit.
                true => {
                    self.current_value = new_value;
                    if self.current_value <= min || self.current_value >= max {
                        self.current_value = self.start_value;
                    }
                },

                // LOOPING IS INACTIVE
                // current_value can assume its new host after new_value has been clamped to the allowed range.
                false => {
                    self.current_value = new_value.clamp(min, max);
                },
            };
        }
    }
}
