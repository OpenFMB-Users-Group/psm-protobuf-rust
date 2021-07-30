/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPoint {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
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
    /// Enable real power set point
    #[prost(message, optional, tag="7")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// ESS state
    #[prost(message, optional, tag="9")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="10")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// X-axis value (Unix time).
    #[prost(message, optional, tag="11")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<SolarPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarControlScheduleFsch {
    /// Discrete value in SolarCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<SolarCsg>,
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_control_schedule_fsch: ::core::option::Option<SolarControlScheduleFsch>,
}
/// Solar control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_control_fscc: ::core::option::Option<SolarControlFscc>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarInverter {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Solar control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_control: ::core::option::Option<SolarControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPointStatus {
    /// Enable frequency set point
    #[prost(message, optional, tag="1")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
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
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarEventAndStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="2")]
    pub aux_pwr_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="4")]
    pub emg_stop: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub point_status: ::core::option::Option<SolarPointStatus>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub solar_event_and_status_zgen: ::core::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub gri_mod: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
/// Solar event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_event_zgen: ::core::option::Option<SolarEventZgen>,
}
/// Solar event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_event: ::core::option::Option<SolarEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
}
/// Solar reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarReading {
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
/// Solar reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_reading: ::core::option::Option<SolarReading>,
}
/// Specialized 61850 ZGEN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub solar_event_and_status_zgen: ::core::option::Option<SolarEventAndStatusZgen>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub gri_mod: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
}
/// Solar status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_status_zgen: ::core::option::Option<SolarStatusZgen>,
}
/// Solar status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_status: ::core::option::Option<SolarStatus>,
}
