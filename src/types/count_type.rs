
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
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
    fn sat_add(self, value: Self)   -> Self;
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
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
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
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
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
    fn sat_add(self, value: Self)   -> Self { self.saturating_add(value) }
    fn as_f64(self)                 -> f64  { self as f64 }
    fn as_i8(self)                  -> i8   { self as i8 }
    fn as_i64(self)                 -> i64  { self as i64 }
    fn from_f64(value: f64)         -> Self { value as i32 }
    fn from_i32(value: i32)         -> Self { value }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum CountMarkers {
    Anchor,
    LowerBound,
    UpperBound,
    CurrentValue,
}

///
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
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
            lower_bound:            V::from_i32(0),
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
        current_value:          V,
        lower_bound:            V,
        upper_bound:            V,
        is_lower_bound_active:  bool,
        is_upper_bound_active:  bool,
    ) -> Self {

        // PANIC EVALUATION
        // Panic if either boundary is being constructed with literals that don't match their definition.
        panic_if_lower_bound_is_greater_than_upper_bound(lower_bound, upper_bound);
        panic_if_upper_bound_is_less_than_lower_bound(lower_bound, upper_bound);

        // PANIC EVALUATION
        // Panic if current_value or anchor are being constructed with literals outside the active boundaries.
        let active_lower_bound = if is_lower_bound_active { lower_bound } else { V::MIN };
        let active_upper_bound = if is_upper_bound_active { upper_bound } else { V::MAX };
        panic_if_value_is_out_of_range("current_value", current_value, active_lower_bound, active_upper_bound);
        panic_if_value_is_out_of_range("anchor", anchor, active_lower_bound, active_upper_bound);

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
    pub fn current_value(&self) -> V {
        self.current_value
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
    // ######################################################################################## //



    // ##################################### SETTERS ########################################## //
    ///
    pub fn set_anchor(&mut self, value: V) {

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign anchor to the clamped passed value that is following the active bounds.
        self.anchor = value.clamp(active_lower_bound, active_upper_bound);
    }

    ///
    pub fn set_current_value(&mut self, value: V) {

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign current_value to the clamped passed value that is following the active bounds.
        self.current_value = value.clamp(active_lower_bound, active_upper_bound);
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
                1. Make sure you're setting the lower bound of a Count to be below or equal to the upper bound, not above it.  Also, the add method uses setters, so make sure to check your usage of it as well.
                2. You can use the set_lower_bound_with_swap method on a Count to handle any reordering of bound values if setting the lower bound value exceeds the upper bound value.  For adding, you can use the add_with_swap to achieve the same functionality.",
                "\x1b[31m", "\x1b[0m"
            );
        }
        else {
            self.lower_bound = passed_value;
        }

        // Clamp the anchor and current_value to the new boundary range.
        self.enforce_bounds();
    }

    ///
    pub fn set_lower_bound_with_swap(&mut self, value: V) {

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

        // Clamp the anchor and current_value to the new boundary range.
        self.enforce_bounds();
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
                1. Make sure you're setting the upper bound of a Count to be greater or equal to the lower bound, not below it.  Also, the add method uses setters, so make sure to check your usage of it as well.
                2. You can use the set_upper_bound_with_swap method on a Count to handle any reordering of bound values if setting the upper bound value goes below the lower bound value.  For adding, you can use the add_with_swap to achieve the same functionality.",
                "\x1b[31m", "\x1b[0m"
            );
        }
        else {
            self.upper_bound = passed_value;
        }

        // Clamp the anchor and current_value to the new boundary range.
        self.enforce_bounds();
    }

    ///
    pub fn set_upper_bound_with_swap(&mut self, value: V) {

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

        // Clamp the anchor and current_value to the new boundary range.
        self.enforce_bounds();
    }

    ///
    #[inline]
    pub fn activate_lower_bound(&mut self) {
        self.is_lower_bound_active = true;
        self.enforce_bounds();
    }

    ///
    #[inline]
    pub fn activate_upper_bound(&mut self) {
        self.is_upper_bound_active = true;
        self.enforce_bounds();
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
        self.enforce_bounds();
    }

    ///
    #[inline]
    pub fn deactivate_bounds(&mut self) {
        self.is_lower_bound_active = false;
        self.is_upper_bound_active = false;
    }
    // ######################################################################################## //



    // ################################### MARKER METHODS ##################################### //

    ///
    pub fn add(
        &mut self,
        value: V,
        marker: CountMarkers
    ) {
        match marker {

            CountMarkers::Anchor => {
                self.set_anchor(self.anchor.sat_add(value));
            }

            CountMarkers::LowerBound => {
                self.set_lower_bound(self.lower_bound.sat_add(value));
            }

            CountMarkers::UpperBound => {
                self.set_upper_bound(self.upper_bound.sat_add(value));
            }

            CountMarkers::CurrentValue => {
                self.set_current_value(self.current_value.sat_add(value));
            }
        }
    }

    ///
    pub fn add_with_swap(
        &mut self,
        value: V,
        marker: CountMarkers
    ) {
        match marker {

            CountMarkers::Anchor => {
                self.set_anchor(self.anchor.sat_add(value));
            }

            CountMarkers::LowerBound => {
                self.set_lower_bound_with_swap(self.lower_bound.sat_add(value));
            }

            CountMarkers::UpperBound => {
                self.set_upper_bound_with_swap(self.upper_bound.sat_add(value));
            }

            CountMarkers::CurrentValue => {
                self.set_current_value(self.current_value.sat_add(value));
            }
        }
    }

    ///
    #[inline]
    pub fn are_markers_equal(
        &self,
        marker_1: CountMarkers,
        marker_2: CountMarkers,
    ) -> bool {
        self.marker_value(marker_1) == self.marker_value(marker_2)
    }

    #[inline]
    pub fn get_digit(
        &self,
        place: i32,
        marker: CountMarkers,
    ) -> Option<i8> {

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

        // Count fields supports negatives, must flip to positive for calculation.
        let value = self.marker_value(marker).absolute();

        // The ones place always exists; every other place requires the marker value to reach it.
        if (place == 1) || (value >= divisor) {
            Some(((value / divisor) % V::from_i32(10)).as_i8())
        }
        else {
            None
        }
    }

    /// PRESERVES POSITIVE/NEGATIVE IN DIFFERENCES TO INDICATE DIRECTION.
    pub fn get_signed_difference(
        &self,
        marker_1: CountMarkers,
        marker_2: CountMarkers,
    ) -> i64 {
        let value_of_marker_1: i64 = self.marker_value(marker_1).as_i64();
        let value_of_marker_2: i64 = self.marker_value(marker_2).as_i64();
        value_of_marker_1 - value_of_marker_2
    }

    /// WILL ALWAYS RETURN A POSITIVE VALUE, THIS DOES INCLUDE THE POSSIBILITY OF 0.
    pub fn get_absolute_difference(
        &self,
        marker_1: CountMarkers,
        marker_2: CountMarkers,
    ) -> i64 {

        let value_of_marker_1: V = self.marker_value(marker_1);
        let value_of_marker_2: V = self.marker_value(marker_2);

        let min: i64 = value_of_marker_1.min(value_of_marker_2).as_i64();
        let max: i64 = value_of_marker_1.max(value_of_marker_2).as_i64();

        max - min
    }

    /// REMEMBER TO MENTION THAT STARTING_MARKER AND ENDING_MARKER CAN BE FLIPPED TO OBTAIN THE INVERSE PERCENTAGE!
    /// MENTION THAT NONE WILL BE RETURNED IN THE CASE THAT START == END
    pub fn get_percentage_of_value(
        &self,
        value_marker: CountMarkers,
        starting_marker: CountMarkers,
        ending_marker: CountMarkers,
    ) -> Option<f64> {

        // Obtaining the values of the markers as f64 floats to ensure the returned percentage holds
        // the highest level of precision possible. A better alternative to this would be allowing
        // the specification of the precision, but I don't got time for that.
        let value: f64 = self.marker_value(value_marker).as_f64();
        let start: f64 = self.marker_value(starting_marker).as_f64();
        let end: f64 = self.marker_value(ending_marker).as_f64();

        // Returning None if start and end are the same value, we do this to avoid dividing by 0.
        // Otherwise, the requested percentage gets returned.
        if start == end {
            None
        }
        else {
            let range_reciprocal: f64 = 1.0 / (end - start);
            Some((value - start) * range_reciprocal)
        }
    }

    /// BASICALLY, GET A VALUE FROM A PERCENTAGE WITHIN A RANGE.
    pub fn get_value_at_percentage(
        &self,
        percentage: f64,
        starting_marker: CountMarkers,
        ending_marker: CountMarkers,
    ) -> V {
        let start: f64 = self.marker_value(starting_marker).as_f64();
        let end: f64 = self.marker_value(ending_marker).as_f64();
        V::from_f64(((end - start) * percentage) + start)
    }
    // ######################################################################################## //



    // ################################# MISCELLANEOUS METHODS ################################## //
    ///
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.anchor;
    }
    // ########################################################################################## //



    // #################################### HELPER METHODS ###################################### //
    ///
    pub fn print_information(&self) {
        println!("ANCHOR : {}", self.anchor);
        println!("CURRENT_VALUE : {}", self.current_value);
        println!("LOWER_BOUND : {}", self.lower_bound);
        println!("UPPER_BOUND : {}", self.upper_bound);
        println!("IS_LOWER_BOUND_ACTIVE : {}", self.is_lower_bound_active);
        println!("IS_UPPER_BOUND_ACTIVE : {}", self.is_upper_bound_active);
        println!("V::MIN : {}", V::MIN);
        println!("V::MAX : {}", V::MAX);
    }

    /// TECHNICALLY NOT NECESSARY FOR PUBLIC USAGE, BUT MAYBE IT COULD BE USED BY OTHERS?
    /// THIS IS USED FOR DIFFERENCE AND PERCENTAGE METHODS SO THAT PARAMETERS ARE ENUM VALUES RATHER THAN STRINGS, BUT
    /// IT MIGHT HAVE A USE BEYOND SUCH THINGS.  DEFINITELY SHOULDN'T BE USED OVER THE GETTERS, THAT WOULD BE SILLY.
    pub fn marker_value(&self, marker: CountMarkers) -> V {
        match marker {
            CountMarkers::Anchor =>         { self.anchor }
            CountMarkers::LowerBound =>     { self.lower_bound }
            CountMarkers::UpperBound =>     { self.upper_bound }
            CountMarkers::CurrentValue =>   { self.current_value }
        }
    }

    /// NOT A PUBLIC METHOD
    fn enforce_bounds(&mut self) {

        match (self.is_lower_bound_active, self.is_upper_bound_active) {

            // Both bounds are active, so we clamp current_value and anchor into the bounded range.
            (true, true) => {
                self.current_value = self.current_value.clamp(self.lower_bound, self.upper_bound);
                self.anchor = self.anchor.clamp(self.lower_bound, self.upper_bound);
            }

            // Only the lower bound is active, so we check to see if current_value or anchor is below it
            // and raise them to the lower bound if they are.
            (true, false) => {
                if self.current_value < self.lower_bound { self.current_value = self.lower_bound; }
                if self.anchor < self.lower_bound { self.anchor = self.lower_bound; }
            }

            // Only the upper bound is active, so we check to see if current_value or anchor is above it
            // and lower them to the upper bound if they are.
            (false, true) => {
                if self.current_value > self.upper_bound { self.current_value = self.upper_bound; }
                if self.anchor > self.upper_bound { self.anchor = self.upper_bound; }
            }

            // Neither bounds are active, so bounds don't need to be enforced.
            (false, false) => {}
        }
    }
    // ############################################################################################## //
}



// ##################################### PANIC FUNCTIONS ######################################## //
/// Checks if a value falls within the provided minimum and maximum range (inclusive), will `PANIC` if the value is outside the provided range.
/// If a `PANIC` were to occur, a printed message will be displayed to explain the cause of the `PANIC`.
///
/// #### Example
/// ```ignore
/// panic_if_value_is_out_of_range(5, 1, 10);    // Passes
/// panic_if_value_is_out_of_range(15, 1, 10);   // Panics
/// ```
fn panic_if_value_is_out_of_range<V: CountValue>(name_of_value: &str, value: V, minimum: V, maximum: V) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[COUNT PANIC]{} You are constructing a Count's {name_of_value} with the value {value}.  {name_of_value} must be between {minimum} and {maximum} (inclusive).",
        "\x1b[31m", "\x1b[0m",
    );
}

///
fn panic_if_lower_bound_is_greater_than_upper_bound<V: CountValue>(lower_bound: V, upper_bound: V) {
    if lower_bound > upper_bound {
        panic!(
            "{}[COUNT PANIC]{} You are constructing a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your lower_bound can not be greater than your upper_bound.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
fn panic_if_upper_bound_is_less_than_lower_bound<V: CountValue>(lower_bound: V, upper_bound: V) {
    if upper_bound > lower_bound {
        panic!(
            "{}[COUNT PANIC]{} You are constructing a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your upper_bound can not be less than your lower_bound.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}
// ############################################################################################## //
