pub mod analog;
pub mod digital;

/**
 * Type representing pin's high or low state (on/off).
 */
pub use rp_sys::rp_pinState_t as State;

/**
 * Type representing pin's input or output direction.
 */
pub use rp_sys::rp_pinDirection_t as Direction;
