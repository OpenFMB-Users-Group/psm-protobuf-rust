/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityConfigurationDeao {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// The rated maximum AC charging current of the outlet
    #[prost(message, optional, tag="3")]
    pub cha_a_rtg: ::core::option::Option<super::commonmodule::ControlIng>,
    /// Available AC current (6-80A) to signal to the EV when not using digital communication
    #[prost(message, optional, tag="4")]
    pub cha_a_max: ::core::option::Option<super::commonmodule::ControlIng>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityConfigurationDedo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityConfigurationDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_configuration: ::core::option::Option<super::commonmodule::SourceCapabilityConfiguration>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub capability_configuration_dea0: ::core::option::Option<CapabilityConfigurationDeao>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub capability_configuration_dedo: ::core::option::Option<CapabilityConfigurationDedo>,
    /// Rated maximum charging power of the EVSE
    #[prost(message, optional, tag="4")]
    pub cha_pwr_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// The power value that the EVSE requires to grid
    #[prost(message, optional, tag="5")]
    pub cha_pwr_tgt: ::core::option::Option<super::commonmodule::Asg>,
    /// The power value that the grid limits to the charger
    #[prost(message, optional, tag="6")]
    pub cha_pwr_lim: ::core::option::Option<super::commonmodule::Asg>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseCapabilityOverride {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub nameplate_value: ::core::option::Option<super::commonmodule::NameplateValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub capability_configuration_dese: ::prost::alloc::vec::Vec<CapabilityConfigurationDese>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evse {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// EVSE Capability Override Profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseCapabilityOverrideProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_capability_override: ::core::option::Option<EvseCapabilityOverride>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRatingsZcab {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// Rated current of the cable in A
    #[prost(message, optional, tag="2")]
    pub a_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// Maximum overload of the cable in percent
    #[prost(message, optional, tag="3")]
    pub ovl_max_pct: ::core::option::Option<super::commonmodule::Asg>,
    /// Maximum time allowed for corresponding overload set as Tmm
    #[prost(message, optional, tag="4")]
    pub ovl_max_tm: ::core::option::Option<super::commonmodule::ControlIng>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRatingsDeao {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// The rated maximum AC charging current of the outlet
    #[prost(message, optional, tag="3")]
    pub cha_a_rtg: ::core::option::Option<super::commonmodule::ControlIng>,
    /// Available AC current (6-80A) to signal to the EV when not using digital communication
    #[prost(message, optional, tag="4")]
    pub cha_a_max: ::core::option::Option<super::commonmodule::ControlIng>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRatingsDedo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityRatingsDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub source_capability_ratings: ::core::option::Option<super::commonmodule::SourceCapabilityRatings>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub capability_ratings_deao: ::prost::alloc::vec::Vec<CapabilityRatingsDeao>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub capability_ratings_dedo: ::prost::alloc::vec::Vec<CapabilityRatingsDedo>,
    /// Rated maximum charging power of the EVSE
    #[prost(message, optional, tag="4")]
    pub cha_pwr_rtg: ::core::option::Option<super::commonmodule::Asg>,
    /// The power value that the EVSE requires to grid
    #[prost(message, optional, tag="5")]
    pub cha_pwr_tgt: ::core::option::Option<super::commonmodule::Asg>,
    /// The power value that the grid limits to the charger
    #[prost(message, optional, tag="6")]
    pub cha_pwr_lim: ::core::option::Option<super::commonmodule::Asg>,
    /// True = DC charging is supported
    #[prost(message, optional, tag="7")]
    pub conn_typ_dc: ::core::option::Option<super::commonmodule::Spg>,
    /// True = AC 1 phase charging is supported. Use ConnTypPhs1 for one phase charging, ConnTypPhs2 for
    /// two phase charging and ConnTypPhs3 for three phase charging.
    #[prost(message, optional, tag="8")]
    pub conn_typ_phs1: ::core::option::Option<super::commonmodule::Spg>,
    /// True = AC 2 phase charging is supported.
    #[prost(message, optional, tag="9")]
    pub conn_typ_phs2: ::core::option::Option<super::commonmodule::Spg>,
    /// True = AC 3 phase charging is supported.
    #[prost(message, optional, tag="10")]
    pub conn_typ_phs3: ::core::option::Option<super::commonmodule::Spg>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseCapability {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub nameplate_value: ::core::option::Option<super::commonmodule::NameplateValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub capability_ratings_dese: ::prost::alloc::vec::Vec<CapabilityRatingsDese>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub capability_configuration_dese: ::prost::alloc::vec::Vec<CapabilityConfigurationDese>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub capability_ratings_zcab: ::core::option::Option<CapabilityRatingsZcab>,
}
/// EVSE Capability Profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseCapabilityProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub capability_message_info: ::core::option::Option<super::commonmodule::CapabilityMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_capability: ::core::option::Option<EvseCapability>,
}
/// EVSE inverter high level functions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseFunction {
    /// EVSE inverter high level function to reduce (smooth) charging or discharging rate of change.
    #[prost(message, optional, tag="1")]
    pub capacity_firming: ::core::option::Option<super::commonmodule::CapacityFirming>,
    /// EVSE inverter high level function to maintain frequency within dead bands.
    #[prost(message, optional, tag="2")]
    pub frequency_regulation: ::core::option::Option<super::commonmodule::FrequencyRegulation>,
    /// EVSE inverter high level function to maintain power level by charging or discharging
    #[prost(message, optional, tag="3")]
    pub peak_shaving: ::core::option::Option<super::commonmodule::PeakShaving>,
    /// EVSE inverter high level function to shut down ESS if SOC exceeds high or low limits.
    #[prost(message, optional, tag="4")]
    pub soc_limit: ::core::option::Option<super::commonmodule::SocLimit>,
    /// EVSE inverter high level function to maintain SOC within dead bands
    #[prost(message, optional, tag="5")]
    pub soc_management: ::core::option::Option<super::commonmodule::SocManagement>,
    /// EVSE inverter high level function to maintain voltage within droop dead bands.
    #[prost(message, optional, tag="6")]
    pub voltage_droop: ::core::option::Option<super::commonmodule::VoltageDroop>,
    /// EVSE inverter high level function to maintain voltage within dead bands.
    #[prost(message, optional, tag="7")]
    pub voltage_pi: ::core::option::Option<super::commonmodule::VoltagePi>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalChargingStateKind {
    #[prost(enumeration="ChargingStateKind", tag="1")]
    pub value: i32,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvsePoint {
    /// EVSE function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::core::option::Option<EvseFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Reset device
    #[prost(message, optional, tag="10")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// EV state
    #[prost(message, optional, tag="11")]
    pub state: ::core::option::Option<OptionalChargingStateKind>,
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
pub struct EvseCurvePoint {
    /// The array with the points specifying a curve shape.
    #[prost(message, optional, tag="1")]
    pub control: ::core::option::Option<EvsePoint>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
    /// Duration of the charging schedule in seconds.
    #[prost(uint64, tag="3")]
    pub duration: u64,
    /// Value determining level in hierarchy stack of profiles. Higher values have precedence over lower
    /// values. Lowest level is 0.
    #[prost(uint64, tag="4")]
    pub stack_level: u64,
    /// Identifies the Charging Schedule.
    #[prost(uint64, tag="5")]
    pub id: u64,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Evsecsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<EvseCurvePoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeseControlScheduleFsch {
    /// Discrete value in EVSECSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<Evsecsg>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeevControlScheduleFsch {
    /// Analog CSG
    #[prost(message, optional, tag="1")]
    pub val_acsg: ::core::option::Option<super::commonmodule::ScheduleCsg>,
}
/// LN: E-Mobility Electric Vehicle Name: DEEV
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlDeev {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub deev_control_schedule_fsch: ::core::option::Option<DeevControlScheduleFsch>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlDeao {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control_deev: ::core::option::Option<ControlDeev>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlDedo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control_deev: ::core::option::Option<ControlDeev>,
}
/// Specialized 61850 DESE class.  LN: Controlling the features of an EVSE   Name: DESE
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dese_control_schedule_fsch: ::core::option::Option<DeseControlScheduleFsch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub control_deao: ::core::option::Option<ControlDeao>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub control_deeo: ::core::option::Option<ControlDedo>,
    /// maximum charging current that the charger can provide to the EV
    #[prost(message, optional, tag="5")]
    pub cha_a_max: ::core::option::Option<f32>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub control_dese: ::prost::alloc::vec::Vec<ControlDese>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_control: ::core::option::Option<EvseControl>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscreteControlDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub control: ::core::option::Option<EvsePoint>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub discrete_control_dese: ::prost::alloc::vec::Vec<DiscreteControlDese>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_discrete_control: ::core::option::Option<EvseDiscreteControl>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvsePointStatus {
    /// EVSE function parameter
    #[prost(message, optional, tag="3")]
    pub function: ::core::option::Option<EvseFunction>,
    /// Grid connect mode
    #[prost(message, optional, tag="4")]
    pub mode: ::core::option::Option<super::commonmodule::EngGridConnectModeKind>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Reset device
    #[prost(message, optional, tag="10")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// EV state
    #[prost(message, optional, tag="11")]
    pub state: ::core::option::Option<OptionalChargingStateKind>,
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
    /// charge an EV when plugged in with no restrictions.
    #[prost(message, optional, tag="27")]
    pub no_restric_enable: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="28")]
    pub charge_control_enable: ::core::option::Option<super::commonmodule::StatusSps>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvacConnectionStateKind {
    #[prost(enumeration="EvacConnectionStateKind", tag="1")]
    pub value: i32,
}
/// Connection states of electrical vehicles (notation from IEC 61851-1).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvacConnectionStateKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvacConnectionStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvacPlugStateKind {
    #[prost(enumeration="EvacPlugStateKind", tag="1")]
    pub value: i32,
}
/// Plug present and coupler lock states (according to IEC 61851-1)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvacPlugStateKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvacPlugStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvacCableCapabilityKind {
    #[prost(enumeration="EvacCableCapabilityKind", tag="1")]
    pub value: i32,
}
/// <b>EV AC Charging Cable Capability</b>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvacCableCapabilityKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvacCableCapabilityKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvConnectionChargingKind {
    #[prost(enumeration="EvConnectionChargingKind", tag="1")]
    pub value: i32,
}
/// This enumeration lists the connection charging types (according to IEC 61851-1). Used in logical
/// node DEEV.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvConnectionChargingKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvConnectionChargingKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
/// Specialized 61850 DEEV class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAndStatusDeev {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// In ISO 15118 compliant implementations, the EVId refers to the EVCCID Identifier as defined in
    /// [ISO 15118-2:2014]  Specifies the EV’s identification in a readable format. It contains the MAC
    /// address of the EVCC as six hexBinary encoded bytes, i.e. the element shall have a length of six
    /// bytes.
    #[prost(message, optional, tag="2")]
    pub ev_id: ::core::option::Option<::prost::alloc::string::String>,
    /// In ISO 15118-2:2014 compliant implementations, EMobility Account Identifier as defined in Annex
    /// H.1 of [ISO 15118-2:2014] This element identifies the charging contract.
    #[prost(message, optional, tag="3")]
    pub ema_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Selected connection type according to 61851-1
    #[prost(message, optional, tag="4")]
    pub conn_typ_sel: ::core::option::Option<EnsEvConnectionChargingKind>,
    /// Departure time is used to indicate when the vehicle intends to finish the charging process. A
    /// value of zero (0) indicates that the charging process shall be finished as fast as possible.
    #[prost(message, optional, tag="5")]
    pub dpt_tm: ::core::option::Option<super::commonmodule::ClearingTime>,
    /// Amount of energy required by the EV until the departure time has been reached or the EV
    /// battery's SOC is at 100%. This might include the amount of energy the EV consumes for other vehicle
    /// features than solely charging the EV battery.
    #[prost(message, optional, tag="6")]
    pub wh_req: ::core::option::Option<super::commonmodule::Mv>,
    /// Energy available of the EV
    #[prost(message, optional, tag="7")]
    pub wh_avail: ::core::option::Option<super::commonmodule::Mv>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub soc: ::core::option::Option<super::commonmodule::Mv>,
}
/// Specialized 61850 DEAO class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAndStatusDeao {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub event_and_status_deev: ::core::option::Option<EventAndStatusDeev>,
    /// Enable digital communication with the EV
    #[prost(message, optional, tag="3")]
    pub dig_comm: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Connection state (notation from IEC 61851-1)
    #[prost(message, optional, tag="4")]
    pub conn_st: ::core::option::Option<EnsEvacConnectionStateKind>,
    /// Plug present and coupler lock state (according to 61851-1)
    #[prost(message, optional, tag="5")]
    pub plg_st_ac: ::core::option::Option<EnsEvacPlugStateKind>,
    /// Capability of the EV cable assembly (according to 61851-1)
    #[prost(message, optional, tag="6")]
    pub cab_rtg_ac: ::core::option::Option<EnsEvacCableCapabilityKind>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvdcCableCapabilityKind {
    #[prost(enumeration="EvdcCableCapabilityKind", tag="1")]
    pub value: i32,
}
/// <b>EV DC Charging Cable Capability (EVDCCableCapabilityKind enumeration)</b>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvdcCableCapabilityKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvdcCableCapabilityKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvdcConnectionStateAKind {
    #[prost(enumeration="EvdcConnectionStateAKind", tag="1")]
    pub value: i32,
}
/// <b>EV DC Connection State for IEC 61851-23/24 system A</b> <b>(EVDCConnectionStateAKind
/// enumeration)</b>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvdcConnectionStateAKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvdcConnectionStateAKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvdcConnectionStateCKind {
    #[prost(enumeration="EvdcConnectionStateCKind", tag="1")]
    pub value: i32,
}
/// <b>EV DC Connection State for IEC 61851-23/24 system C</b> <b>(EVDCConnectionStateCKind
/// enumeration)</b>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvdcConnectionStateCKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvdcConnectionStateCKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalEvdcPlugStateKind {
    #[prost(enumeration="EvdcPlugStateKind", tag="1")]
    pub value: i32,
}
/// <b>EV DC Plug Present and Coupler Lock State (EVDCPlugStateKind</b> <b>enumeration)</b>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsEvdcPlugStateKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<super::commonmodule::Quality>,
    /// Value of the data.
    #[prost(enumeration="EvdcPlugStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<super::commonmodule::Timestamp>,
}
/// Specialized 61850 DEDO class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAndStatusDedo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub event_and_status_deev: ::core::option::Option<EventAndStatusDeev>,
    /// Capability of the EV cable assembly (according to 61851-1)
    #[prost(message, optional, tag="3")]
    pub cab_rtg_dc: ::core::option::Option<EnsEvdcCableCapabilityKind>,
    /// Connection state for system A connection type (notation from IEC 61851-23/24 system A)
    #[prost(message, optional, tag="4")]
    pub conn_st_a: ::core::option::Option<EnsEvdcConnectionStateAKind>,
    /// Connection state for system C connection type (notation from IEC 61851-23/24 system c)
    #[prost(message, optional, tag="5")]
    pub conn_st_c: ::core::option::Option<EnsEvdcConnectionStateCKind>,
    /// Plug present and coupler lock state (according to 61851-1)
    #[prost(message, optional, tag="6")]
    pub plg_st_dc: ::core::option::Option<EnsEvdcPlugStateKind>,
}
/// Specialized 61850 DESE class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventAndStatusDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub event_and_status_deao: ::core::option::Option<EventAndStatusDeao>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub event_and_status_dedo: ::core::option::Option<EventAndStatusDedo>,
    /// Isolation test fault (i.e. the isolation test executed before charging has failed)
    #[prost(message, optional, tag="4")]
    pub iso_test_flt: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Short circuit test fault (i.e. short circuit test before charging has failed)
    #[prost(message, optional, tag="5")]
    pub sc_test_flt: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Detection of loss of digital communication
    #[prost(message, optional, tag="6")]
    pub dig_comm_los: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Detection of a welding condition
    #[prost(message, optional, tag="7")]
    pub wld_det: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub point_status: ::core::option::Option<EvsePointStatus>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub event_and_status_dese: ::prost::alloc::vec::Vec<EventAndStatusDese>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_event: ::core::option::Option<EvseEvent>,
}
/// LN: E-Mobility Electric Vehicle Name: DEEV
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingDeev {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// State of charge
    #[prost(message, optional, tag="2")]
    pub soc: ::core::option::Option<super::commonmodule::Mv>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingDeao {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub reading_deev: ::core::option::Option<ReadingDeev>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingDedo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub reading_deev: ::core::option::Option<ReadingDeev>,
}
/// LN: E-Mobility supply equipment Name: DESE
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseReadingDese {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub reading_deao: ::core::option::Option<ReadingDeao>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_dedo: ::core::option::Option<ReadingDedo>,
    /// Charging voltage  (AllOrNonePerGroup with ChaA)
    #[prost(message, optional, tag="4")]
    pub cha_v: ::core::option::Option<super::commonmodule::Mv>,
    /// Charging current (AllOrNonePerGroup with ChaV). actual current being drawn by the EV from the
    /// charger.
    #[prost(message, optional, tag="5")]
    pub cha_a: ::core::option::Option<super::commonmodule::Mv>,
    /// Power Import
    #[prost(message, optional, tag="6")]
    pub cha_w: ::core::option::Option<super::commonmodule::Mv>,
    /// Energy Import
    #[prost(message, optional, tag="7")]
    pub cha_wh: ::core::option::Option<super::commonmodule::Mv>,
}
/// EVSE reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::core::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub reading_mmxu: ::core::option::Option<super::commonmodule::ReadingMmxu>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmtr: ::core::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub evse_reading_dese: ::prost::alloc::vec::Vec<EvseReadingDese>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub phase_mmtn: ::core::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub reading_mmdc: ::core::option::Option<super::commonmodule::ReadingMmdc>,
}
/// EVSE Reading Profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_reading: ::core::option::Option<EvseReading>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub event_and_status_dese: ::prost::alloc::vec::Vec<EventAndStatusDese>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EvseStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub evse: ::core::option::Option<Evse>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub evse_status: ::core::option::Option<EvseStatus>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChargingStateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// There is no connection between EV and EVSE.
    Idle = 1,
    /// The contactor of the Connector is closed and energy is flowing to between EVSE and EV.
    Charging = 2,
    /// There is a connection between EV and EVSE, in case the protocol used between EV and the Charging
    /// Station can detect a connection, the protocol needs to detect this for the state to become active.
    /// The connection can either be wired or wireless.
    EvConnected = 3,
    /// When the EV is connected to the EVSE and the EVSE is offering energy but the EV is not taking
    /// any energy.
    SuspendedEv = 4,
    /// When the EV is connected to the EVSE but the EVSE is not offering energy to the EV, e.g. due to
    /// a smart charging restriction, local supply power constraints, or when charging has stopped because
    /// of the authorization status in the response to a transactionEventRequest indicating that charging is
    /// not allowed etc.
    SuspendedEvse = 5,
}
/// This enumeration lists the connection states of electrical vehicles (notation from IEC 61851-1).
/// Used in logical node DEAO or DEDO for IEC 61851-23 and -24 system C.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvacConnectionStateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// No vehicle connected
    StateA = 1,
    /// Vehicle connected, not ready for energy flow
    StateB = 2,
    /// Vehicle connected, ready for energy flow, ventilation not required
    StateC = 3,
    /// Vehicle connected, ready for energy flow, ventilation required
    StateD = 4,
    /// Vehicle connected, charge spot fault
    StateE = 5,
    /// Charge spot not available for action
    StateF = 6,
}
/// This enumeration lists the plug present and coupler lock states (according to IEC 61851-1). Used
/// in logical node DEAO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvacPlugStateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Disconnected = 1,
    /// MISSING DOCUMENTATION!!!
    ConnectedandUnlocked = 2,
    /// MISSING DOCUMENTATION!!!
    ConnectedandLocked = 3,
    /// Connected but not locked (no locking mechanism available)
    Connected = 4,
}
/// This enumeration lists the capability of the EV AC charging cable assembly (according to IEC
/// 61851-1). Used in logical node DEAO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvacCableCapabilityKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// 13 amps per phase
    EvacCableCapabilityKind13a = 1,
    /// 20 amps per phase
    EvacCableCapabilityKind20a = 2,
    /// 32 amps per phase
    EvacCableCapabilityKind32a = 3,
    /// 63 amps (3 phase) or 70 amps (1 phase)
    EvacCableCapabilityKind63to70A = 4,
}
/// Connection charging types (according to IEC 61851-1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvConnectionChargingKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// Single phase AC charging
    SinglePhase = 1,
    /// Three phase AC charging
    ThreePhase = 2,
    /// System A DC Charging
    SystemA = 3,
    /// System B DC Charging
    SystemB = 4,
    /// System C 5 System C DC Charging
    SystemC = 5,
}
/// This enumeration lists the capability of the EV DC charging cable assembly (according to IEC
/// 61851-1). Used in logical node DEDO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvdcCableCapabilityKind {
    /// Tera 10**12.
    Undefined = 0,
    /// 13 A
    EvdcCableCapabilityKind13a = 1,
    /// 16 to 20 A
    EvdcCableCapabilityKind16to20A = 2,
    /// 30 to 32 A
    EvdcCableCapabilityKind30to32A = 3,
    /// 60 to 63 A
    EvdcCableCapabilityKind60to63A = 4,
    /// 70 A
    EvdcCableCapabilityKind70a = 5,
    /// 80 A
    EvdcCableCapabilityKind80a = 6,
    /// 125 A
    EvdcCableCapabilityKind125a = 7,
    /// 200 A
    EvdcCableCapabilityKind200a = 8,
    /// 250 A
    EvdcCableCapabilityKind250a = 9,
    /// 400 A
    EvdcCableCapabilityKind400a = 10,
}
/// This enumeration lists the connection states of electrical vehicles for DC charging IEC 61851
/// 23/24 system A. Used in logical node DEDO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvdcConnectionStateAKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// Vehicle unconnected
    DcA = 1,
    /// Vehicle connected and start request
    DcB1 = 2,
    /// Initialisation 1: Handshaking
    DcB2 = 3,
    /// Initialisation 2: Vehicle connector lock Initialisation 3: Insulation test before charging
    DcB3 = 4,
    /// Energy transfer
    DcC = 5,
    /// Ventilation
    DcD = 6,
    /// Shutdown 1: Termination of current output
    DcBS1 = 7,
    /// Shutdown 2: Verification of voltage
    DcBS2 = 8,
    /// Shutdown 3: Connector unlock - Connector unlocked
    DcBS3 = 9,
    /// Shutdown 4: Connector unlock - End of charge at communication level
    DcBS4 = 10,
    /// Not ready
    DcE = 11,
    /// Not ready
    DcF = 12,
}
/// This enumeration lists the connection states of electrical vehicles for DC charging IEC 61851
/// 23/24 system C. Used in logical node DEDO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvdcConnectionStateCKind {
    /// Voltage source inverter regulating to P and Q references (VSI PQ)
    Undefined = 0,
    /// Current-source inverter (CSI)
    StateA = 1,
    /// Undefined
    StateB = 2,
    /// Voltage-controlled voltage-source inverter (VC-VSI)
    StateC = 3,
    /// Current-controlled voltage-source inverter (CC-VSI)
    StateD = 4,
    /// Not applicable / Unknown
    StateE = 5,
    /// MISSING DOCUMENTATION!!!
    StateF = 6,
}
/// This enumeration lists the plug present and coupler lock states (according to IEC 61851-1). Used
/// in logical node DEDO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EvdcPlugStateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Disconnected = 1,
    /// MISSING DOCUMENTATION!!!
    ConnectedandUnlocked = 2,
    /// MISSING DOCUMENTATION!!!
    ConnectedandLocked = 3,
    /// Connected but not locked (no locking mechanism available)
    Connected = 4,
}
