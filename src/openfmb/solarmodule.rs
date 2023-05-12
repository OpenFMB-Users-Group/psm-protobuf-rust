/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarInverter {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapabilityConfiguration {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_configuration: ::core::option::Option<super::commonmodule::SourceCapabilityConfiguration>,
}
/// Generation capability
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapabilityOverride {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_capability_configuration: ::core::option::Option<SolarCapabilityConfiguration>,
}
/// Generation capability profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapabilityOverrideProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_capability_override: ::core::option::Option<SolarCapabilityOverride>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapabilityRatings {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_ratings: ::core::option::Option<super::commonmodule::SourceCapabilityRatings>,
}
/// Generation capability
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapability {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub nameplate_value: ::core::option::Option<super::commonmodule::NameplateValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_capability_configuration: ::core::option::Option<SolarCapabilityConfiguration>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_capability_ratings: ::core::option::Option<SolarCapabilityRatings>,
}
/// Generation capability profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCapabilityProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_capability: ::core::option::Option<SolarCapability>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_inverter: ::core::option::Option<SolarInverter>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarPoint {
    /// Grid connect mode
    #[prost(message, optional, tag="2")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Ramp rates
    #[prost(message, optional, tag="5")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Reset device
    #[prost(message, optional, tag="8")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="9")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="12")]
    pub enter_service_operation: ::core::option::Option<super::commonmodule::EnterServiceApc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="13")]
    pub hz_w_operation: ::core::option::Option<super::commonmodule::HzWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="14")]
    pub limit_w_operation: ::core::option::Option<super::commonmodule::LimitWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="15")]
    pub p_f_operation: ::core::option::Option<super::commonmodule::Pfspc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="16")]
    pub tm_hz_trip_operation: ::core::option::Option<super::commonmodule::TmHzCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="17")]
    pub tm_volt_trip_operation: ::core::option::Option<super::commonmodule::TmVoltCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="18")]
    pub v_ar_operation: ::core::option::Option<super::commonmodule::VarSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="19")]
    pub volt_var_operation: ::core::option::Option<super::commonmodule::VoltVarCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="20")]
    pub volt_w_operation: ::core::option::Option<super::commonmodule::VoltWcsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="21")]
    pub w_var_operation: ::core::option::Option<super::commonmodule::WVarCsg>,
    /// Black start enable
    #[prost(message, optional, tag="22")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="24")]
    pub w_operation: ::core::option::Option<super::commonmodule::Wspc>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCurvePoint {
    /// The array with the points specifying a curve shape.
    #[prost(message, optional, tag="1")]
    pub control: ::core::option::Option<SolarPoint>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<SolarCurvePoint>,
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
/// OpenFMB specialization for cap bank discrete control:
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarDiscreteControlPv {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control: ::core::option::Option<SolarPoint>,
}
/// Cap bank discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub solar_discrete_control_pv: ::core::option::Option<SolarDiscreteControlPv>,
}
/// Cap bank discrete control profile.  Instructs an end device (or an end device group) to perform
/// a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolarDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub solar_discrete_control: ::core::option::Option<SolarDiscreteControl>,
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pct_hz_droop: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="10")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="11")]
    pub enter_service_operation: ::core::option::Option<super::commonmodule::EnterServiceApc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="12")]
    pub hz_w_operation: ::core::option::Option<super::commonmodule::HzWPoint>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="13")]
    pub limit_w_operation: ::core::option::Option<super::commonmodule::LimitWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="14")]
    pub p_f_operation: ::core::option::Option<super::commonmodule::Pfspc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="15")]
    pub sync_back_to_grid: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="16")]
    pub tm_hz_trip_operation: ::core::option::Option<super::commonmodule::TmHzCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="17")]
    pub tm_volt_trip_operation: ::core::option::Option<super::commonmodule::TmVoltCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="18")]
    pub v_ar_operation: ::core::option::Option<super::commonmodule::VarSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="19")]
    pub volt_var_operation: ::core::option::Option<super::commonmodule::VoltVarCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="20")]
    pub volt_w_operation: ::core::option::Option<super::commonmodule::VoltWcsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="21")]
    pub w_var_operation: ::core::option::Option<super::commonmodule::WVarCsg>,
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
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="6")]
    pub alrm: ::core::option::Option<super::commonmodule::OptionalAlrmKind>,
    /// Emergency stop
    #[prost(message, optional, tag="7")]
    pub gn_syn_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="8")]
    pub grid_connection_state: ::core::option::Option<super::commonmodule::OptionalGridConnectionStateKind>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="9")]
    pub man_alrm_info: ::core::option::Option<::prost::alloc::string::String>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="10")]
    pub operating_state: ::core::option::Option<super::commonmodule::OptionalOperatingStateKind>,
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
