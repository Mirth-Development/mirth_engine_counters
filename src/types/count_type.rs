
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Rem, Sub};
use half::f16;

// ##################################### CountValue TRAIT ####################################### //
///
pub trait CountValue:
Copy                    // CountValue types are safe to copy.
+ PartialOrd            // Every supported type can be compared.
+ Display               // Making it so values can be printed to the console.
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Mul<Output = Self>
+ Rem<Output = Self>
+ Send                  // Needed for Bevy queries; also lets Counts move safely across threads.
+ Sync                  // Needed for Bevy queries; also lets Counts be shared safely across threads.
+ 'static               // Needed for Bevy queries; also enforces that CountValue types own their data, with no borrowed lifetimes.
{
    /// Text
    type Difference;

    /// Text
    const MIN: Self;

    /// Text
    const MAX: Self;

    /// Text
    const IS_FLOAT: bool;

    /// Text
    const EPSILON: Self;

    /// Text
    const MAX_FLOATING_PLACE: i8;

    /// Text
    fn signed_difference(from: Self, to: Self) -> Self::Difference;

    /// Text
    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference;

    /// Text
    fn absolute(self) -> Self;

    /// Text
    fn sat_add(self, value: Self) -> Self;

    /// Text
    fn truncate(self) -> Self;

    /// Text
    fn count_min(self, other: Self) -> Self;

    /// Text
    fn count_max(self, other: Self) -> Self;

    /// Text
    fn count_clamp(self, min: Self, max: Self) -> Self;

    /// Text
    fn is_nan(self) -> bool;

    /// Text
    fn as_f64(self) -> f64;

    /// INT TO INT CASTING DOESN'T SATURATE, IT WRAPS.  MENTION THIS FOR UNSIGNED TYPES.
    fn as_i8(self) -> i8;

    /// Text
    fn as_i64(self) -> i64;

    /// Text
    fn from_f64(value: f64) -> Self;

    /// INT TO INT CASTING DOESN'T SATURATE, IT WRAPS.  MENTION THIS FOR SIGNED AND UNSIGNED TYPES.
    fn from_i64(value: i64) -> Self;
}
impl CountValue for u8 {
    type Difference = i16;

    const MIN: Self = u8::MIN;

    const MAX: Self = u8::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self.clamp(0, i8::MAX as u8) as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as u8 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u8 }
}
impl CountValue for u16 {
    type Difference = i32;

    const MIN: Self = u16::MIN;

    const MAX: Self = u16::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self.clamp(0, i8::MAX as u16) as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as u16 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u16 }
}
impl CountValue for u32 {
    type Difference = i64;

    const MIN: Self = u32::MIN;

    const MAX: Self = u32::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self.clamp(0, i8::MAX as u32) as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as u32 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u32 }
}
impl CountValue for i8 {
    type Difference = i16;

    const MIN: Self = i8::MIN + 1;

    const MAX: Self = i8::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value as i8 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i8 }
}
impl CountValue for i16 {
    type Difference = i32;

    const MIN: Self = i16::MIN + 1;

    const MAX: Self = i16::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value as i16 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i16 }
}
impl CountValue for i32 {
    type Difference = i64;

    const MIN: Self = i32::MIN + 1;

    const MAX: Self = i32::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_FLOATING_PLACE: i8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn truncate(self) -> Self
    { self }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { false }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value as i32 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i32 }
}
impl CountValue for f16 {
    type Difference = f32;

    const MIN: Self = f16::MIN;

    const MAX: Self = f16::MAX;

    const IS_FLOAT: bool = true;

    const EPSILON: Self = f16::from_f32_const(1e-3);

    const MAX_FLOATING_PLACE: i8 = 2;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { to.to_f32() - from.to_f32() }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { (val_1.to_f32() - val_2.to_f32()).abs() }

    fn absolute(self) -> Self
    { if self < f16::from_f32(0.0) { -self } else { self } }

    fn sat_add(self, value: Self) -> Self
    { (self + value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn truncate(self) -> Self
    { f16::from_f32(self.to_f32().trunc()) }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { self.is_nan() }

    fn as_f64(self) -> f64
    { self.to_f64() }

    fn as_i8(self) -> i8
    { self.to_f64() as i8 }

    fn as_i64(self) -> i64
    { self.to_f64() as i64 }

    fn from_f64(value: f64) -> Self
    { f16::from_f64(value) }

    fn from_i64(value: i64) -> Self
    { f16::from_f64(value as f64) }
}
impl CountValue for f32 {
    type Difference = f64;

    const MIN: Self = f32::MIN;

    const MAX: Self = f32::MAX;

    const IS_FLOAT: bool = true;

    const EPSILON: Self = 1e-5;

    const MAX_FLOATING_PLACE: i8 = 4;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn sat_add(self, value: Self) -> Self
    { (self + value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn truncate(self) -> Self
    { self.trunc() }

    fn count_min(self, other: Self) -> Self
    { self.min(other) }

    fn count_max(self, other: Self) -> Self
    { self.max(other) }

    fn count_clamp(self, min: Self, max: Self) -> Self
    { self.clamp(min, max) }

    fn is_nan(self) -> bool
    { self.is_nan() }

    fn as_f64(self) -> f64
    { self as f64 }

    fn as_i8(self) -> i8
    { self as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value as f32 }

    fn from_i64(value: i64) -> Self
    { value as f32 }
}



// ################################### CountDiffCaster TRAIT #################################### //
///
pub trait CountDiffCaster {
    fn from_f64(value: f64) -> Self;
}
impl CountDiffCaster for f64 {
    fn from_f64(value: f64) -> Self { value }
}
impl CountDiffCaster for i64 {
    fn from_f64(value: f64) -> Self { value as i64 }
}



// ##################################### CountMarkers ENUM ###################################### //
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum CountMarkers {
    Anchor,
    LowerBound,
    UpperBound,
    CurrentValue,
}



// ####################################### Count STRUCT ######################################### //
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
            anchor:                 V::from_i64(0),
            lower_bound:            V::from_i64(0),
            upper_bound:            V::MAX,
            current_value:          V::from_i64(0),
            is_lower_bound_active:  true,
            is_upper_bound_active:  true,
        }
    }
}
impl<V: CountValue> Count<V> {

    // ##################################### CONSTRUCTORS ######################################## //
    /// PANIC EVALUATION ACCOUNTS FOR WHICH BOUNDARIES ARE ACTIVE
    pub fn new(
        anchor:                 V,
        current_value:          V,
        lower_bound:            V,
        upper_bound:            V,
        is_lower_bound_active:  bool,
        is_upper_bound_active:  bool,
    ) -> Self {

        // PANIC EVALUATION
        // Panic if a passed value for anchor, current_value, lower_bound, or upper_bound is NaN.
        panic_if_is_nan("anchor", "constructing", anchor);
        panic_if_is_nan("current_value", "constructing", current_value);
        panic_if_is_nan("lower_bound", "constructing", lower_bound);
        panic_if_is_nan("upper_bound", "constructing", upper_bound);

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




    // ##################################### SETTERS ########################################## //
    ///
    pub fn set_anchor(&mut self, value: V) {

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("anchor", "setting or adding", value);

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign anchor to the clamped passed value that is following the active bounds.
        self.anchor = value.count_clamp(active_lower_bound, active_upper_bound);
    }

    ///
    pub fn set_current_value(&mut self, value: V) {

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("current_value", "setting or adding", value);

        // Determine the active bounds.
        // If a bound is inactive, they are replaced by V::MIN or V::MAX depending on which bound is inactive.
        let active_lower_bound = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
        let active_upper_bound = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };

        // Reassign current_value to the clamped passed value that is following the active bounds.
        self.current_value = value.count_clamp(active_lower_bound, active_upper_bound);
    }

    ///
    pub fn set_lower_bound(&mut self, value: V) {

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("lower_bound", "setting or adding", value);

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.count_clamp(V::MIN, V::MAX);

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

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("lower_bound", "setting or adding", value);

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.count_clamp(V::MIN, V::MAX);

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

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("upper_bound", "setting or adding", value);

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.count_clamp(V::MIN, V::MAX);

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

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("upper_bound", "setting or adding", value);

        // Pushing up/down the passed value to be within the acceptable range for the Count datatype.
        let passed_value: V = value.count_clamp(V::MIN, V::MAX);

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

    ///
    pub fn get_digit(
        &self,
        place: i8,
        marker: CountMarkers,
    ) -> Option<i8> {

        let scaler: V = match place {
            1  => V::from_i64(1),
            2  => V::from_i64(10),
            3  => V::from_i64(100),
            4  => V::from_i64(1_000),
            5  => V::from_i64(10_000),
            6  => V::from_i64(100_000),
            7  => V::from_i64(1_000_000),
            8  => V::from_i64(10_000_000),
            9  => V::from_i64(100_000_000),
            10 => V::from_i64(1_000_000_000),

            // Out-Of-Range Places
            _  => return None,
        };

        // Count fields supports negatives, must flip to positive for calculation.
        let value = self.marker_value(marker).absolute();

        // This looks more complicated than it needs to be because I attempted to make get_digit
        // have float support - it failed miserably.  If I do end up one day coming back to it, then
        // I'm going to need this match statement; trying to save myself later.
        let is_whole_place: bool = (place == 1) || (value >= scaler);
        match (V::IS_FLOAT, is_whole_place) {

            // Integers/floats requesting whole digits.
            (_, true) => {
                let integer_part = (value / scaler).truncate();
                Some((integer_part % V::from_i64(10)).as_i8())
            }

            _ => None,
        }
    }

    /// Returns `to_marker`'s value minus `from_marker`'s value, preserving sign to indicate direction:
    /// - **Positive Result**: `to_marker` sits to the right of (greater than) `from_marker`.
    /// - **Negative Result**: `to_marker` sits to the left of (less than) `from_marker`.
    /// - **Zero Result**: The two markers currently hold equal values.
    pub fn get_signed_difference(
        &self,
        from_marker: CountMarkers,
        to_marker: CountMarkers,
    ) -> V::Difference {
        let from_value: V = self.marker_value(from_marker);
        let to_value: V = self.marker_value(to_marker);
        V::signed_difference(to_value, from_value)
    }

    /// WILL ALWAYS RETURN A POSITIVE VALUE, THIS DOES INCLUDE THE POSSIBILITY OF 0.
    pub fn get_absolute_difference(
        &self,
        marker_1: CountMarkers,
        marker_2: CountMarkers,
    ) -> V::Difference {
        let value_1: V = self.marker_value(marker_1);
        let value_2: V = self.marker_value(marker_2);
        V::absolute_difference(value_1, value_2)
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
        percentage: f32,
        starting_marker: CountMarkers,
        ending_marker: CountMarkers,
    ) -> V {

        // PANIC EVALUATION
        // Passed value can not be NaN.
        panic_if_is_nan("value from a percentage", "getting", percentage);

        // Using f64 for calculation to increase the precision of the result.  There will be a lossy
        // conversion for what V is since
        let modified_percentage: f64 = percentage.as_f64();
        let start: f64 = self.marker_value(starting_marker).as_f64();
        let end: f64 = self.marker_value(ending_marker).as_f64();
        V::from_f64(((end - start) * modified_percentage) + start)
    }



    // ################################# MISCELLANEOUS METHODS ################################## //
    ///
    #[inline]
    pub fn reset(&mut self) {
        self.current_value = self.anchor;
    }



    // #################################### HELPER METHODS ###################################### //
    ///
    #[inline]
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
    #[inline]
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
                self.current_value = self.current_value.count_clamp(self.lower_bound, self.upper_bound);
                self.anchor = self.anchor.count_clamp(self.lower_bound, self.upper_bound);
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
#[inline]
fn panic_if_value_is_out_of_range<V: CountValue>(name_of_value: &str, value: V, minimum: V, maximum: V) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[COUNT PANIC]{} You are constructing a Count's {name_of_value} with the value {value}.  {name_of_value} must be between {minimum} and {maximum} (inclusive).",
        "\x1b[31m", "\x1b[0m",
    );
}

///
#[inline]
fn panic_if_lower_bound_is_greater_than_upper_bound<V: CountValue>(lower_bound: V, upper_bound: V) {
    if lower_bound > upper_bound {
        panic!(
            "{}[COUNT PANIC]{} You are constructing a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your lower_bound can not be greater than your upper_bound.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
#[inline]
fn panic_if_upper_bound_is_less_than_lower_bound<V: CountValue>(lower_bound: V, upper_bound: V) {
    if upper_bound < lower_bound {
        panic!(
            "{}[COUNT PANIC]{} You are constructing a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your upper_bound can not be less than your lower_bound.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
#[inline]
fn panic_if_is_nan<V: CountValue>(name_of_value: &str, name_of_action: &str, value: V) {
    if value.is_nan() {
        panic!(
            "{}[COUNT PANIC]{} You are {name_of_action} a Count's {name_of_value} with NaN.
            NaN is not a valid CountValue for any comparison, bound, or arithmetic operation.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}
