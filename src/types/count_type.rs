
// Imports
use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Rem, Sub};
use half::f16;
use crate::types::operation_type::Operation;

// ##################################### CountValue TRAIT ####################################### //
/// THIS IS A TEST, AGAIN
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

    ///
    const MAX_WHOLE_PLACES: u8;

    /// Text
    const MAX_FLOATING_PLACES: u8;

    /// Text
    const RELIABLE_FLOATING_PLACES: u8;

    /// Text
    fn signed_difference(from: Self, to: Self) -> Self::Difference;

    /// Text
    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference;

    /// Text
    fn absolute(self) -> Self;

    /// Text
    fn power_with_int(self, value: u32) -> Self;

    /// FLOATS DON'T HAVE SAT FUNCTIONS, SO THEIR IMPL IS MANUALLY DONE THROUGH CLAMP
    fn sat_add(self, value: Self) -> Self;

    /// FLOATS DON'T HAVE SAT FUNCTIONS, SO THEIR IMPL IS MANUALLY DONE THROUGH CLAMP
    fn sat_subtract(self, value: Self) -> Self;

    /// FLOATS DON'T HAVE SAT FUNCTIONS, SO THEIR IMPL IS MANUALLY DONE THROUGH CLAMP
    fn sat_multiply(self, value: Self) -> Self;

    /// FLOATS DON'T HAVE SAT FUNCTIONS, SO THEIR IMPL IS MANUALLY DONE THROUGH CLAMP
    fn sat_divide(self, value: Self) -> Self;

    /// FLOATS DON'T HAVE SAT FUNCTIONS, SO THEIR IMPL IS MANUALLY DONE THROUGH CLAMP
    fn sat_power(self, value: Self) -> Self;

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

    /// This always saturates to `i8::MIN` - `i8::MAX` rather than wrapping.
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

    const MAX_WHOLE_PLACES: u8 = 3;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value as u32) }

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
    { value as u8 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u8 }
}
impl CountValue for u16 {
    type Difference = i32;

    const MIN: Self = u16::MIN;

    const MAX: Self = u16::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_WHOLE_PLACES: u8 = 5;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value as u32) }

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
    { value as u16 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u16 }
}
impl CountValue for u32 {
    type Difference = i64;

    const MIN: Self = u32::MIN;

    const MAX: Self = u32::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_WHOLE_PLACES: u8 = 10;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value) }

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
    { value as u32 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as u32 }
}
impl CountValue for i8 {
    type Difference = i16;

    const MIN: Self = i8::MIN + 1;

    const MAX: Self = i8::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_WHOLE_PLACES: u8 = 3;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value as u32) }

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
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as i8 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i8 }
}
impl CountValue for i16 {
    type Difference = i32;

    const MIN: Self = i16::MIN + 1;

    const MAX: Self = i16::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_WHOLE_PLACES: u8 = 5;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value as u32) }

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
    { self.clamp(i8::MIN as i16, i8::MAX as i16) as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as i16 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i16 }
}
impl CountValue for i32 {
    type Difference = i64;

    const MIN: Self = i32::MIN + 1;

    const MAX: Self = i32::MAX;

    const IS_FLOAT: bool = false;

    const EPSILON: Self = 0;

    const MAX_WHOLE_PLACES: u8 = 10;

    const MAX_FLOATING_PLACES: u8 = 0;

    const RELIABLE_FLOATING_PLACES: u8 = 0;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn power_with_int(self, value: u32) -> Self
    { self.pow(value) }

    fn sat_add(self, value: Self) -> Self
    { self.saturating_add(value) }

    fn sat_subtract(self, value: Self) -> Self
    { self.saturating_sub(value) }

    fn sat_multiply(self, value: Self) -> Self
    { self.saturating_mul(value) }

    fn sat_divide(self, value: Self) -> Self
    { self.saturating_div(value) }

    fn sat_power(self, value: Self) -> Self
    { self.saturating_pow(value as u32) }

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
    { self.clamp(i8::MIN as i32, i8::MAX as i32) as i8 }

    fn as_i64(self) -> i64
    { self as i64 }

    fn from_f64(value: f64) -> Self
    { value.clamp(<Self as CountValue>::MIN as f64, <Self as CountValue>::MAX as f64) as i32 }

    fn from_i64(value: i64) -> Self
    { value.clamp(<Self as CountValue>::MIN as i64, <Self as CountValue>::MAX as i64) as i32 }
}
impl CountValue for f16 {
    type Difference = f32;

    const MIN: Self = f16::MIN;

    const MAX: Self = f16::MAX;

    const IS_FLOAT: bool = true;

    const EPSILON: Self = f16::from_f32_const(1e-3);

    const MAX_WHOLE_PLACES: u8 = 5;

    const MAX_FLOATING_PLACES: u8 = 8;

    const RELIABLE_FLOATING_PLACES: u8 = 2;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { to.to_f32() - from.to_f32() }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { (val_1.to_f32() - val_2.to_f32()).abs() }

    fn absolute(self) -> Self
    { if self < f16::from_f32(0.0) { -self } else { self } }

    fn power_with_int(self, value: u32) -> Self
    { f16::from_f32(self.to_f32().powi(value as i32)) }

    fn sat_add(self, value: Self) -> Self
    { (self + value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_subtract(self, value: Self) -> Self
    { (self - value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_multiply(self, value: Self) -> Self
    { (self * value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_divide(self, value: Self) -> Self
    { (self / value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_power(self, value: Self) -> Self
    { f16::from_f32(self.to_f32().powf(value.to_f32())).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

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

    const EPSILON: Self = 1e-6;

    const MAX_WHOLE_PLACES: u8 = 39;

    const MAX_FLOATING_PLACES: u8 = 45;

    const RELIABLE_FLOATING_PLACES: u8 = 5;

    fn signed_difference(from: Self, to: Self) -> Self::Difference
    { (to as Self::Difference) - (from as Self::Difference) }

    fn absolute_difference(val_1: Self, val_2: Self) -> Self::Difference
    { ((val_1 as Self::Difference) - (val_2 as Self::Difference)).abs() }

    fn absolute(self) -> Self
    { self.abs() }

    fn power_with_int(self, value: u32) -> Self
    { self.powi(value as i32) }

    fn sat_add(self, value: Self) -> Self
    { (self + value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }
    fn sat_subtract(self, value: Self) -> Self
    { (self - value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_multiply(self, value: Self) -> Self
    { (self * value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_divide(self, value: Self) -> Self
    { (self / value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

    fn sat_power(self, value: Self) -> Self
    { self.powf(value).clamp(<Self as CountValue>::MIN, <Self as CountValue>::MAX) }

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

// ###################################### CountMarker ENUM ###################################### //
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum CountMarker {
    Anchor,
    Value,
    LowerBound,
    UpperBound,
}

// ####################################### CountBound ENUM ###################################### //
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum CountBound {
    Lower,
    Upper,
}

// ######################################## CountError ENUM ##################################### //
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CountError<V: CountValue> {
    ExceedsLowerLimit{
        value: V,
        limit: V,
        name_of_value: &'static str,
    },
    ExceedsUpperLimit{
        value: V,
        limit: V,
        name_of_value: &'static str,
    }
}
impl<V: CountValue> Display for CountError<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CountError::ExceedsLowerLimit { value, limit, name_of_value } => {
                write!(f,
                       "{}[COUNT ERROR]{} A Count's {name_of_value} can not be set to {value} as that is below its current lower limit of {limit}.  Here are some notes about this error:
                       1. When changing the upper_bound of a Count, you can not go below the lower_bound even if the lower_bound is inactive.
                       2. When changing the anchor or value of a Count, you can not go below the lower_bound when it's active.  But you are able to when it's inactive.
                       3. When changing the anchor or value of a Count, you can not go below its CountValue's MIN.",
                       "\x1b[31m", "\x1b[0m"
                )
            },
            CountError::ExceedsUpperLimit { value, limit, name_of_value } => {
                write!(f,
                       "{}[COUNT ERROR]{} A Count's {name_of_value} can not be set to {value} as that is above its current upper limit of {limit}.  Here are some notes about this error:
                       1. When changing the lower_bound of a Count, you can not go above the upper_bound even if the upper_bound is inactive.
                       2. When changing the anchor or value of a Count, you can not go above the upper_bound when it's active.  But you are able to when it's inactive.
                       3. When changing the anchor or value of a Count, you can not go above its CountValue's MAX.",
                       "\x1b[31m", "\x1b[0m"
                )
            },
        }
    }
}



// ####################################### Count STRUCT ######################################### //
/// EXPLAIN THE REASON WHY I'M USING THE MARKER ENUM, BOUND ENUM, COUNTERRROR ENUM, AND OPERATION ENUM!
/// IT'S FOR MAINTAINABILITY!
///
/// EXPLAIN THE DIFFERENCE BETWEEN "LIMIT" AND "BOUND"!
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "count_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "count_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub struct Count<V: CountValue> {
    anchor:                 V,
    value:                  V,
    lower_bound:            V,
    upper_bound:            V,
    is_lower_bound_active:  bool,
    is_upper_bound_active:  bool,
}
impl<V: CountValue> Default for Count<V> {

    ///
    fn default() -> Self {
        Self {
            anchor:                 V::from_i64(0),
            value:                  V::from_i64(0),
            lower_bound:            V::MIN,
            upper_bound:            V::MAX,
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
        value:                  V,
        lower_bound:            V,
        upper_bound:            V,
        is_lower_bound_active:  bool,
        is_upper_bound_active:  bool,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_construction_is_invalid(
            anchor,
            value,
            lower_bound,
            upper_bound,
            is_lower_bound_active,
            is_upper_bound_active
        );

        Self {
            anchor,
            value,
            lower_bound,
            upper_bound,
            is_lower_bound_active,
            is_upper_bound_active,
        }
    }

    /// PANIC EVALUATION ACCOUNTS FOR WHICH BOUNDARIES ARE ACTIVE
    pub fn new_with_active_bounds(
        anchor:         V,
        value:          V,
        lower_bound:    V,
        upper_bound:    V,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_construction_is_invalid(
            anchor,
            value,
            lower_bound,
            upper_bound,
            true,
            true
        );

        Self {
            anchor,
            value,
            lower_bound,
            upper_bound,
            is_lower_bound_active: true,
            is_upper_bound_active: true,
        }
    }

    /// PANIC EVALUATION ACCOUNTS FOR WHICH BOUNDARIES ARE ACTIVE
    pub fn new_with_inactive_bounds(
        anchor:         V,
        value:          V,
        lower_bound:    V,
        upper_bound:    V,
    ) -> Self {

        // PANIC EVALUATION
        panic_if_construction_is_invalid(
            anchor,
            value,
            lower_bound,
            upper_bound,
            false,
            false
        );

        Self {
            anchor,
            value,
            lower_bound,
            upper_bound,
            is_lower_bound_active: false,
            is_upper_bound_active: false,
        }
    }



    // ###################################### GETTERS ########################################### //
    /// Returns the number of the anchor marker.
    #[inline]
    pub fn anchor(&self) -> V {
        self.anchor
    }

    /// Returns the number of the value marker.
    #[inline]
    pub fn value(&self) -> V {
        self.value
    }

    /// Returns the number of the lower_bound marker.
    #[inline]
    pub fn lower_bound(&self) -> V {
        self.lower_bound
    }

    /// Returns the number of the upper_bound marker.
    #[inline]
    pub fn upper_bound(&self) -> V {
        self.upper_bound
    }

    /// Returns true if the lower_bound is active, false otherwise.
    #[inline]
    pub fn is_lower_bound_active(&self) -> bool {
        self.is_lower_bound_active
    }

    /// Returns true if the upper_bound is active, false otherwise.
    #[inline]
    pub fn is_upper_bound_active(&self) -> bool {
        self.is_upper_bound_active
    }



    // ###################################### SETTERS ########################################### //
    /// Sets a specified marker to the given number. Setting a bound marker that is active will clamp
    /// anchor and value to the new boundary range. Will return a CountError if either the lower or
    /// upper limit is exceeded for the specified marker.
    ///
    /// ### Panic Warning
    /// Will panic if NaN is passed in for value.
    ///
    /// ### Why Does This Exist?
    /// Allows for the control of what happens when exceeding a limit, rather than auto-clamping like
    /// .set_marker_with_limits().  Use .set_marker() when you need to specify a unique event when
    /// overflow or underflow would occur.
    pub fn set_marker(
        &mut self,
        marker: CountMarker,
        number: V,
    ) -> Result<(), CountError<V>> {

        // PANIC EVALUATION
        panic_if_is_nan(self.marker_name(marker), "setting", number);

        match marker {

            CountMarker::Anchor => {
                if      (number < self.lower_bound) && self.is_lower_bound_active { return Err(CountError::<V>::ExceedsLowerLimit{ value: number, limit: self.lower_bound, name_of_value: "anchor" }); }
                else if (number > self.upper_bound) && self.is_upper_bound_active { return Err(CountError::<V>::ExceedsUpperLimit{ value: number, limit: self.upper_bound, name_of_value: "anchor" }); }
                else if number < V::MIN { return Err(CountError::<V>::ExceedsLowerLimit{ value: number, limit: V::MIN, name_of_value: "anchor" }); }
                else if number > V::MAX { return Err(CountError::<V>::ExceedsUpperLimit{ value: number, limit: V::MAX, name_of_value: "anchor" }); }
                self.anchor = number;
            }

            CountMarker::Value => {
                if      (number < self.lower_bound) && self.is_lower_bound_active { return Err(CountError::<V>::ExceedsLowerLimit{ value: number, limit: self.lower_bound, name_of_value: "number" }); }
                else if (number > self.upper_bound) && self.is_upper_bound_active { return Err(CountError::<V>::ExceedsUpperLimit{ value: number, limit: self.upper_bound, name_of_value: "number" }); }
                else if number < V::MIN { return Err(CountError::<V>::ExceedsLowerLimit { value: number, limit: V::MIN, name_of_value: "number" }); }
                else if number > V::MAX { return Err(CountError::<V>::ExceedsUpperLimit { value: number, limit: V::MAX, name_of_value: "number" }); }
                self.value = number;
            }

            CountMarker::LowerBound => {
                if      number > self.upper_bound { return Err(CountError::<V>::ExceedsUpperLimit { value: number, limit: self.upper_bound, name_of_value: "lower_bound" }); }
                else if number < V::MIN { return Err(CountError::<V>::ExceedsLowerLimit { value: number, limit: V::MIN, name_of_value: "lower_bound" }); }
                self.lower_bound = number;
                self.enforce_bounds();
            }

            CountMarker::UpperBound => {
                if      number < self.lower_bound { return Err(CountError::<V>::ExceedsLowerLimit { value: number, limit: self.lower_bound, name_of_value: "upper_bound" }); }
                else if number > V::MAX { return Err(CountError::<V>::ExceedsUpperLimit { value: number, limit: V::MAX, name_of_value: "upper_bound" }); }
                self.upper_bound = number;
                self.enforce_bounds();
            }
        }

        Ok(())
    }

    /// Sets a specified marker to the given number, the number is clamped to the marker's limits.
    /// Setting a bound marker that is active will clamp anchor and value to the new boundary range.
    ///
    /// ### Panic Warning
    /// Will panic if NaN is passed in for value.
    ///
    /// ### Why Does This Exist?
    /// To ease clamping markers to their limits.  This method doesn't require error handling like
    /// .set_marker() does, so it makes working on markers that are purposed to not have unique events
    /// on overflow/underflow easier; keeps things within their intended boundaries without handling.
    pub fn set_marker_with_limits(
        &mut self,
        marker: CountMarker,
        number: V,
    ) {

        // PANIC EVALUATION
        panic_if_is_nan(self.marker_name(marker), "setting", number);

        match marker {

            CountMarker::Anchor => {
                let lower_limit = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
                let upper_limit = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };
                self.anchor = number.count_clamp(lower_limit, upper_limit);
            }

            CountMarker::Value => {
                let lower_limit = if self.is_lower_bound_active { self.lower_bound } else { V::MIN };
                let upper_limit = if self.is_upper_bound_active { self.upper_bound } else { V::MAX };
                self.value = number.count_clamp(lower_limit, upper_limit);
            }

            CountMarker::LowerBound => {
                self.lower_bound = number.count_clamp(V::MIN, self.upper_bound);
                self.enforce_bounds();
            }

            CountMarker::UpperBound => {
                self.lower_bound = number.count_clamp(self.lower_bound, V::MAX);
                self.enforce_bounds();
            }
        }
    }

    /// Turns on a specified CountBound.
    ///
    /// Turning on a bound will clamp the anchor and value markers to the updated boundary range,
    /// and this will prevent them from exceeding the specified CountBound.
    #[inline]
    pub fn activate_bound(&mut self, bound: CountBound) {
        match bound {
            CountBound::Lower => self.is_lower_bound_active = true,
            CountBound::Upper => self.is_upper_bound_active = true,
        }
        self.enforce_bounds();
    }

    /// Turns off a specified CountBound.
    ///
    /// This will allow the anchor and value markers to exceed the specified CountBound.
    #[inline]
    pub fn deactivate_bound(&mut self, bound: CountBound) {
        match bound {
            CountBound::Lower => self.is_lower_bound_active = false,
            CountBound::Upper => self.is_upper_bound_active = false,
        }
    }


    // ################################### MARKER METHODS ##################################### //
    /// Grants the ability to apply a specified Operation onto the provided CountMarker by the number
    /// that is passed in.  Will return a CountError if the operation ends up pushing a marker past
    /// its lower or upper limit.
    ///
    /// ### Panic Warnings
    /// - Will panic if NaN is passed in for value.
    /// - Will panic if you choose to divide by 0.
    ///
    /// ### Why Does This Exist?
    /// To allow operations on a marker and to grant you the ability to define what happens when an
    /// operation causes a marker to exceed one of its limits.  If you'd like to just clamp to a limit
    /// when it has been exceeded, use .operate_with_limits().
    #[inline]
    pub fn operate(
        &mut self,
        operation: Operation,
        marker: CountMarker,
        number: V,
    ) -> Result<(), CountError<V>> {

        // PANIC EVALUATION
        panic_if_is_nan(self.marker_name(marker), "operating on", number);

        match operation {

            Operation::Add => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker(CountMarker::Anchor, self.anchor.sat_add(number)) }
                    CountMarker::Value         => { self.set_marker(CountMarker::Value, self.value.sat_add(number)) }
                    CountMarker::LowerBound    => { self.set_marker(CountMarker::LowerBound, self.lower_bound.sat_add(number)) }
                    CountMarker::UpperBound    => { self.set_marker(CountMarker::UpperBound, self.upper_bound.sat_add(number)) }
                }
            }

            Operation::Subtract => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker(CountMarker::Anchor, self.anchor.sat_subtract(number)) }
                    CountMarker::Value         => { self.set_marker(CountMarker::Value, self.value.sat_subtract(number)) }
                    CountMarker::LowerBound    => { self.set_marker(CountMarker::LowerBound, self.lower_bound.sat_subtract(number)) }
                    CountMarker::UpperBound    => { self.set_marker(CountMarker::UpperBound, self.upper_bound.sat_subtract(number)) }
                }
            }

            Operation::Multiply => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker(CountMarker::Anchor, self.anchor.sat_multiply(number)) }
                    CountMarker::Value         => { self.set_marker(CountMarker::Value, self.value.sat_multiply(number)) }
                    CountMarker::LowerBound    => { self.set_marker(CountMarker::LowerBound, self.lower_bound.sat_multiply(number)) }
                    CountMarker::UpperBound    => { self.set_marker(CountMarker::UpperBound, self.upper_bound.sat_multiply(number)) }
                }
            }

            Operation::Divide => {
                panic_if_zero(self.marker_name(marker), "dividing", number);
                match marker {
                    CountMarker::Anchor        => { self.set_marker(CountMarker::Anchor, self.anchor.sat_divide(number)) }
                    CountMarker::Value         => { self.set_marker(CountMarker::Value, self.value.sat_divide(number)) }
                    CountMarker::LowerBound    => { self.set_marker(CountMarker::LowerBound, self.lower_bound.sat_divide(number)) }
                    CountMarker::UpperBound    => { self.set_marker(CountMarker::UpperBound, self.upper_bound.sat_divide(number)) }
                }
            }

            Operation::Power => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker(CountMarker::Anchor, self.anchor.sat_power(number)) }
                    CountMarker::Value         => { self.set_marker(CountMarker::Value, self.value.sat_power(number)) }
                    CountMarker::LowerBound    => { self.set_marker(CountMarker::LowerBound, self.lower_bound.sat_power(number)) }
                    CountMarker::UpperBound    => { self.set_marker(CountMarker::UpperBound, self.upper_bound.sat_power(number)) }
                }
            }
        }
    }

    /// Grants the ability to apply a specified Operation onto the provided CountMarker by the number
    /// that is passed in.  The operation does not have the ability to exceed a marker's limits.
    ///
    /// ### Panic Warnings
    /// - Will panic if NaN is passed in for value.
    /// - Will panic if you choose to divide by 0.
    ///
    /// ### Why Does This Exist?
    /// To allow operations on a marker without having to worry about the operation pushing a marker
    /// past its limits.  This method uses the logic from .set_marker_with_limits(), so when operating
    /// on a bound marker that is active it will clamp anchor and value to the new boundary range.
    #[inline]
    pub fn operate_with_limits(
        &mut self,
        operation: Operation,
        marker: CountMarker,
        number: V,
    ) {

        // PANIC EVALUATION
        panic_if_is_nan(self.marker_name(marker), "operating on", number);

        match operation {

            Operation::Add => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker_with_limits(CountMarker::Anchor, self.anchor.sat_add(number)) }
                    CountMarker::Value         => { self.set_marker_with_limits(CountMarker::Value, self.value.sat_add(number)) }
                    CountMarker::LowerBound    => { self.set_marker_with_limits(CountMarker::LowerBound, self.lower_bound.sat_add(number)) }
                    CountMarker::UpperBound    => { self.set_marker_with_limits(CountMarker::UpperBound, self.upper_bound.sat_add(number)) }
                }
            }

            Operation::Subtract => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker_with_limits(CountMarker::Anchor, self.anchor.sat_subtract(number)) }
                    CountMarker::Value         => { self.set_marker_with_limits(CountMarker::Value, self.value.sat_subtract(number)) }
                    CountMarker::LowerBound    => { self.set_marker_with_limits(CountMarker::LowerBound, self.lower_bound.sat_subtract(number)) }
                    CountMarker::UpperBound    => { self.set_marker_with_limits(CountMarker::UpperBound, self.upper_bound.sat_subtract(number)) }
                }
            }

            Operation::Multiply => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker_with_limits(CountMarker::Anchor, self.anchor.sat_multiply(number)) }
                    CountMarker::Value         => { self.set_marker_with_limits(CountMarker::Value, self.value.sat_multiply(number)) }
                    CountMarker::LowerBound    => { self.set_marker_with_limits(CountMarker::LowerBound, self.lower_bound.sat_multiply(number)) }
                    CountMarker::UpperBound    => { self.set_marker_with_limits(CountMarker::UpperBound, self.upper_bound.sat_multiply(number)) }
                }
            }

            Operation::Divide => {
                panic_if_zero(self.marker_name(marker), "dividing", number);
                match marker {
                    CountMarker::Anchor        => { self.set_marker_with_limits(CountMarker::Anchor, self.anchor.sat_divide(number)) }
                    CountMarker::Value         => { self.set_marker_with_limits(CountMarker::Value, self.value.sat_divide(number)) }
                    CountMarker::LowerBound    => { self.set_marker_with_limits(CountMarker::LowerBound, self.lower_bound.sat_divide(number)) }
                    CountMarker::UpperBound    => { self.set_marker_with_limits(CountMarker::UpperBound, self.upper_bound.sat_divide(number)) }
                }
            }

            Operation::Power => {
                match marker {
                    CountMarker::Anchor        => { self.set_marker_with_limits(CountMarker::Anchor, self.anchor.sat_power(number)) }
                    CountMarker::Value         => { self.set_marker_with_limits(CountMarker::Value, self.value.sat_power(number)) }
                    CountMarker::LowerBound    => { self.set_marker_with_limits(CountMarker::LowerBound, self.lower_bound.sat_power(number)) }
                    CountMarker::UpperBound    => { self.set_marker_with_limits(CountMarker::UpperBound, self.upper_bound.sat_power(number)) }
                }
            }
        }
    }

    ///
    pub fn get_whole_digit(
        &self,
        place: u8,
        marker: CountMarker,
    ) -> Option<i8> {

        // If the wanted place exists within the given CountValue type, see if the digit exists and return it.
        // If the wanted place does not exist within given CountValue type, return None.
        if (place > 0) && (place <= V::MAX_WHOLE_PLACES) {

            let scaler: V = V::from_i64(10).power_with_int((place - 1) as u32);
            let value: V = self.marker_value(marker).absolute();

            // The ones place always exists (even for a value of 0).
            // Any other place only exists if the value is large enough to reach it.
            let digit_exists: bool = (place == 1) || (value >= scaler);

            // Return the digit if it exists, otherwise return None.
            if digit_exists {
                let integer_part: V = (value / scaler).truncate();
                Some((integer_part % V::from_i64(10)).as_i8())
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    /// Retrieves a specific decimal digit from the fractional part of a marker's value, exactly
    /// as it is stored in memory — without accounting for floating-point representation noise.
    ///
    /// `place` is 1-indexed from the decimal point (`1` = tenths, `2` = hundredths, etc.).
    /// Returns `None` if `place` is `0` or exceeds `V::MAX_FLOATING_PLACES` for this type.
    pub fn get_floating_digit(
        &self,
        place: u8,
        marker: CountMarker,
    ) -> Option<i8> {

        if (place > 0) && (place <= V::MAX_FLOATING_PLACES) {

            let value: V = self.marker_value(marker).absolute();
            let whole_part: V = value.truncate();

            // Isolate just the fractional part of the value (e.g. 123.456 -> 0.456).
            let mut working: V = value - whole_part;
            let ten: V = V::from_i64(10);
            let mut digit: V = V::from_i64(0);

            // Shift the fractional part left one decimal digit at a time, extracting the
            // integer part as the digit at that place and keeping only the remainder for
            // the next iteration. This keeps `working` bounded within [0, 10) at every
            // step, regardless of how large `place` is, avoiding overflow.
            for _ in 0..place {
                working = working * ten;
                digit = working.truncate();
                working = working - digit;
            }

            Some((digit % ten).as_i8())
        }
        else {
            None
        }
    }

    ///
    pub fn get_floating_digit_with_epsilon(
        &self,
        place: u8,
        marker: CountMarker,
    ) -> Option<i8> {

        if (place > 0) && (place <= V::RELIABLE_FLOATING_PLACES) {

            let scaler: V = V::from_i64(10).power_with_int(place as u32);

            let value: V = self.marker_value(marker).absolute();
            let whole_part: V = value.truncate();
            let fractional_part: V = value - whole_part;

            let shifted: V = fractional_part * scaler;
            let floored: V = shifted.truncate();
            let remainder: V = shifted - floored;

            // EPSILON must be scaled by the same factor the fractional part was shifted by,
            // since a noise margin of EPSILON in the original value becomes a margin of
            // EPSILON * scaler once shifted into digit-extraction space.
            let epsilon_at_place: V = V::EPSILON * scaler;

            let corrected: V = if remainder >= (V::from_i64(1) - epsilon_at_place) {
                floored + V::from_i64(1)
            } else {
                floored
            };

            let digit: V = corrected % V::from_i64(10);
            Some(digit.as_i8())
        }
        else {
            None
        }
    }

    /// Returns `to_marker`'s value minus `from_marker`'s value, preserving sign to indicate direction:
    /// - **Positive Result**: `to_marker` sits to the right of (greater than) `from_marker`.
    /// - **Negative Result**: `to_marker` sits to the left of (less than) `from_marker`.
    /// - **Zero Result**: The two markers currently hold equal values.
    #[inline]
    pub fn get_signed_difference(
        &self,
        from_marker: CountMarker,
        to_marker: CountMarker,
    ) -> V::Difference {
        V::signed_difference(
            self.marker_value(from_marker),
            self.marker_value(to_marker)
        )
    }

    /// Returns the difference between the 2 passed markers -- result is always positive.
    #[inline]
    pub fn get_absolute_difference(
        &self,
        marker_1: CountMarker,
        marker_2: CountMarker,
    ) -> V::Difference {
        V::absolute_difference(
            self.marker_value(marker_1),
            self.marker_value(marker_2)
        )
    }

    /// REMEMBER TO MENTION THAT STARTING_MARKER AND ENDING_MARKER CAN BE FLIPPED TO OBTAIN THE INVERSE PERCENTAGE!
    /// MENTION THAT NONE WILL BE RETURNED IN THE CASE THAT START == END
    pub fn get_percentage(
        &self,
        marker_to_evaluate: CountMarker,
        starting_marker: CountMarker,
        ending_marker: CountMarker,
    ) -> Option<f64> {

        // Obtaining the values of the markers as f64 floats to ensure the returned percentage holds
        // the highest level of precision possible. A better alternative to this would be allowing
        // the specification of the precision, but I don't got time for that.
        let value: f64 = self.marker_value(marker_to_evaluate).as_f64();
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

    /// Returns the number that is associated with the provided percentage between the given markers.
    ///
    /// ### Panic Warning
    /// Will panic if NaN is passed for the percentage.
    pub fn get_linear_interpolation(
        &self,
        percentage: f32,
        starting_marker: CountMarker,
        ending_marker: CountMarker,
    ) -> V {

        // PANIC EVALUATION
        panic_if_is_nan("linear interpolation", "getting", percentage);

        // Using f64 for calculation to increase the precision of the result; can't specify percentage to start
        // with an f64 type since CountValue can't handle it due to supporting difference methods (f128 isn't fully
        // supported in Rust yet unfortunately).  There will be a lossy conversion for the return since the ending
        // f64 value must be returned as V, but the lossy factor is irrelevant since what is trying to be obtained
        // is a value between markers that are defined with V.
        let modified_percentage: f64 = percentage.as_f64();
        let start: f64 = self.marker_value(starting_marker).as_f64();
        let end: f64 = self.marker_value(ending_marker).as_f64();
        V::from_f64(((end - start) * modified_percentage) + start)
    }

    /// Returns true if the passed marker is at its lower limit, false otherwise.
    #[inline]
    pub fn is_at_lower_limit(
        &self,
        marker: CountMarker,
    ) -> bool {
        let value: V = self.marker_value(marker);
        match marker {
            CountMarker::Anchor |
            CountMarker::Value => {
                ((value == self.lower_bound) && self.is_lower_bound_active) ||
                (value == V::MIN)
            },
            CountMarker::LowerBound => value == V::MIN,
            CountMarker::UpperBound => value == self.lower_bound,
        }
    }

    /// Returns true if the passed marker is at its upper limit, false otherwise.
    #[inline]
    pub fn is_at_upper_limit(
        &self,
        marker: CountMarker,
    ) -> bool {
        let value: V = self.marker_value(marker);
        match marker {
            CountMarker::Anchor |
            CountMarker::Value => {
                ((value == self.upper_bound) && self.is_upper_bound_active) ||
                (value == V::MAX)
            },
            CountMarker::LowerBound => value == self.upper_bound,
            CountMarker::UpperBound => value == V::MAX,
        }
    }

    /// Returns true if the passed marker is at either its lower or upper limit, false otherwise.
    #[inline]
    pub fn is_at_a_limit(
        &self,
        marker: CountMarker,
    ) -> bool {
        let value: V = self.marker_value(marker);
        match marker {
            CountMarker::Anchor |
            CountMarker::Value => {
                ((value == self.lower_bound) && self.is_lower_bound_active) ||
                ((value == self.upper_bound) && self.is_upper_bound_active) ||
                (value == V::MIN) ||
                (value == V::MAX)
            },
            CountMarker::LowerBound => (value == V::MIN) || (value == self.upper_bound),
            CountMarker::UpperBound => (value == self.lower_bound) || (value == V::MAX),
        }
    }

    /// Returns true if the marker values are equal, false otherwise.
    #[inline]
    pub fn are_markers_equal(
        &self,
        marker_1: CountMarker,
        marker_2: CountMarker,
    ) -> bool {
        self.marker_value(marker_1) == self.marker_value(marker_2)
    }

    // #################################### HELPER METHODS ###################################### //
    /// Will print out all the fields and values of a Count, plus the Count's value minimum
    /// and maximum values for its markers.
    #[inline]
    pub fn print_information(&self) {
        println!("ANCHOR : {}", self.anchor);
        println!("VALUE : {}", self.value);
        println!("LOWER_BOUND : {}", self.lower_bound);
        println!("UPPER_BOUND : {}", self.upper_bound);
        println!("IS_LOWER_BOUND_ACTIVE : {}", self.is_lower_bound_active);
        println!("IS_UPPER_BOUND_ACTIVE : {}", self.is_upper_bound_active);
        println!("MINIMUM POTENTIAL VALUE FOR MARKERS : {}", V::MIN);
        println!("MAXIMUM POTENTIAL VALUE FOR MARKERS: {}", V::MAX);
    }

    /// Sets anchor, value, or both fields to a boundary's number if the boundary is active and
    /// anchor or value is outside the bound.  If a bound is inactive and anchor or value exceeds the
    /// bound's number, nothing will happen as anchor/value are allowed to exceed inactive bounds.
    fn enforce_bounds(&mut self) {

        match (self.is_lower_bound_active, self.is_upper_bound_active) {

            // Both bounds are active, so we clamp value and anchor into the bounded range.
            (true, true) => {
                self.value = self.value.count_clamp(self.lower_bound, self.upper_bound);
                self.anchor = self.anchor.count_clamp(self.lower_bound, self.upper_bound);
            }

            // Only the lower bound is active, so we check to see if value or anchor is below it
            // and raise them to the lower bound if they are.
            (true, false) => {
                if self.value < self.lower_bound { self.value = self.lower_bound; }
                if self.anchor < self.lower_bound { self.anchor = self.lower_bound; }
            }

            // Only the upper bound is active, so we check to see if value or anchor is above it
            // and lower them to the upper bound if they are.
            (false, true) => {
                if self.value > self.upper_bound { self.value = self.upper_bound; }
                if self.anchor > self.upper_bound { self.anchor = self.upper_bound; }
            }

            // Neither bounds are active, so bounds don't need to be enforced.
            (false, false) => {}
        }
    }

    /// Returns the name of the passed marker.
    #[inline]
    fn marker_name(
        &self,
        marker: CountMarker
    ) -> &str {
        match marker {
            CountMarker::Anchor        => { "ANCHOR" }
            CountMarker::Value         => { "VALUE" }
            CountMarker::LowerBound    => { "LOWER_BOUND" }
            CountMarker::UpperBound    => { "UPPER_BOUND" }
        }
    }

    /// Returns the current number for the passed marker.
    #[inline]
    fn marker_value(
        &self,
        marker: CountMarker
    ) -> V {
        match marker {
            CountMarker::Anchor        => { self.anchor }
            CountMarker::Value         => { self.value }
            CountMarker::LowerBound    => { self.lower_bound }
            CountMarker::UpperBound    => { self.upper_bound }
        }
    }
}


// ##################################### PANIC FUNCTIONS ######################################## //
///
#[inline]
fn panic_if_value_is_out_of_range<V: CountValue>(name_of_value: &str, name_of_action: &str, value: V, minimum: V, maximum: V) {
    assert!(
        value >= minimum && value <= maximum,
        "{}[COUNT PANIC]{} You are {name_of_action} a Count's {name_of_value} with the value {value}.  {name_of_value} must be between {minimum} and {maximum} (inclusive).",
        "\x1b[31m", "\x1b[0m",
    );
}

///
#[inline]
fn panic_if_lower_bound_is_greater_than_upper_bound<V: CountValue>(name_of_action: &str, lower_bound: V, upper_bound: V) {
    if lower_bound > upper_bound {
        panic!(
            "{}[COUNT PANIC]{} You are {name_of_action} a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your lower_bound can not be greater than your upper_bound.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

///
#[inline]
fn panic_if_upper_bound_is_less_than_lower_bound<V: CountValue>(name_of_action: &str, lower_bound: V, upper_bound: V) {
    if upper_bound < lower_bound {
        panic!(
            "{}[COUNT PANIC]{} You are {name_of_action} a Count's lower_bound with the value {lower_bound}, and its upper_bound with the value {upper_bound}; your upper_bound can not be less than your lower_bound.",
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

///
#[inline]
fn panic_if_zero<V: CountValue>(name_of_value: &str, name_of_action: &str, value: V) {
    if value == V::from_i64(0) {
        panic!(
            "{}[COUNT PANIC]{} You are {name_of_action} a Count's {name_of_value} with 0.  This will produce NaN.
            NaN is not a valid CountValue for any comparison, bound, or arithmetic operation.",
            "\x1b[31m", "\x1b[0m",
        );
    }
}

/// Will cause a panic if any of the following were to occur:
/// - anchor, value, lower_bound, or upper_bound are being set to NaN.
/// - lower_bound is being set to a value greater than upper_bound.
/// - upper_bound is being set to a value lower than lower_bound.
/// - anchor, value, lower_bound, or upper_bound are being set to values that exceed their limits.
fn panic_if_construction_is_invalid<V: CountValue>(
    anchor: V,
    value: V,
    lower_bound: V,
    upper_bound: V,
    is_lower_bound_active: bool,
    is_upper_bound_active: bool,
) {
    // Panic if a passed value for markers are NaN.
    panic_if_is_nan("anchor", "constructing", anchor);
    panic_if_is_nan("value", "constructing", value);
    panic_if_is_nan("lower_bound", "constructing", lower_bound);
    panic_if_is_nan("upper_bound", "constructing", upper_bound);

    // Panic if either boundary is being constructed with literals that don't match their definition.
    panic_if_lower_bound_is_greater_than_upper_bound("constructing", lower_bound, upper_bound);
    panic_if_upper_bound_is_less_than_lower_bound("constructing", lower_bound, upper_bound);

    // Panic if markers are being constructed with literals outside their limits.
    let active_lower_limit = if is_lower_bound_active { lower_bound } else { V::MIN };
    let active_upper_limit = if is_upper_bound_active { upper_bound } else { V::MAX };
    panic_if_value_is_out_of_range("value", "constructing", value, active_lower_limit, active_upper_limit);
    panic_if_value_is_out_of_range("anchor", "constructing", anchor, active_lower_limit, active_upper_limit);
    panic_if_value_is_out_of_range("lower_bound", "constructing", lower_bound, V::MIN, V::MAX);
    panic_if_value_is_out_of_range("upper_bound", "constructing", upper_bound, V::MIN, V::MAX);
}
