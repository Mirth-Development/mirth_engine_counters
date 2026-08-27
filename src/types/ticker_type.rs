
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, Mul, Rem, RemAssign, Sub, SubAssign};
use half::f16;
use crate::{Count, CountMarker, CountValue, Operation};

/// Used for implementing the `V` generic to define integer primitives a Ticker can store for its `start_value`, `end_value`, and `current_value`.
///
/// Supports i8, i16, i32 for `start_value`, `current_value`, and `end_value` within Ticker.
///
/// #### Why Add 1 to MIN?
/// The MIN addition is present to help avoid absolute errors on integer ranges.  MIN's
/// assignment on value types will always add 1 to an integer's minimum to avoid things like -128 in
/// the i8 primitive becoming 128 after .absolute() is applied to a value.  We have to do this since
/// 128 is outside the i8 range; 127 is the max for i8.
// pub trait TickerValue:



// ################################## TickerPrecision TRAIT ##################################### //
/// Used for implementing the `P` generic to define float types a Ticker can use for its precision in tracking time, `P` impacts the `time_interval` and `stored_time` fields.
///
/// Supports f16, f32, and f64 for `stored_time` and `time_interval` fields within Ticker.
///
/// #### Why Add Precision?
/// f16, f32, and f64 types for precision determine how accurate the calculations inside the .tick() method are.
/// f16 being the least accurate, f32 being the middle ground, and f64 being the most accurate. Precision
/// control is useful if your aim is to save memory (especially if thousands of tickers are active).
/// - `Recommendation for f16 Usage` : 2 Floating Digits Matter
/// - `Recommendation for f32 Usage` : 5 Floating Digits Matter
/// - `Recommendation for f64 Usage` : 10 Floating Digits Matter
///
/// #### When to Consider More Precision?
/// I'd say the only scenarios where the precision jump becomes important is for big clocks (world clocks)
/// that can impact many entities, or if PvP is involved in a game and the timing of things should be as
/// accurate as possible to reduce frustration.
pub trait TickerPrecision:
Copy                    // TickerPrecision types are floats, which means they're safe to copy.
+ PartialOrd            // TickerPrecision types are floats, hence PartialOrd is necessary for comparisons.
+ Display               // Making it so values can be printed to the console.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Mul<Output = Self>
+ Rem<Output = Self>
+ AddAssign
+ SubAssign
+ RemAssign
+ Send                  // Needed for Bevy queries; also lets Tickers move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Tickers be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that TickerPrecision types own their data, with no borrowed lifetimes.
{
    ///
    const MIN_POSITIVE: Self;

    ///
    const MAX: Self;

    ///
    fn is_nan(self) -> bool;

    /// Text
    fn clamp(self, min: Self, max: Self) -> Self;

    ///
    fn power(self, value: Self) -> Self;

    /// Text
    fn as_f64(self) -> f64;

    /// Text
    fn from_f64(value: f64) -> Self;
}
impl TickerPrecision for f16 {
    const MIN_POSITIVE: Self = Self::MIN_POSITIVE;

    const MAX: Self = Self::MAX;

    fn is_nan(self) -> bool
    { self.is_nan() }

    fn clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn power(self, value: Self) -> Self
    { f16::from_f32(self.to_f32().powf(value.to_f32())) }

    fn as_f64(self) -> f64
    { self.to_f64() }

    fn from_f64(value: f64) -> Self
    { f16::from_f64(value) }
}
impl TickerPrecision for f32 {
    const MIN_POSITIVE: Self = Self::MIN_POSITIVE;

    const MAX: Self = Self::MAX;

    fn is_nan(self) -> bool
    { self.is_nan() }

    fn clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn power(self, value: Self) -> Self
    { self.powf(value) }

    fn as_f64(self) -> f64
    { self as f64 }

    fn from_f64(value: f64) -> Self
    { value as f32 }
}
impl TickerPrecision for f64 {
    const MIN_POSITIVE: Self = Self::MIN_POSITIVE;

    const MAX: Self = Self::MAX;

    fn is_nan(self) -> bool
    { self.is_nan() }

    fn clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn power(self, value: Self) -> Self
    { self.powf(value) }

    fn as_f64(self) -> f64
    { self }

    fn from_f64(value: f64) -> Self
    { value }
}



// ################################### TickerBehavior ENUM ###################################### //
/// Defines the set of possible behaviors a Ticker can be assigned, controlling both whether the
/// ticker is mutable and what happens to current_value once it reaches a boundary.
///
/// - **`Looper`**
///     - The ticker is **immutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`MutLooper`**
///     - The ticker is **mutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`Oneshot`**
///     - The ticker is **immutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`MutOneshot`**
///     - The ticker is **mutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`Freezing`**
///     - The ticker begins **mutable**, but will become **immutable** once current_value hits end_value.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "ticker_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ticker_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum TickerBehavior {
    Looper,
    MutLooper,
    Oneshot,
    MutOneshot,
    Freezing,
}



// ###################################### Ticker STRUCT ######################################### //
/// In short, a ticker is a struct used to track the time between events.
///
/// #### What Is A Ticker?
/// In long, a ticker is a self-contained counter that advances a value (current_value) between two boundaries
/// (start_value and end_value) at a fixed or dynamic rate, driven by calls to the .tick() method.
/// Depending on its behavior, a Ticker can loop back to start_value when
/// it reaches a boundary, stop at the boundary it hits, or become locked in place once it
/// reaches end_value.
///
/// "Time" here is intentionally generic — .tick() doesn't assume seconds, frames, or any
/// specific unit. It just compares whatever delta you feed .tick() against time_interval and
/// advances current_value accordingly, which is what lets the same ticker logic drive a
/// frame-timed cooldown, a world clock, or anything else that changes over some unit of "time".
///
/// A ticker's unit of time will be equal to the unit of the number you pass into its .tick() call(s).
///
///
/// ---
///
/// #### What Are The Fields of a Ticker And What Do They Do?
///
/// - **`start_value`**
///     - Represents the beginning value of a ticker and acts as one of the boundaries for current_value.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`current_value`**
///     - Represents the value a ticker is currently at.
///     - current_value is bound to the range that start_value and end_value create.
///     - Ticking causes current_value to change, even if the ticker is immutable.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///     - **For Looping Tickers**
///         - current_value will be set to start_value when a loop triggers.
///         - current_value hitting end_value will cause a loop to trigger.
///     - **For Oneshot Tickers**
///         - current_value hitting a boundary will assign current_value to be the boundary's value.
///     - **For Freezing Tickers**
///         - current_value hitting end_value will cause the ticker to become immutable.
///
/// - **`end_value`**
///     - Represents the ending value of a ticker and acts as one of the boundaries for current_value.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`time_interval`**
///     - The amount of time that it takes for current_value to change by 1.
///     - Use this field to slow or speed up current_value's change.
///     - Can be manipulated through addition and setter methods if the ticker is mutable.
///
/// - **`stored_time`**
///     - Represents the remainder of time from the last .tick() call.
///     - Used by the .tick() method to keep the timing accurate.
///
/// - **`is_paused`**
///     - Represents whether a ticker is paused or not. A paused ticker prevents .tick() calls from doing anything.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`is_ticking_up`**
///     - Represents the tick direction of a ticker.
///     - Ticking up means that current_value will increase by 1 when the value of time stored in time_interval passes.
///     - Ticking down means that current_value will decrease by 1 when the value of time stored in time_interval passes.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`is_handling_time_spikes`**
///     - **If True**
///         - Will make it so that .tick() calls on a ticker are to add or subtract all built-up integer time since
///         the last .tick() call to current_value; addition/subtraction is dependent on is_ticking_up.
///         Any floating remainder gets put into stored_time for the next .tick() call.
///     - **If False**
///         - Will make it so that .tick() calls on a ticker are to add or subtract 1 to current_value;
///         addition/subtraction is dependent on is_ticking_up.
///     - Can be manipulated through a setter method if the ticker is mutable.
///
/// - **`behavior`**
///     - Dictates the type of behavior a ticker is currently set to.
///     - Can be used to stop a ticker from looping, or to start a ticker to loop.
///
/// ---
///
/// #### What Are the Different Behaviors a Ticker Can Have?
///
/// - **`Looper`**
///     - The ticker is **immutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`MutLooper`**
///     - The ticker is **mutable** and will loop when current_value hits either start_value or end_value.  When a loop triggers, current_value is reset back to start_value.
///
///
/// - **`Oneshot`**
///     - The ticker is **immutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`MutOneshot`**
///     - The ticker is **mutable** and will assign current_value to a boundary's value if current_value were to hit start_value or end_value; start and end values are the boundaries.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
///
/// - **`Freezing`**
///     - The ticker begins **mutable**, but it will become **immutable** once current_value hits end_value.
///     - The ticker's stored_time is set to 0.0 when current_value hits end_value.  This ensures the time state is completely reset once it reaches the end.
///
/// ---
///
/// #### What Exactly is Mutable in Tickers?
/// First off, a ticker should always be declared with the `mut` keyword.  We do this since tickers are purposed to tick, and ticking always has the potential to change `current_value` and `stored_time`.
/// From there, the actual mutability of a ticker is dependent on the mutability of its `behavior`.  Here is some info regarding such a thing:
///
/// - **`Behavior is Mutable`**
///     - Every field besides stored_time can be manipulated directly.
///     - stored_time can be changed indirectly through the .hard_reset() method.
///     - stored_time and current_value will be changed indirectly through ticking.  How and when these fields change is based on the ticker's boolean fields, what behavior the ticker is set to, and when .tick() gets called.
///         - Do not regard current_value and stored_time's change from .tick() as a factor of mutability.  Tickers are purposed to tick, hence the changing of such fields should always be expected unless a ticker is paused.
///
/// - **`Behavior is Immutable`**
///     - No fields can be changed directly.
///     - stored_time and current_value will be changed indirectly through ticking.  How and when these fields change is based on the ticker's boolean fields, what behavior the ticker is set to, and when .tick() gets called.
///         - Do not regard current_value and stored_time's change from .tick() as a factor of mutability.  Tickers are purposed to tick, hence the changing of such fields should always be expected unless a ticker is paused.
///
/// ---
///
/// #### What Are the Different Ticker Datatypes?
/// - **`Ticker<i8, f16>`** : 59+ Bits
/// - **`Ticker<i16, f16>`** : 83+ Bits
/// - **`Ticker<i32, f16>`** : 131+ Bits
/// - **`Ticker<i8, f32>`** : 91+ Bits
/// - **`Ticker<i16, f32>`** : 115+ Bits
/// - **`Ticker<i32, f32>`** : 163+ Bits
/// - **`Ticker<i8, f64>`** : 155+ Bits
/// - **`Ticker<i16, f64>`** : 179+ Bits
/// - **`Ticker<i32, f64>`** : 227+ Bits
///
/// The "+" in the bit count is to recognize that the behavior field holds an enum value, and I have no idea
/// how many bits an enum declaration represents.
///
/// ---
///
/// #### How Do I Make A Ticker Tick?
/// Call the .tick() method on a ticker and pass in the time between 2 events, ticking is *usually* for events
/// that happen both consistently and constantly (such as when frames render); the .tick() method does have the
/// ability to take in **any** delta time.
///
/// Due to the complexity of the .tick() method, it can not be properly summarized here. Go read its
/// documentation if you'd like to know more about the method.
#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "ticker_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "ticker_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct Ticker<V: CountValue, P: TickerPrecision> {
    count:                      Count<V>,
    time_interval:              P,
    stored_time:                P,
    is_paused:                  bool,
    is_ticking_up:              bool,
    is_handling_time_spikes:    bool,
    behavior:                   TickerBehavior,
}
impl<V: CountValue, P: TickerPrecision> Default for Ticker<V, P> {

    /// Creates a MutLooper ticker that has the following properties:
    /// - start_value is set to 0.
    /// - Ticks `current_value` from 0 to the ticker's max integer value (`V::MAX`).
    /// - end_value is set to the ticker's max integer value (`V::MAX`).
    /// - time interval is set to 1.0.
    /// - Begins unpaused.
    /// - Will tick up.
    /// - Will handle time spikes.
    ///
    /// #### What is the Behavior of a MutLooper Ticker?
    /// The ticker is **mutable** and will loop when `current_value` hits either `start_value` or `end_value`.
    /// When a loop triggers, `current_value` is reset back to `start_value`.
    fn default() -> Self {
        let count: Count<V> = Count::default();
        Self {
            count,
            time_interval:              P::from_f64(1.0),
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              true,
            is_handling_time_spikes:    true,
            behavior:                   TickerBehavior::MutLooper,
        }
    }
}
impl<V: CountValue, P: TickerPrecision> Ticker<V, P> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// Used for defining a custom ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let ticker = Ticker::<i32, f32>::new(0, 10, 100, 1.0, false, true, true, TickerBehavior::MutLooper);
    /// assert_eq!(ticker.behavior(), TickerBehavior::MutLooper);
    /// ```
    pub fn new(
        count:                      Count<V>,
        time_interval:              P,
        is_paused:                  bool,
        is_ticking_up:              bool,
        is_handling_time_spikes:    bool,
        behavior:                   TickerBehavior,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_time_interval_is_invalid("Ticker::new", time_interval);

        Self {
            count,
            time_interval,
            stored_time: P::from_f64(0.0),
            is_paused,
            is_ticking_up,
            is_handling_time_spikes,
            behavior,
        }
    }

    /// Creates an unpaused Looper that ticks a `Count` from the supplied `starting_value` to the passed `ending_value`.
    ///
    /// #### What Is the Tick Direction If My Initial starting_value and ending_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let ticker = Ticker::<i32, f32>::new_looper(0, 100, 1.0, true);
    /// assert!(ticker.is_ticking_up());
    /// assert_eq!(ticker.behavior(), TickerBehavior::Looper);
    /// ```
    pub fn new_looper(
        starting_value:             V,
        ending_value:               V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
        is_runtime_mutable:         bool,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_time_interval_is_invalid("Ticker::new_looper", time_interval);

        // Determining bound locations since the ending_value could be below or above starting_value.
        // Lower bound must always be the lesser number, upper bound must always be the greater number.
        let lower_bound_value: V;
        let upper_bound_value: V;
        if starting_value <= ending_value {
            lower_bound_value = starting_value;
            upper_bound_value = ending_value;
        }
        else {
            lower_bound_value = ending_value;
            upper_bound_value = starting_value;
        }

        Self {
            count: Count::new(
                starting_value,
                starting_value,
                lower_bound_value,
                upper_bound_value,
                true,
                true,
            ),
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= ending_value,
            is_handling_time_spikes,
            behavior:                   if is_runtime_mutable { TickerBehavior::MutLooper } else { TickerBehavior::Looper },
        }
    }

    /// Creates an unpaused Oneshot that ticks a `Count` from the supplied `starting_value` to the passed `ending_value`.
    ///
    /// #### What Is the Tick Direction If My Initial starting_value and ending_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let ticker = Ticker::<i32, f32>::new_oneshot(0, 10, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehavior::Oneshot);
    /// ```
    pub fn new_oneshot(
        starting_value:             V,
        ending_value:               V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
        is_runtime_mutable:         bool,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_time_interval_is_invalid("Ticker::new_oneshot", time_interval);

        // Determining bound locations since the ending_value could be below or above starting_value.
        // Lower bound must always be the lesser number, upper bound must always be the greater number.
        let lower_bound_value: V;
        let upper_bound_value: V;
        if starting_value <= ending_value {
            lower_bound_value = starting_value;
            upper_bound_value = ending_value;
        }
        else {
            lower_bound_value = ending_value;
            upper_bound_value = starting_value;
        }

        Self {
            count: Count::new(
                starting_value,
                starting_value,
                lower_bound_value,
                upper_bound_value,
                true,
                true,
            ),
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= ending_value,
            is_handling_time_spikes,
            behavior:                   if is_runtime_mutable { TickerBehavior::MutOneshot } else { TickerBehavior::Oneshot },
        }
    }

    /// Creates an unpaused Freezing that ticks a `Count` from the supplied `start_value` to the `end_value`.
    ///
    /// #### What Is the Tick Direction If My Initial start_value and end_value Are Equal?
    /// Up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let ticker = Ticker::<i32, f32>::new_freezing(0, 100, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehavior::Freezing);
    /// ```
    pub fn new_freezing(
        starting_value:             V,
        ending_value:               V,
        time_interval:              P,
        is_handling_time_spikes:    bool,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_time_interval_is_invalid("Ticker::new_freezing", time_interval);

        // Determining bound locations since the ending_value could be below or above starting_value.
        // Lower bound must always be the lesser number, upper bound must always be the greater number.
        let lower_bound_value: V;
        let upper_bound_value: V;
        if starting_value <= ending_value {
            lower_bound_value = starting_value;
            upper_bound_value = ending_value;
        }
        else {
            lower_bound_value = ending_value;
            upper_bound_value = starting_value;
        }

        Self {
            count: Count::new(
                starting_value,
                starting_value,
                lower_bound_value,
                upper_bound_value,
                true,
                true,
            ),
            time_interval,
            stored_time:                P::from_f64(0.0),
            is_paused:                  false,
            is_ticking_up:              starting_value <= ending_value,
            is_handling_time_spikes,
            behavior:                   TickerBehavior::Freezing,
        }
    }

    /// Creates a copy of the passed ticker.
    pub fn new_copy(
        ticker: Ticker<V, P>,
    ) -> Self {
        Self {
            count: Count::new(
                ticker.count.anchor(),
                ticker.count.value(),
                ticker.count.lower_bound(),
                ticker.count.upper_bound(),
                ticker.count.is_lower_bound_active(),
                ticker.count.is_upper_bound_active(),
            ),
            time_interval:              ticker.time_interval(),
            stored_time:                ticker.stored_time(),
            is_paused:                  ticker.is_paused(),
            is_ticking_up:              ticker.is_ticking_up(),
            is_handling_time_spikes:    ticker.is_handling_time_spikes(),
            behavior:                   ticker.behavior(),
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //
    /// Returns the start_value of a ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(5, 5, 10, 1.0, true, true);
    /// assert_eq!(ticker.start_value(), 5);
    /// ```
    #[inline]
    pub fn count(&self) -> &Count<V> {
        &self.count
    }

    /// Returns the time_interval of a Ticker.
    ///
    /// #### What Exactly is time_interval?
    /// The time_interval is what dictates how long in \[INSERT_TIME_UNIT_HERE\] that it takes for
    /// current_value to increase or decrease by 1; direction depends on `is_ticking_up`.
    ///
    /// #### Unit of time_interval?
    /// ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside the .tick() method. The unit they represent is
    /// determined entirely by whatever unit they pass into the .tick() method.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 2.5, true, true);
    /// assert_eq!(ticker.time_interval(), 2.5);
    /// ```
    #[inline]
    pub fn time_interval(&self) -> P {
        self.time_interval
    }

    /// Returns the stored_time of a Ticker.
    ///
    /// #### When Should I Use This Method?
    /// Realistically speaking, this method has limited use in most cases — stored_time holds
    /// only the leftover remainder from the last call to .tick(), not the total elapsed time
    /// since the ticker was created or last reset.  It exists mainly for debugging, logging, or
    /// custom structures that need to inspect or manually carry over a Ticker's in-progress
    /// timing state.
    ///
    /// #### Unit of stored_time?
    /// ticker has no built-in concept of "seconds" or any other unit — time_interval and stored_time
    /// are just two numbers compared against each other inside the .tick() method. The unit they represent is
    /// determined entirely by whatever unit they pass into the .tick() method.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert_eq!(ticker.stored_time(), 0.0);
    /// ```
    #[inline]
    pub fn stored_time(&self) -> P {
        self.stored_time
    }

    /// Returns true if the ticker is paused, false otherwise.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(!ticker.is_paused());
    /// ticker.pause();
    /// assert!(ticker.is_paused());
    /// ```
    #[inline]
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Returns true if a ticker is set to tick its `current_value` up, false otherwise.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_ticking_up());
    /// ```
    #[inline]
    pub fn is_ticking_up(&self) -> bool {
        self.is_ticking_up
    }

    /// Returns true if the ticker can fire more than once in a single .tick() call, false otherwise.
    ///
    /// - `TRUE`: The ticker can change `current_value` by any number greater than or equal to 0 per
    /// .tick() call; the ticker will catch up all at once.
    ///
    /// - `FALSE`: The ticker can `change current_value` by 1 or 0 per .tick() call, no matter how much
    /// time has built up between .tick() calls.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn is_handling_time_spikes(&self) -> bool {
        self.is_handling_time_spikes
    }

    /// Returns the TickerBehavior type of the ticker.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let ticker = Ticker::<i32, f32>::new_mut_looper(0, 10, 1.0, true);
    /// assert_eq!(ticker.behavior(), TickerBehavior::MutLooper);
    /// ```
    #[inline]
    pub fn behavior(&self) -> TickerBehavior {
        self.behavior
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    /// Changes `start_value` to the passed value.
    ///
    ///
    /// #### What Happens If Setting start_value Pushes current_value Out of Bounds?
    /// If the new `start_value` shifts the valid range such that `current_value` is left outside
    /// the boundaries, `current_value` is automatically clamped to the nearest valid edge.
    ///
    ///
    /// #### Important
    /// `start_value` can NOT go out of the range of `V::MIN` to `V::MAX`.
    /// Attempting to set `start_value` outside the range will cause it to be clamped down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 20, 100, 1.0, true, true);
    /// ticker.set_start_value(40);
    ///
    /// assert_eq!(ticker.start_value(), 40);
    /// assert_eq!(ticker.current_value(), 40); // Clamped from 20 up to the new start_value 40
    /// ```
    #[inline]
    pub fn set_count(&mut self) -> &mut Count<V>{
        if self.is_runtime_mutable() {
            &mut self.count
        }
        else {
            panic_and_print_mutability_message("Count", "set_count()");
        }
    }

    /// Time interval can not be negative or 0--would mess up tick calculation.
    ///
    /// CREATE ERROR FOR SUCH A THING!  CLAMP IS BAD!
    #[inline]
    pub fn set_time_interval(&mut self, value: P) {

        // PANIC EVALUATION
        panic_if_time_interval_is_invalid("set_time_interval()", value);

        if self.is_runtime_mutable() {
            self.time_interval = value.clamp(P::MIN_POSITIVE, P::MAX);
        }
        else {
            panic_and_print_mutability_message("time_interval", "set_time_interval()");
        }
    }

    /// Prevents .tick() calls on a ticker from doing their job.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.pause();
    /// assert!(ticker.is_paused());
    /// ```
    #[inline]
    pub fn pause(&mut self) {
        if self.is_runtime_mutable() {
            self.is_paused = true;
        }
        else {
            panic_and_print_mutability_message("is_paused boolean", "pause()");
        }
    }

    /// Allows .tick() calls on a ticker to do their job.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.pause();
    /// ticker.unpause();
    /// assert!(!ticker.is_paused());
    /// ```
    #[inline]
    pub fn unpause(&mut self) {
        if self.is_runtime_mutable() {
            self.is_paused = false;
        }
        else {
            panic_and_print_mutability_message("is_paused boolean", "unpause()");
        }
    }

    /// Causes the ticker's current_value to count up.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(5, 5, 0, 1.0, false, true);
    /// assert!(ticker.is_ticking_down());
    /// ticker.tick_up();
    /// assert!(ticker.is_ticking_up());
    /// ```
    #[inline]
    pub fn tick_up(&mut self) {
        if self.is_runtime_mutable() {
            self.is_ticking_up = true;
        }
        else {
            panic_and_print_mutability_message("is_ticking_up boolean", "tick_up()");
        }
    }

    /// Causes the ticker's current_value to count down.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_ticking_up());
    /// ticker.tick_down();
    /// assert!(ticker.is_ticking_down());
    /// ```
    #[inline]
    pub fn tick_down(&mut self) {
        if self.is_runtime_mutable() {
            self.is_ticking_up = false;
        }
        else {
            panic_and_print_mutability_message("is_ticking_up boolean", "tick_down()");
        }
    }

    /// Will make it so that .tick() calls on a ticker are to add or subtract all built-up integer time since
    /// the last .tick() call to `current_value`; addition/subtraction is dependent on `is_ticking_up`.
    /// Any floating remainder gets put into stored_time for the next .tick() call.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, false);
    /// assert!(!ticker.is_handling_time_spikes());
    /// ticker.start_handling_time_spikes();
    /// assert!(ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn start_handling_time_spikes(&mut self) {
        if self.is_runtime_mutable() {
            self.is_handling_time_spikes = true;
        }
        else {
            panic_and_print_mutability_message("is_handling_time_spikes boolean", "start_handling_time_spikes()");
        }
    }

    /// Will make it so that .tick() calls on a ticker are to add or subtract 1 to `current_value`;
    /// addition/subtraction is dependent on `is_ticking_up`.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// assert!(ticker.is_handling_time_spikes());
    /// ticker.stop_handling_time_spikes();
    /// assert!(!ticker.is_handling_time_spikes());
    /// ```
    #[inline]
    pub fn stop_handling_time_spikes(&mut self) {
        if self.is_runtime_mutable() {
            self.is_handling_time_spikes = false;
        }
        else {
            panic_and_print_mutability_message("is_handling_time_spikes boolean", "stop_handling_time_spikes()");
        }
    }

    /// Switches the behavior of a ticker to the passed TickerBehavior type.
    ///
    /// #### Does This Work For Tickers That Are Runtime Immutable?
    /// Yes.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::{Ticker, TickerBehavior};
    ///
    /// let mut ticker = Ticker::<i32, f32>::new_mut_looper_custom(0, 0, 10, 1.0, true, true);
    /// ticker.set_behavior(TickerBehavior::Oneshot);
    /// assert_eq!(ticker.behavior(), TickerBehavior::Oneshot);
    #[inline]
    pub fn set_behavior(&mut self, new_behavior: TickerBehavior) {
        self.behavior = new_behavior;
    }
    // ########################################################################################## //



    // ################################ MISCELLANEOUS METHODS ################################### //
    ///
    #[inline]
    pub fn operate_on_time_interval(
        &mut self,
        operation: Operation,
        value: P
    ) {
        // PANIC EVALUATION
        panic_if_is_nan("time_interval", "operating on", value);

        if self.is_runtime_mutable() {
            match operation {
                Operation::Add      => self.time_interval = (self.time_interval + value).clamp(P::MIN_POSITIVE, P::MAX),
                Operation::Subtract => self.time_interval = (self.time_interval - value).clamp(P::MIN_POSITIVE, P::MAX),
                Operation::Multiply => self.time_interval = (self.time_interval * value).clamp(P::MIN_POSITIVE, P::MAX),
                Operation::Power    => self.time_interval = self.time_interval.power(value).clamp(P::MIN_POSITIVE, P::MAX),
                Operation::Divide   => {
                    panic_if_zero("time_interval", "dividing", value);
                    self.time_interval = (self.time_interval / value).clamp(P::MIN_POSITIVE, P::MAX);
                },
            }
        }
        else {
            panic_and_print_mutability_message("time_interval", "operate_on_time_interval()");
        }
    }

    ///
    #[inline]
    pub fn reset(&mut self) {
        if self.is_runtime_mutable() {
            self.count.set_marker_with_clamp(CountMarker::Anchor, self.count.anchor());
            self.stored_time = P::from_f64(0.0);
        }
        else {
            panic_and_print_mutability_message("stored_time and Count's value", "reset()");
        }
    }
    // ########################################################################################## //



    // ############################### THE TICK (MOST IMPORTANT METHOD) ######################### //
    /// #### Description of .tick()
    /// Used to advance a ticker by taking in a passing of time between 2 events, *usually* for events
    /// that happen both consistently and constantly (such as when frames render); it does have the
    /// ability to take in **any** delta time.
    ///
    /// ---
    ///
    /// #### What The Hell Does Ticking Do?
    /// The simplified version (read the method's code for the complex version) of calling the .tick()
    /// method on a ticker is as follows:
    /// 1. Increase stored_time by the value passed to the .tick() call (elapsed_time_between_events).
    /// 2. If stored_time is greater than or equal to the time_interval, change current_value.
    /// 3. Reassign stored_time to the result of `stored_time %= time_interval`.  We do this to carry
    /// over our remainder to keep the state of a Ticker's time accurate to the events that are being
    /// tracked.
    ///
    /// ---
    ///
    /// #### What Impacts Ticking And How?
    /// The fields of a ticker which impact the calculations inside this method are as follows:
    /// - `is_paused`
    ///     - **True** : Prevent the tick method from doing anything.
    ///     - **False** : Tick method will determine if current_value needs to be changed.
    /// - `is_ticking_up`
    ///     - **True** : .tick() calls will increase current_value.
    ///     - **False** : .tick() calls will decrease current_value.
    /// - `is_handling_time_spikes`
    ///     - **True** : The full integer magnitude from the result of `elapsed_time + stored_time` will be used to change current_value.
    ///     - **False** : As long as stored_time is greater than or equal to the time_interval, current_value will change by 1.
    /// - `behavior`
    ///     - **Looper** : Reset current_value to start_value when a ticker boundary is hit; boundaries are start_value and end_value.
    ///     - **MutLooper** :  Reset current_value to start_value when a ticker boundary is hit; boundaries are start_value and end_value.
    ///     - **Oneshot** : current_value will be set to the boundary it hits or goes past.
    ///     - **MutOneshot** : current_value will be set to the boundary it hits or goes past.
    ///     - **Freezing** : current_value will be set to the boundary it hits or goes past.
    ///
    /// ---
    ///
    /// #### Does .tick() Impact a Ticker's Units?
    /// Yes.  The units for a Ticker's integers and float fields are based on what is passed into the
    /// .tick() method.  If the passed value represents the difference in seconds between 2 frames,
    /// the Ticker's time_interval and stored_time units would be seconds.  From there, the start_value,
    /// current_value, and end_value would be based on time_interval's value and unit.  For instance,
    /// if time_interval is set to 37.0 and .tick() took in seconds, then a 1 inside any of the value
    /// fields will be equal to 37 seconds; a 2 in current_value in this example would be 74 seconds.
    ///
    /// ---
    ///
    /// #### Example of .tick() in Action
    /// Consider the following factors first:
    /// - `The Horror You've Undergone` : You've seen 8 clowns between 2 blinks.  You decide to make a ticker to calculate this horror.
    /// - `Ticker's Starting time_interval Value` : 1.7
    /// - `Ticker's Starting stored_time Value` : 0.0
    /// - A .tick() call produced 4 for its magnitude_of_time_that_passed.
    /// - At the end of the .tick() call, stored_time is 0.705882353.
    /// - The ticker you're using to track all of this has is_handling_time_spikes set to true.
    ///
    /// Now, here's a step-by-step scenario involving our known factors:
    /// 1. You've undergone horror.
    /// 2. You throw in the 8 clowns to a .tick() call as it represents the time between your known events (blinks).
    /// 2. Your stored_time started at 0 and now adds up to 8 clowns seen.  Taking in these clowns has turned your Ticker's units to "clowns seen".
    ///     - The clowns seen is our time unit.  An equivalent to this would be seconds as a unit between frames being rendered.
    ///     - A blink is the event that tells us when to log things.  An equivalent to this would be a new frame being rendered.
    /// 3. Your time_interval is set to 1.7 clowns seen, which will mean that we only add 1 to current_value after we've seen 1.7 clowns.
    /// 4. Since is_handling_time_spikes is set to true, we take our 8 clowns and divide that by our time_interval of 1.7.  4.705882353 is the result.
    /// 5. .tick() will truncate 4.705882353 to 4 and the 0.705882353 will act as a remainder.
    /// 6. The remainder of 0.705882353 gets assigned to stored_time so that the next .tick() call can account for how many clowns were carried over from the last blink period.
    /// 7. The 4 is what gets added or subtracted from current_value depending on if is_ticking_up is set to true or false.
    /// 8. Voilà. .tick() call is complete.
    pub fn tick(&mut self, elapsed_time_between_events: P) {

        // POTENTIAL RETURN
        // If paused, go no further as we don't need to calculate the new current_value since the ticker is frozen.
        if self.is_paused {
            return;
        }

        // TIME ACCUMULATION
        // Add to the stored_time so that we can later determine if we've gone over the time_interval value and need to fire another tick.
        self.stored_time += elapsed_time_between_events;

        // DETERMINING PASSED TIME
        // Acquiring the integer magnitude of time that passed between events.
        // How this is done is dependent on what is_handling_time_spikes is set to.
        let magnitude_of_time_that_passed = match self.is_handling_time_spikes {

            // PASSED TIME WHEN HANDLING TIME SPIKES
            // When time spike handling is active, the entire integer value of time that passed since
            // the last .tick() call will be used for magnitude_of_time_that_passed.  The floating
            // remainder after division is kept in stored_time so that partial progress toward the
            // next tick is not lost between the events that are being timed.
            //
            // as_64() in tick_count_truncated_to_value_type is acting as a bridge for the V and P generics
            // to work with one another.  It does mean that a typecast to f64 happens here, but the requested
            // precision is still maintained since the calculated magnitude_of_time_that_passed happened
            // inside the variable "magnitude_of_time_that_passed_in_active_precision".
            true => {
                let magnitude_of_time_that_passed_in_active_precision: P = self.stored_time / self.time_interval;
                let magnitude_of_time_that_passed_in_value_type: V = V::from_f64(magnitude_of_time_that_passed_in_active_precision.as_f64());
                self.stored_time %= self.time_interval; // Carrying remainder over to keep ticking accuracy.
                magnitude_of_time_that_passed_in_value_type
            },

            // PASSED TIME WHEN ~NOT~ HANDLING TIME SPIKES
            // When time spike handling is inactive, only 1 tick is allowed to fire per call
            // regardless of how large the elapsed_time_between_events was.  The value of time_interval
            // is subtracted from stored_time rather than resetting to zero so that the timer remains
            // accurate over continuous .tick() calls — any leftover time beyond the single tick carries
            // into the next .tick() call.
            //
            // We subtract by time_interval (rather than just discarding stored_time) specifically because
            // is_handling_time_spikes can be toggled at runtime.  If this flag is permanently false for
            // a given ticker, the leftover precision wouldn't matter — each call only ever checks "has
            // one time_interval passed, yes or no" regardless of how much excess sits in stored_time.  But
            // since the flag can flip to true later, preserving the leftover ensures that switch correctly
            // picks up every unit of time that was quietly accumulating while spike handling was off,
            // rather than discarding that history the moment is_handling_time_spikes gets re-enabled.
            false => match self.stored_time >= self.time_interval {
                true => {
                    self.stored_time -= self.time_interval;
                    V::from_i64(1)
                },
                false => V::from_i64(0),
            },
        };

        // DETERMINING IF CURRENT_VALUE SHOULD BE CHANGED
        // Will only ever change current_value if the stored_time pushed the magnitude_of_time_that_passed
        // beyond the time_interval value.  This check ensures we aren't needlessly adding to current_value
        // for every .tick() call; for specifically frame logic, this prevents changing current_value every frame.
        //
        // To be perfectly clear, magnitude_of_time_that_passed can only be greater than 0 if the stored_time went past the
        // time_interval value.  Greater than 0 means 1 or higher in this case, decimals in between 0 and 1
        // don't count.
        if magnitude_of_time_that_passed > V::from_i64(0) {

            // VALUE ADDITION OR SUBTRACTION?
            // Increase or decrease current_value based on if the ticker is ticking up or down.
            match self.is_ticking_up {
                true  => self.count.operate_with_clamp(Operation::Add, CountMarker::Value, magnitude_of_time_that_passed),
                false => self.count.operate_with_clamp(Operation::Subtract, CountMarker::Value, magnitude_of_time_that_passed),
            }

            // DETERMINE IF AN ACTIVE BOUNDARY WAS HIT
            if self.count.is_at_lower_limit(CountMarker::Value) ||
               self.count.is_at_upper_limit(CountMarker::Value) {

                // RESET DETERMINATION
                // Will reset stored_time or the Count's value based on a ticker's behavior.
                match self.behavior {

                    // LOOPER LOGIC
                    // Reset value to the anchor if either of the count's boundaries - lower_bound and upper_bound - are hit.
                    TickerBehavior::Looper |
                    TickerBehavior::MutLooper => {
                        self.count.set_marker_with_clamp(CountMarker::Value, self.count.anchor());
                    },

                    // ONESHOT + FREEZING LOGIC
                    // stored_time will be zeroed out if value hits an active bound.  We
                    // do this wipe for stored_time since oneshotters and freezings are purposed to clear their
                    // time storage upon hitting an active boundary.
                    TickerBehavior::Oneshot |
                    TickerBehavior::MutOneshot |
                    TickerBehavior::Freezing => {
                        self.stored_time = P::from_f64(0.0);
                    },
                };
            }
        }
    }
    // ############################################################################################## //



    // ###################################### HELPER METHODS ######################################## //
    /// Returns true if the current behavior of the ticker is runtime mutable, otherwise false.  Best used
    /// when mass querying tickers that could be either mutable or immutable.  Can be used to avoid
    /// panics that would occur when attempting to mutate an immutable behavior.
    ///
    /// #### Example
    /// ```
    /// use mirth_engine_counters::Ticker;
    ///
    /// let looper = Ticker::<i32, f32>::new_looper(0, 10, 1.0, true);
    /// assert!(!looper.is_runtime_mutable());
    ///
    /// let mut_looper = Ticker::<i32, f32>::new_mut_looper(0, 10, 1.0, true);
    /// assert!(mut_looper.is_runtime_mutable());
    /// ```
    #[inline]
    pub fn is_runtime_mutable(&self) -> bool {
        match self.behavior {
            TickerBehavior::Looper     => false,
            TickerBehavior::MutLooper  => true,
            TickerBehavior::Oneshot    => false,
            TickerBehavior::MutOneshot => true,
            TickerBehavior::Freezing   => !self.count.is_at_a_limit(CountMarker::Value),
        }
    }

    /// Will print out all the fields and their values of a ticker.
    pub fn print_information(&self) {
        self.count.print_information();
        println!("TIME_INTERVAL: {}", self.time_interval);
        println!("STORED_TIME: {}", self.stored_time);
        println!("IS_PAUSED: {}", self.is_paused);
        println!("IS_TICKING_UP: {}", self.is_ticking_up);
        println!("IS_HANDLING_TIME_SPIKES: {}", self.is_handling_time_spikes);
        println!("BEHAVIOR: {:?}", self.behavior);
    }
    // ############################################################################################## //
}



// ##################################### PANIC FUNCTIONS ######################################## //

#[inline]
fn panic_if_zero<P: TickerPrecision>(name_of_value: &str, name_of_action: &str, value: P) {
    if value == P::from_f64(0.0) {
        panic!(
            "{}[TICKER PANIC]{} You are {name_of_action} a ticker's {name_of_value} with 0.  This will produce NaN.
            NaN is not a valid CountValue for any comparison, bound, or arithmetic operation.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
#[inline]
fn panic_if_is_nan<P: TickerPrecision>(name_of_value: &str, name_of_action: &str, value: P) {
    if value.is_nan() {
        panic!(
            "{}[TICKER PANIC]{} You are {name_of_action} a ticker's {name_of_value} with NaN.
            NaN is not a valid TickerPrecision for any comparison, bound, or arithmetic operation.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
#[inline]
fn panic_if_time_interval_is_invalid<P: TickerPrecision>(name_of_action: &str, value: P) {
    if value.is_nan() {
        panic!(
            "{}[TICKER PANIC]{} You are making a ticker's time_interval NaN with {name_of_action}.
            NaN is not a valid TickerPrecision for any comparison, bound, or arithmetic operation.",
            "\x1b[31m", "\x1b[0m",
        );
    }
    else if value <= P::from_f64(0.0) {
        panic!(
            "{}[TICKER PANIC]{} You are making a ticker's time_interval be less than or equal to 0.0 with {name_of_action}.
            A value that is less than or equal to 0.0 for time_interval is NOT acceptable since it messes up the .tick() method.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

/// Used to cause a `PANIC` when something attempts to mutate an immutable ticker.
fn panic_and_print_mutability_message(name_of_value: &str, name_of_action: &str,) -> ! {
    panic!(
        "{}[TICKER PANIC]{} You are attempting to mutate the {name_of_value} through {name_of_action} in a runtime immutable ticker.",
        "\x1b[31m", "\x1b[0m",
    )
}
