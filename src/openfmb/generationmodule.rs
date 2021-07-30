/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    pub pct_hz_droop: ::core::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_v_droop: ::core::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// ESS state
    #[prost(message, optional, tag="9")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="10")]
    pub sync_back_to_grid: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="11")]
    pub trans_to_islnd_on_grid_loss_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="12")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Start time
    #[prost(message, optional, tag="13")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<GenerationPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationControlScheduleFsch {
    /// Discrete value in GenerationCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<GenerationCsg>,
}
/// LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_control_schedule_fsch: ::core::option::Option<GenerationControlScheduleFsch>,
}
/// Generation control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_control_fscc: ::core::option::Option<GenerationControlFscc>,
}
/// A single or set of synchronous machines for converting mechanical power into alternating-current
/// power. For example, individual machines within a set may be defined for scheduling purposes while a
/// single control signal is derived for the set. In this case there would be a GeneratingUnit for each
/// member of the set and an additional GeneratingUnit corresponding to the set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeneratingUnit {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// This is the maximum operating active power limit the dispatcher can enter for this unit.
    #[prost(message, optional, tag="2")]
    pub max_operating_p: ::core::option::Option<super::commonmodule::ActivePower>,
}
/// Generation control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::core::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_control: ::core::option::Option<GenerationControl>,
}
/// Generation discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DroopParameter {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub slope: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub unloaded_offset: ::core::option::Option<f32>,
}
/// Generation real power control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealPowerControl {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub droop_setpoint: ::core::option::Option<DroopParameter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub isochronous_setpoint: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub real_power_control_mode: ::core::option::Option<super::commonmodule::OptionalRealPowerControlKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub real_power_setpoint: ::core::option::Option<f32>,
}
/// Generation real power control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactivePowerControl {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub droop_setpoint: ::core::option::Option<DroopParameter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub power_factor_setpoint: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reactive_power_control_mode: ::core::option::Option<super::commonmodule::OptionalReactivePowerControlKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reactive_power_setpoint: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub voltage_setpoint: ::core::option::Option<f32>,
}
/// Generation discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reactive_power_control: ::core::option::Option<ReactivePowerControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub real_power_control: ::core::option::Option<RealPowerControl>,
}
/// Generation discrete control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::core::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_discrete_control: ::core::option::Option<GenerationDiscreteControl>,
}
/// Generation reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::core::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub phase_mmtn: ::core::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::core::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmxu: ::core::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Generation reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::core::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_reading: ::core::option::Option<GenerationReading>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationPointStatus {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Black start enable
    #[prost(message, optional, tag="3")]
    pub pct_hz_droop: ::core::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_v_droop: ::core::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="6")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// ESS state
    #[prost(message, optional, tag="8")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="9")]
    pub sync_back_to_grid: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="10")]
    pub trans_to_islnd_on_grid_loss_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable voltage set point
    #[prost(message, optional, tag="11")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationEventAndStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub aux_pwr_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    pub emg_stop: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    pub gn_syn_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="6")]
    pub point_status: ::core::option::Option<GenerationPointStatus>,
}
/// Specialized generation event ZGEN
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::core::option::Option<GenerationEventAndStatusZgen>,
}
/// Generation event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_event_zgen: ::core::option::Option<GenerationEventZgen>,
}
/// Generation event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::core::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_event: ::core::option::Option<GenerationEvent>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub generation_event_and_status_zgen: ::core::option::Option<GenerationEventAndStatusZgen>,
}
/// Generation status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generation_status_zgen: ::core::option::Option<GenerationStatusZgen>,
}
/// Generation status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerationStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub generating_unit: ::core::option::Option<GeneratingUnit>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub generation_status: ::core::option::Option<GenerationStatus>,
}
