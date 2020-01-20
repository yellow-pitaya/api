pub mod analog;
pub mod digital;

/**
 * Type representing pin's high or low state (on/off).
 */
pub use crate::rp::rp_pinState_t as State;

/**
 * Type representing pin's input or output direction.
 */
pub use crate::rp::rp_pinDirection_t as Direction;
