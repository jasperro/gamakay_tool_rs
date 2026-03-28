mod complex_action;
mod key_code;
mod key_combo;

use crate::utils::DynEq;

pub use self::complex_action::*;
pub use self::key_code::*;
pub use self::key_combo::*;

pub trait KeyAction: std::fmt::Debug + DynEq {
    fn to_bytes(&self) -> [u8; 4];
    fn legend(&self) -> String;
}

impl PartialEq for dyn KeyAction {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}
