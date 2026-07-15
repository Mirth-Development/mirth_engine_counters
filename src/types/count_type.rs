
// Imports
use bevy_ecs::prelude::*;
use std::fmt::Display;
use std::ops::{Add, Div, Rem, Sub};

///
pub trait CountValue:
Copy                    // CountValue types are integers, which means they're safe to copy.
+ Ord                   // CountValue types are integers, hence Ord is necessary for comparison.
+ Display               // Making it so values can be printed to the console.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ Send                  // Needed for Bevy queries; also lets Counts move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Counts be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that CountValue types own their data, with no borrowed lifetimes.
{
    const MIN: Self;
    const MAX: Self;
    fn absolute(self)               -> Self;
    fn as_f64(self)                 -> f64;
    fn as_i8(self)                  -> i8;
    fn as_i64(self)                 -> i64;
    fn from_f64(value: f64)         -> Self;
    fn from_i32(val: i32)           -> Self;
}

impl CountValue for i8 {
    const MIN: Self                 = i8::MIN + 1;
    const MAX: Self                 = i8::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i8 }
    fn from_i32(value: i32)         -> Self { value as i8 }
}

impl CountValue for i16 {
    const MIN: Self                 = i16::MIN + 1;
    const MAX: Self                 = i16::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i16 }
    fn from_i32(value: i32)         -> Self { value as i16 }
}

impl CountValue for i32 {
    const MIN: Self                 = i32::MIN + 1;
    const MAX: Self                 = i32::MAX;
    fn absolute(self)               -> Self { self.abs() }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i32 }
    fn from_i32(value: i32)         -> Self { value }
}


///
#[derive(Component, Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct Count<V: CountValue> {
    anchor:                 V,
    lower_bound:            V,
    upper_bound:            V,
    current_value:          V,
    is_lower_bound_active:  bool,
    is_upper_bound_active:  bool,
}

impl<V: CountValue> Default for Count<V> {

    ///
    fn default() -> Self {
        Self {
            anchor:                 V::from_i32(0),
            lower_bound:            V::MIN,
            upper_bound:            V::MAX,
            current_value:          V::from_i32(0),
            is_lower_bound_active:  true,
            is_upper_bound_active:  true,
        }
    }
}

impl<V: CountValue> Count<V> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// PANIC EVALUATION WILL HAVE TO ACCOUNT FOR WHICH BOUNDARIES ARE ACTIVE
    pub fn new(
        anchor:                 V,
        lower_bound:            V,
        upper_bound:            V,
        current_value:          V,
        is_lower_bound_active:  bool,
        is_upper_bound_active:  bool,
    ) -> Self {

        // MIN AND MAX SHOULDN'T BE CALCULATED BY MIN/MAX FUNCTIONS!  A PANIC SHOULD OCCUR ON CONSTRUCTION
        // IF THE LOWER BOUND IS GREATER THAN THE UPPER BOUND OR IF THE UPPER BOUND IS LESS THAN THE LOWER
        // BOUND!
        let min = lower_bound.min(upper_bound);
        let max = lower_bound.max(upper_bound);

        // PANIC SHOULD OCCUR IF CURRENT_VALUE IS NOT WITHIN BOUNDS!  NO NEED FOR THE ADDITIONAL VALUE
        // CHECKS WITHIN RANGE!
        check_if_value_is_within_range(lower_bound, V::MIN, V::MAX);
        check_if_value_is_within_range(current_value, min, max);
        check_if_value_is_within_range(upper_bound, V::MIN, V::MAX);

        Self {
            anchor,
            lower_bound,
            upper_bound,
            current_value,
            is_lower_bound_active,
            is_upper_bound_active,
        }
    }
    // ######################################################################################## //



    // ##################################### GETTERS ########################################## //

    ///
    #[inline]
    pub fn anchor(&self) -> V {
        self.anchor
    }

    ///
    #[inline]
    pub fn lower_bound(&self) -> V {
        self.lower_bound
    }

    ///
    #[inline]
    pub fn upper_bound(&self) -> V {
        self.upper_bound
    }

    ///
    #[inline]
    pub fn current_value(&self) -> V {
        self.current_value
    }

    ///
    #[inline]
    pub fn is_lower_bound_active(&self) -> bool {
        self.is_lower_bound_active
    }

    ///
    #[inline]
    pub fn is_upper_bound_active(&self) -> bool {
        self.is_upper_bound_active
    }

    ///
    #[inline]
    pub fn is_double_bounded(&self) -> bool {
        self.is_lower_bound_active && self.is_upper_bound_active
    }

    #[inline]
    pub fn digit(&self, place: i32) -> Option<i8> {

        // The divisor for place N is 10^(N-1).
        let divisor = match place {
            1  => V::from_i32(1),
            2  => V::from_i32(10),
            3  => V::from_i32(100),
            4  => V::from_i32(1_000),
            5  => V::from_i32(10_000),
            6  => V::from_i32(100_000),
            7  => V::from_i32(1_000_000),
            8  => V::from_i32(10_000_000),
            9  => V::from_i32(100_000_000),
            10 => V::from_i32(1_000_000_000),
            _  => return None, // out-of-range place
        };

        // Count supports negatives for current_value, must flip to positive for calculation.
        let value = self.current_value.absolute();

        // The ones place always exists; every other place requires current_value to reach it.
        if (place == 1) || (value >= divisor) {
            Some(((value / divisor) % V::from_i32(10)).as_i8())
        }
        else {
            None
        }
    }
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //

    ///
    pub fn set_anchor(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign anchor to the clamped passed value that is following the active bounds.
        self.anchor = passed_value.clamp(active_lower_bound, active_upper_bound);
    }
    
    ///
    pub fn set_lower_bound(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // If the passed value is greater than the upper bound, PANIC.
        // Otherwise, assign the lower bound to the passed value.
        if passed_value > self.upper_bound {
            panic!(
                "{}[COUNT PANIC]{} Count's lower bound can not be set to a value past the upper bound.  You can avoid this panic by doing any of the following:
                1. Make sure you're setting the lower bound of a Count to be below or equal to the upper bound, not above it.
                2. You can use the set_lower_bound_and_swap method on a Count to handle any reordering of bound values if setting the lower bound value exceeds the upper bound value.",
                "\x1b[31m", "\x1b[0m"
            );
        }
        else {
            self.lower_bound = passed_value;
        }

        // If both bounds are active, clamp current_value and anchor to their range.
        // If the lower bound is active and current_value is below it, reassign current_value to the lower_bound value.
        if self.is_lower_bound_active && self.is_upper_bound_active {
            self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
        }
        else if self.is_lower_bound_active && (self.current_value < self.lower_bound) {
            self.current_value = self.lower_bound;
        }
    }

    ///
    pub fn set_lower_bound_and_swap(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // If the passed value is greater than the upper bound, than the lower bound
        // gets reassigned to the upper bound value and the new upper bound value will become the
        // passed value; flip-flopping bound values to ensure the word "lower" remains as it's defined.
        //
        // If the passed value is NOT greater than the upper bound, assign the lower
        // bound value to the passed value.
        if passed_value > self.upper_bound {
            let new_lower_bound: V = self.upper_bound;
            self.upper_bound = passed_value;
            self.lower_bound = new_lower_bound;
        }
        else {
            self.lower_bound = passed_value;
        }

        // If both bounds are active, clamp current_value to their range.
        // If the lower bound is active and current_value is below it, reassign current_value to the lower_bound value.
        if self.is_lower_bound_active && self.is_upper_bound_active {
            self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
        }
        else if self.is_lower_bound_active && (self.current_value < self.lower_bound){
            self.current_value = self.lower_bound;
        }
    }

    ///
    pub fn set_upper_bound(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // If the passed value is greater than the lower bound, PANIC.
        // Otherwise, assign the upper bound to the passed value.
        if passed_value < self.lower_bound {
            panic!(
                "{}[COUNT PANIC]{} Count's upper bound can not be set to a value below the lower bound.  You can avoid this panic by doing any of the following:
                1. Make sure you're setting the upper bound of a Count to be greater or equal to the lower bound, not below it.
                2. You can use the set_upper_bound_and_swap method on a Count to handle any reordering of bound values if setting the upper bound value goes below the lower bound value.",
                "\x1b[31m", "\x1b[0m"
            );
        }
        else {
            self.upper_bound = passed_value;
        }

        // If both bounds are active, clamp current_value to their range.
        // If the upper bound is active and current_value is above it, reassign current_value to the upper_bound value.
        if self.is_lower_bound_active && self.is_upper_bound_active {
            self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
        }
        else if self.is_upper_bound_active && (self.current_value > self.upper_bound){
            self.current_value = self.upper_bound;
        }
    }

    ///
    pub fn set_upper_bound_and_swap(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // If the passed value is less than the lower bound, than the upper bound
        // gets reassigned to the lower bound value and the new lower bound value will become the
        // passed value; flip-flopping bound values to ensure the word "upper" remains as it's defined.
        //
        // If the passed value is NOT less than the lower bound, assign the upper
        // bound value to the passed value.
        if passed_value < self.lower_bound {
            let new_upper_bound: V = self.lower_bound;
            self.lower_bound = passed_value;
            self.upper_bound = new_upper_bound;
        }
        else {
            self.upper_bound = passed_value;
        }

        // If both bounds are active, clamp current_value to their range.
        // If the upper bound is active and current_value is above it, reassign current_value to the upper_bound value.
        if self.is_lower_bound_active && self.is_upper_bound_active {
            self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
        }
        else if self.is_upper_bound_active && (self.current_value > self.upper_bound){
            self.current_value = self.upper_bound;
        }
    }

    ///
    pub fn set_current_value(&mut self, value: V) {

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.clamp(V::MIN, V::MAX);

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign current_value to the clamped passed value that is following the active bounds.
        self.current_value = passed_value.clamp(active_lower_bound, active_upper_bound);
    }

    ///
    /// If current_value is below the activated lower_bound, push it up to meet the lower_bound.
    #[inline]
    pub fn activate_lower_bound(&mut self) {
        self.is_lower_bound_active = true;
        if self.current_value < self.lower_bound {
            self.current_value = self.lower_bound;
        }
    }

    ///
    /// If current_value is above the activated upper_bound, pull it down to meet the upper_bound.
    #[inline]
    pub fn activate_upper_bound(&mut self) {
        self.is_upper_bound_active = true;
        if self.current_value > self.upper_bound {
            self.current_value = self.upper_bound;
        }
    }

    ///
    #[inline]
    pub fn deactivate_lower_bound(&mut self) {
        self.is_lower_bound_active = false;
    }

    ///
    #[inline]
    pub fn deactivate_upper_bound(&mut self) {
        self.is_upper_bound_active = false;
    }

    ///
    #[inline]
    pub fn activate_bounds(&mut self) {
        self.is_lower_bound_active = true;
        self.is_upper_bound_active = true;
        self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
    }

    ///
    #[inline]
    pub fn deactivate_bounds(&mut self) {
        self.is_lower_bound_active = false;
        self.is_upper_bound_active = false;
    }
    // ######################################################################################## //



    // ################################### EQUALITY METHODS ##################################### //
    ///
    #[inline]
    pub fn is_at_lower_bound(&self) -> bool {
        self.current_value == self.lower_bound
    }

    ///
    #[inline]
    pub fn is_at_upper_bound(&self) -> bool {
        self.current_value == self.upper_bound
    }

    ///
    #[inline]
    pub fn bounds_are_equal(&self) -> bool {
        self.lower_bound == self.upper_bound
    }
    // ######################################################################################## //



    // ################################# DIFFERENCE METHODS ################################### //
    ///
    pub fn difference_from_lower_bound(&self) -> i64 {
        let min: i64 = self.current_value.min(self.lower_bound).as_i64();
        let max: i64 = self.current_value.max(self.lower_bound).as_i64();
        max - min
    }

    ///
    pub fn difference_from_upper_bound(&self) -> i64 {
        let min: i64 = self.current_value.min(self.upper_bound).as_i64();
        let max: i64 = self.current_value.max(self.upper_bound).as_i64();
        max - min
    }

    ///
    pub fn difference_between_bounds(&self) -> i64 {
        let min: i64 = self.lower_bound.min(self.upper_bound).as_i64();
        let max: i64 = self.lower_bound.max(self.upper_bound).as_i64();
        max - min
    }
    // ######################################################################################## //



    // ################################### SUM METHODS ######################################## //
    ///
    pub fn sum_to_lower_bound(&mut self, value: V) {

    }

    ///
    #[inline]
    pub fn sum_to_current_value(&mut self, value: V) {

    }

    ///
    pub fn sum_to_upper_bound(&mut self, value: V) {

    }
    // ########################################################################################## //



    // ################################### PERCENTAGE METHODS ################################### //
    ///
    pub fn percentage_completed(&self) -> Option<f64> {

        if self.lower_bound == self.upper_bound {
            return None;
        }

        let start: f64 = self.lower_bound.as_f64();
        let current: f64 = self.current_value.as_f64();
        let end: f64 = self.upper_bound.as_f64();

        let range_reciprocal: f64 = 1.0 / (end - start);

        Some((current - start) * range_reciprocal)
    }

    ///
    pub fn percentage_remaining(&self) -> Option<f64> {

        if self.lower_bound == self.upper_bound {
            return None;
        }

        let start: f64 = self.lower_bound.as_f64();
        let current: f64 = self.current_value.as_f64();
        let end: f64 = self.upper_bound.as_f64();

        let range_reciprocal: f64 = 1.0 / (end - start);

        Some((end - current) * range_reciprocal)
    }
    // ########################################################################################## //



    // ##################################### RESET METHODS ###################################### //
    ///
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.lower_bound;
    }
    // ########################################################################################## //



    // ###################################### HELPER METHODS ######################################## //
    ///
    pub fn print_information(&self) {
        println!("LOWER_BOUND: {}", self.lower_bound);
        println!("CURRENT_VALUE: {}", self.current_value);
        println!("UPPER_BOUND: {}", self.upper_bound);
        println!("IS_LOWER_BOUND_ACTIVE: {}", self.is_lower_bound_active);
        println!("IS_UPPER_BOUND_ACTIVE: {}", self.is_upper_bound_active);
    }
    // ############################################################################################## //
}



// ##################################### PANIC FUNCTIONS ######################################## //
/// Checks if a value falls within the provided minimum and maximum range (inclusive), will `PANIC` if the value is outside the provided range.
/// If a `PANIC` were to occur, a printed message will be displayed to explain how to avoid the `PANIC`.
///
/// Accepts any type that implements [`PartialOrd`] and [`Display`], meaning
/// all numeric primitives, [`char`], [`String`], and [`&str`] are valid inputs.
///
/// #### Example
/// ```ignore
/// check_if_value_is_within_range(5, 1, 10);    // Passes
/// check_if_value_is_within_range(15, 1, 10);   // Panics
/// ```
fn check_if_value_is_within_range<T: PartialOrd + Display>(value: T, minimum: T, maximum: T) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[COUNT PANIC]{} Count value must be between {} and {} (inclusive). Got {}.  You can avoid this panic by doing the following:
        1. Make sure your Count constructor has the current_value set to a number that is between lower_bound and upper_bound (inclusive).
        2. Make sure your Count constructor has the lower_bound set to a number that is between CountValue::MIN and CountValue::MAX (inclusive).
        3. Make sure your Count constructor has the upper_bound set to a number that is between CountValue::MIN and CountValue::MAX (inclusive).",
        "\x1b[31m", "\x1b[0m", minimum, maximum, value
    );
}
// ############################################################################################## //
