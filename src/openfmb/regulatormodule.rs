/// LN: Automatic tap changer controller   Name: ATCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectionalAtcc {
    /// Control (secondary) voltage bandwidth (i.e., range), given either as voltage value or percentage
    /// of the nominal voltage (forward power flow presumed).
    #[prost(message, optional, tag="1")]
    pub bnd_wid: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Time to wait before operating, after reaching the control point (forward power flow presumed).
    #[prost(message, optional, tag="2")]
    pub ctl_dl_tmms: ::core::option::Option<super::commonmodule::PhaseIsc>,
    /// Line drop voltage due to line resistance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="3")]
    pub ldcr: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Line drop voltage due to line reactance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="4")]
    pub ldcx: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// (controllable) Voltage setpoint. Analog value (MX) feeds back the setpoint of the controller.
    #[prost(message, optional, tag="5")]
    pub vol_spt: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="6")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::PhaseDpc>,
}
/// LN: Automatic tap changer controller   Name: ATCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorControlAtcc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// Forward voltage regulation
    #[prost(message, optional, tag="2")]
    pub dir_fwd: ::core::option::Option<DirectionalAtcc>,
    /// The control characteristics for power flow operation
    #[prost(message, optional, tag="3")]
    pub dir_mode: ::core::option::Option<super::commonmodule::OptionalDirectionModeKind>,
    /// Reverse voltage regulation
    #[prost(message, optional, tag="4")]
    pub dir_rev: ::core::option::Option<DirectionalAtcc>,
    /// This is the percentage used to determine the current threshold at which the control recognizes
    /// current flow direction. Below the threshold, the current flow is considered to be indeterminate.
    #[prost(message, optional, tag="5")]
    pub dir_thd: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// (controllable) If true, transformers operate in parallel, otherwise they operate independently.
    #[prost(message, optional, tag="6")]
    pub par_op: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// Ramp rates
    #[prost(message, optional, tag="7")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// (controllable) Tap position change to the specified value.
    #[prost(message, optional, tag="8")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// If true, tap position shall be lowered.
    #[prost(message, optional, tag="9")]
    pub tap_op_l: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// If true, tap position shall be raised.
    #[prost(message, optional, tag="10")]
    pub tap_op_r: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// High voltage limit for Voltage Limiter
    #[prost(message, optional, tag="11")]
    pub vol_lmt_hi: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Low voltage limit for Voltage Limiter
    #[prost(message, optional, tag="12")]
    pub vol_lmt_lo: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Voltage-limiting types
    #[prost(message, optional, tag="13")]
    pub vol_lmt_mode: ::core::option::Option<super::commonmodule::OptionalVoltLimitModeKind>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub control: ::core::option::Option<RegulatorControlAtcc>,
    /// Start time
    #[prost(message, optional, tag="8")]
    pub start_time: ::core::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<RegulatorPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorControlScheduleFsch {
    /// Discrete value in RegulatorCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<RegulatorCsg>,
}
/// Using 61850 FSCC for regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_control_schedule_fsch: ::core::option::Option<RegulatorControlScheduleFsch>,
}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_control_fscc: ::core::option::Option<RegulatorControlFscc>,
}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Regulator control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_control: ::core::option::Option<RegulatorControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_system: ::core::option::Option<RegulatorSystem>,
}
/// Regulator control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_control_atcc: ::core::option::Option<RegulatorControlAtcc>,
}
/// Regulator control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_discrete_control: ::core::option::Option<RegulatorDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_system: ::core::option::Option<RegulatorSystem>,
}
/// LN: Automatic tap changer controller   Name: ATCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorEventAndStatusAtcc {
    /// Centre of voltage control bandwidth (forward power flow presumed).
    #[prost(message, optional, tag="1")]
    pub bnd_ctr: ::core::option::Option<super::commonmodule::Asg>,
    /// Control (secondary) voltage bandwidth (i.e., range), given either as voltage value or percentage
    /// of the nominal voltage (forward power flow presumed).
    #[prost(message, optional, tag="2")]
    pub bnd_wid: ::core::option::Option<super::commonmodule::Asg>,
    /// Compensated Voltage Secondary compared with set point plus or minus Bandwidth
    #[prost(message, optional, tag="3")]
    pub bnd_wid_hi: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// Compensated Voltage Secondary compared with set point plus or minus Bandwidth
    #[prost(message, optional, tag="4")]
    pub bnd_wid_lo: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// Current Power Direction is the direction that regulator is regulating.
    #[prost(message, optional, tag="5")]
    pub dir_ctl_rev: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// True if direction is indeterminate.
    #[prost(message, optional, tag="6")]
    pub dir_indt: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// True if the current direction is the same as the System Direction
    #[prost(message, optional, tag="7")]
    pub dir_rev: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// Line drop voltage due to line resistance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="8")]
    pub ldcr: ::core::option::Option<super::commonmodule::Asg>,
    /// Line drop voltage due to line reactance component (forward power flow presumed) at rated current.
    #[prost(message, optional, tag="9")]
    pub ldcx: ::core::option::Option<super::commonmodule::Asg>,
    /// (controllable) If true, transformers operate in parallel, otherwise they operate independently.
    #[prost(message, optional, tag="10")]
    pub par_op: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Ramp rates
    #[prost(message, optional, tag="11")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// State
    #[prost(message, optional, tag="12")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// OpenFMB extension:  Status for the time to wait before operating (CtrlDlTmms)
    #[prost(message, optional, tag="13")]
    pub st_dl_tmms: ::core::option::Option<super::commonmodule::StatusInc>,
    /// If true, there was an error in tap position change, or in tap indication (for instance, wrong
    /// Binary Coded Decimal (BCD) code).
    #[prost(message, optional, tag="14")]
    pub tap_op_err: ::core::option::Option<super::commonmodule::StatusSps>,
    /// (controllable) Tap position change to the specified value.
    #[prost(message, optional, tag="15")]
    pub tap_pos: ::core::option::Option<super::commonmodule::PhaseIns>,
    /// Load Voltage Secondary compared with VolLmtHi
    #[prost(message, optional, tag="16")]
    pub vol_lmt_hi: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// Load Voltage Secondary compared with VolLmtLo
    #[prost(message, optional, tag="17")]
    pub vol_lmt_lo: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// (controllable) Voltage setpoint. Analog value (MX) feeds back the setpoint of the controller.
    #[prost(message, optional, tag="18")]
    pub vol_spt: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Voltage set point status
    #[prost(message, optional, tag="19")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
}
/// OpenFMB 61850 specialization for both RegulatorEventProfile and RegulatorStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorEventAndStatusAncr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub point_status: ::core::option::Option<RegulatorEventAndStatusAtcc>,
}
/// Regulator event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_event_and_status_ancr: ::core::option::Option<RegulatorEventAndStatusAncr>,
}
/// Regulator event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_event: ::core::option::Option<RegulatorEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_system: ::core::option::Option<RegulatorSystem>,
}
/// Regulator reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorReading {
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
    pub secondary_reading_mmxu: ::core::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Regulator reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub regulator_reading: ::prost::alloc::vec::Vec<RegulatorReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_system: ::core::option::Option<RegulatorSystem>,
}
/// Regulator status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_event_and_status_ancr: ::core::option::Option<RegulatorEventAndStatusAncr>,
}
/// Regulator status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegulatorStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub regulator_status: ::core::option::Option<RegulatorStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub regulator_system: ::core::option::Option<RegulatorSystem>,
}
