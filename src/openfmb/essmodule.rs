/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapabilityConfiguration {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_configuration: ::core::option::Option<super::commonmodule::SourceCapabilityConfiguration>,
    /// Apparent Power Charge Maximum
    #[prost(message, optional, tag="2")]
    pub va_cha_rte_max: ::core::option::Option<super::commonmodule::Asg>,
    /// Apparent Power Discharge Maximum
    #[prost(message, optional, tag="3")]
    pub va_dis_cha_rte_max: ::core::option::Option<super::commonmodule::Asg>,
    /// Active Power Charge Maximum
    #[prost(message, optional, tag="4")]
    pub w_cha_rte_max: ::core::option::Option<super::commonmodule::Asg>,
    /// Active Power Discharge Maximum
    #[prost(message, optional, tag="5")]
    pub w_dis_cha_rte_max: ::core::option::Option<super::commonmodule::Asg>,
}
/// Generation capability
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapabilityOverride {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub nameplate_value: ::core::option::Option<super::commonmodule::NameplateValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_capability_configuration: ::core::option::Option<EssCapabilityConfiguration>,
}
/// ESS control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapabilityOverrideProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_capability_override: ::core::option::Option<EssCapabilityOverride>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapabilityRatings {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_ratings: ::core::option::Option<super::commonmodule::SourceCapabilityRatings>,
    /// Apparent Power Charge Maximum Rating
    #[prost(message, optional, tag="2")]
    pub va_cha_rte_max_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// Apparent Power Discharge Maximum Rating
    #[prost(message, optional, tag="3")]
    pub va_dis_cha_rte_max_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// Active Power Charge Maximum Rating
    #[prost(message, optional, tag="4")]
    pub w_cha_rte_max_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// Active Power Discharge Maximum Rating
    #[prost(message, optional, tag="5")]
    pub w_dis_cha_rte_max_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// Energy rating of the DER storage.
    #[prost(message, optional, tag="6")]
    pub wh_rtg: ::core::option::Option<super::commonmodule::Asg>,
}
/// Generation capability
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapability {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub nameplate_value: ::core::option::Option<super::commonmodule::NameplateValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_capability_ratings: ::core::option::Option<EssCapabilityRatings>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_capability_configuration: ::core::option::Option<EssCapabilityConfiguration>,
}
/// ESS control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCapabilityProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_capability: ::core::option::Option<EssCapability>,
}
/// ESS inverter high level functions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssFunction {
    /// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
    #[prost(message, optional, tag="1")]
    pub capacity_firming: ::core::option::Option<super::commonmodule::CapacityFirming>,
    /// ESS inverter high level function to maintain frequency within dead bands.
    #[prost(message, optional, tag="2")]
    pub frequency_regulation: ::core::option::Option<super::commonmodule::FrequencyRegulation>,
    /// ESS inverter high level function to maintain power level by charging or discharging
    #[prost(message, optional, tag="3")]
    pub peak_shaving: ::core::option::Option<super::commonmodule::PeakShaving>,
    /// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
    #[prost(message, optional, tag="4")]
    pub soc_limit: ::core::option::Option<super::commonmodule::SocLimit>,
    /// ESS inverter high level function to maintain SOC within dead bands
    #[prost(message, optional, tag="5")]
    pub soc_management: ::core::option::Option<super::commonmodule::SocManagement>,
    /// ESS inverter high level function to maintain voltage within droop dead bands.
    #[prost(message, optional, tag="6")]
    pub voltage_droop: ::core::option::Option<super::commonmodule::VoltageDroop>,
    /// ESS inverter high level function to maintain voltage within dead bands.
    #[prost(message, optional, tag="7")]
    pub voltage_pi: ::core::option::Option<super::commonmodule::VoltagePi>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::core::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Reset device
    #[prost(message, optional, tag="10")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// ESS state
    #[prost(message, optional, tag="11")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="13")]
    pub trans_to_islnd_on_grid_loss_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="16")]
    pub enter_service_operation: ::core::option::Option<super::commonmodule::EnterServiceApc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="17")]
    pub hz_w_operation: ::core::option::Option<super::commonmodule::HzWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="18")]
    pub limit_w_operation: ::core::option::Option<super::commonmodule::LimitWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="19")]
    pub p_f_operation: ::core::option::Option<super::commonmodule::Pfspc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="20")]
    pub tm_hz_trip_operation: ::core::option::Option<super::commonmodule::TmHzCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="21")]
    pub tm_volt_trip_operation: ::core::option::Option<super::commonmodule::TmVoltCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="22")]
    pub v_ar_operation: ::core::option::Option<super::commonmodule::VarSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="23")]
    pub volt_var_operation: ::core::option::Option<super::commonmodule::VoltVarCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="24")]
    pub volt_w_operation: ::core::option::Option<super::commonmodule::VoltWcsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="25")]
    pub w_var_operation: ::core::option::Option<super::commonmodule::WVarCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="26")]
    pub w_operation: ::core::option::Option<super::commonmodule::Wspc>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssCurvePoint {
    /// The array with the points specifying a curve shape.
    #[prost(message, optional, tag="1")]
    pub control: ::core::option::Option<EssPoint>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Esscsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<EssCurvePoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssControlScheduleFsch {
    /// Discrete value in ESSCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<Esscsg>,
}
/// Specialized 61850 FSCC class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_control_schedule_fsch: ::core::option::Option<EssControlScheduleFsch>,
}
/// ESS control class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control_fscc: ::core::option::Option<EssControlFscc>,
}
/// ESS control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_control: ::core::option::Option<EssControl>,
}
/// OpenFMB specialization for cap bank discrete control:
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssDiscreteControlDbat {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control: ::core::option::Option<EssPoint>,
}
/// Cap bank discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_discrete_control_dbat: ::core::option::Option<EssDiscreteControlDbat>,
}
/// Cap bank discrete control profile.  Instructs an end device (or an end device group) to perform
/// a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_discrete_control: ::core::option::Option<EssDiscreteControl>,
}
/// Specialized 61850 ZBAT class  LN: Battery   Name: ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssEventZbat {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// If true, the battery is in overcharge (voltage or current) condition.
    #[prost(message, optional, tag="2")]
    pub bat_hi: ::core::option::Option<super::commonmodule::StatusSps>,
    /// If true, the battery voltage or charge has dropped below a pre-set level.
    #[prost(message, optional, tag="3")]
    pub bat_lo: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub bat_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="5")]
    pub soc: ::core::option::Option<super::commonmodule::Mv>,
    /// If stVal TRUE, the device is in standby.
    #[prost(message, optional, tag="6")]
    pub stdby: ::core::option::Option<super::commonmodule::StatusSps>,
    /// State of health of the DER storage.
    #[prost(message, optional, tag="7")]
    pub so_h: ::core::option::Option<super::commonmodule::Mv>,
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    #[prost(message, optional, tag="8")]
    pub wh_avail: ::core::option::Option<super::commonmodule::Mv>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssPointStatus {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// ESS function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::core::option::Option<EssFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub pct_hz_droop: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub pct_v_droop: ::core::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="8")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable real power set point
    #[prost(message, optional, tag="9")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// ESS state
    #[prost(message, optional, tag="10")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Synchronize back to grid
    #[prost(message, optional, tag="11")]
    pub sync_back_to_grid: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Transition to island on grid loss enable
    #[prost(message, optional, tag="12")]
    pub trans_to_islnd_on_grid_loss_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable voltage set point
    #[prost(message, optional, tag="13")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="14")]
    pub enter_service_operation: ::core::option::Option<super::commonmodule::EnterServiceApc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="15")]
    pub hz_w_operation: ::core::option::Option<super::commonmodule::HzWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="16")]
    pub limit_w_operation: ::core::option::Option<super::commonmodule::LimitWapc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="17")]
    pub p_f_operation: ::core::option::Option<super::commonmodule::Pfspc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="18")]
    pub tm_hz_trip_operation: ::core::option::Option<super::commonmodule::TmHzCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="19")]
    pub tm_volt_trip_operation: ::core::option::Option<super::commonmodule::TmVoltCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="20")]
    pub v_ar_operation: ::core::option::Option<super::commonmodule::VarSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="21")]
    pub volt_var_operation: ::core::option::Option<super::commonmodule::VoltVarCsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="22")]
    pub volt_w_operation: ::core::option::Option<super::commonmodule::VoltWcsg>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="23")]
    pub w_var_operation: ::core::option::Option<super::commonmodule::WVarCsg>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssEventAndStatusZgen {
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
    /// Generator is synchronized to EPS, or not; True = Synchronized
    #[prost(message, optional, tag="5")]
    pub gn_syn_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// DC Power On/Off Status; True = DC power on
    #[prost(message, optional, tag="6")]
    pub alrm: ::core::option::Option<super::commonmodule::OptionalAlrmKind>,
    /// Point status
    #[prost(message, optional, tag="7")]
    pub point_status: ::core::option::Option<EssPointStatus>,
}
/// Specialized 61850 ZGEN class for ESS event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssEventZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::core::option::Option<EssEventAndStatusZgen>,
}
/// ESS event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_event_zbat: ::core::option::Option<EssEventZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_event_zgen: ::core::option::Option<EssEventZgen>,
}
/// ESS event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_event: ::core::option::Option<EssEvent>,
}
/// ESS reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssReading {
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub reading_mmdc: ::core::option::Option<super::commonmodule::ReadingMmdc>,
}
/// ESS reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_reading: ::core::option::Option<EssReading>,
}
/// Specialized 61850 ZBAT
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssStatusZbat {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Battery system status &ndash; True: on
    #[prost(message, optional, tag="2")]
    pub bat_st: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub gri_mod: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// State of charge (in percentage)
    #[prost(message, optional, tag="4")]
    pub soc: ::core::option::Option<super::commonmodule::Mv>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub stdby: ::core::option::Option<super::commonmodule::StatusSps>,
    /// State of health of the DER storage.
    #[prost(message, optional, tag="6")]
    pub so_h: ::core::option::Option<super::commonmodule::Mv>,
    /// Energy available of the DER storage (WHAvail = WHRtg * SoC * SoH)
    #[prost(message, optional, tag="7")]
    pub wh_avail: ::core::option::Option<super::commonmodule::Mv>,
}
/// Specialized 61850 ZGEN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssStatusZgen {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub e_ss_event_and_status_zgen: ::core::option::Option<EssEventAndStatusZgen>,
}
/// ESS status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess_status_zbat: ::core::option::Option<EssStatusZbat>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_status_zgen: ::core::option::Option<EssStatusZgen>,
}
/// ESS status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EssStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub ess: ::core::option::Option<super::commonmodule::Ess>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub ess_status: ::core::option::Option<EssStatus>,
}
