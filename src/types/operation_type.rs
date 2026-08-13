
use bevy_reflect::Reflect;

// ###################################### Operation ENUM ######################################## //
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "operation_serialize", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "operation_reflect", derive(Reflect), reflect(Clone, PartialEq))]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}