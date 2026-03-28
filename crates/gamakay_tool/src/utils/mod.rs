use std::any::Any;

pub mod actions;
pub mod layout;

pub trait DynEq: Any {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn DynEq) -> bool;
}

impl<T: PartialEq + Any> DynEq for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn DynEq) -> bool {
        // First check if the other object is the same concrete type
        if let Some(other_concrete) = other.as_any().downcast_ref::<Self>() {
            // If they match, use the concrete PartialEq implementation
            self == other_concrete
        } else {
            // If types differ, they are not equal
            false
        }
    }
}
