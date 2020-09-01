/**
 * Type representing different trigger sources used at acquiring signal.
 */
pub use crate::rp::rp_acq_trig_src_t as Source;

/**
 * Type representing different trigger states.
 */
pub use crate::rp::rp_acq_trig_state_t as State;

/**
 * Type representing Input/Output channels in trigger.
 */
pub use crate::rp::rp_channel_trigger_t as Channel;

/**
 * Sets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn set_source(source: Source) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerSrc(source)
    )
}

/**
 * Gets the trigger source used at acquiring signal.
 *
 * When acquiring is started, the FPGA waits for the trigger condition on the
 * specified source and when the condition is met, it starts writing the signal
 * to the buffer.
 */
pub fn source() -> crate::Result<Source>
{
    let mut source = Source::RP_TRIG_SRC_DISABLED;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerSrc(&mut source)
    ) {
        Ok(_) => Ok(source),
        Err(err) => Err(err),
    }
}

/**
 * Returns the trigger state.
 *
 * Either it is waiting for a trigger to happen, or it has already been
 * triggered.
 *
 * By default it is in the triggered state, which is treated the same as
 * disabled.
 */
pub fn state() -> crate::Result<State>
{
    let mut state = State::RP_TRIG_STATE_TRIGGERED;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerState(&mut state)
    ) {
        Ok(_) => Ok(state),
        Err(err) => Err(err),
    }
}

/**
 * Sets the number of decimated data after trigger written into memory.
 */
pub fn set_delay(decimated_data_num: i32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerDelay(decimated_data_num)
    )
}

/**
 * Returns current number of decimated data after trigger written into memory.
 */
pub fn delay() -> crate::Result<i32>
{
    let mut decimated_data_num = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerDelay(&mut decimated_data_num)
    ) {
        Ok(_) => Ok(decimated_data_num),
        Err(err) => Err(err),
    }
}

/**
 * Sets the amount of decimated data in nanoseconds after trigger written into
 * memory.
 */
pub fn set_delay_ns(time_ns: i64) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerDelayNs(time_ns)
    )
}

/**
 * Returns the current amount of decimated data in nanoseconds after trigger
 * written into memory.
 */
pub fn delay_ns() -> crate::Result<i64>
{
    let mut time_ns = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerDelayNs(&mut time_ns)
    ) {
        Ok(_) => Ok(time_ns),
        Err(err) => Err(err),
    }
}

/**
 * Returns the number of valid data ponts before trigger.
 */
pub fn pre_counter() -> crate::Result<u32>
{
    let mut value = 0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetPreTriggerCounter(&mut value)
    ) {
        Ok(_) => Ok(value),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold value in volts.
 *
 * Makes the trigger when ADC value crosses this value.
 */
pub fn set_level(channel: Channel, volatage: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerLevel(channel, volatage)
    )
}

/**
 * Gets currently set trigger threshold value in volts.
 */
pub fn level(channel: Channel) -> crate::Result<f32>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerLevel(channel, &mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}

/**
 * Sets the trigger threshold hysteresis value in volts.
 *
 * Value must be outside to enable the trigger again.
 */
pub fn set_hysteresis(volatage: f32) -> crate::Result<()>
{
    handle_unsafe!(
        crate::rp::rp_AcqSetTriggerHyst(volatage)
    )
}

/**
 * Gets currently set trigger threshold hysteresis value in volts.
 */
pub fn hysteresis() -> crate::Result<f32>
{
    let mut volatage = 0.0;

    match handle_unsafe!(
        crate::rp::rp_AcqGetTriggerHyst(&mut volatage)
    ) {
        Ok(_) => Ok(volatage),
        Err(err) => Err(err),
    }
}
