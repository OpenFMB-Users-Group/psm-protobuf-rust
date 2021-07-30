/// Cap bank compensator system
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// LN: Power cap bank  Name: YPSH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankControlYpsh {
    /// Current limit (boolean field)
    #[prost(message, optional, tag="1")]
    pub amp_lmt: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// High current  threshold
    #[prost(message, optional, tag="2")]
    pub amp_thd_hi: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Low current threshold
    #[prost(message, optional, tag="3")]
    pub amp_thd_lo: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Control mode auto
    #[prost(message, optional, tag="4")]
    pub ctl_mode_auto: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Control mode override
    #[prost(message, optional, tag="5")]
    pub ctl_mode_ovr_rd: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Control mode remote
    #[prost(message, optional, tag="6")]
    pub ctl_mode_rem: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// The control characteristics for power flow operation
    #[prost(message, optional, tag="7")]
    pub dir_mode: ::core::option::Option<super::commonmodule::OptionalDirectionModeKind>,
    /// (controllable) Position of the switch of power shunt.
    #[prost(message, optional, tag="8")]
    pub pos: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// Temperature limit (boolean field)
    #[prost(message, optional, tag="9")]
    pub temp_lmt: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// High temperature threshold
    #[prost(message, optional, tag="10")]
    pub temp_thd_hi: ::core::option::Option<super::commonmodule::ControlApc>,
    /// Low temperature threshold
    #[prost(message, optional, tag="11")]
    pub temp_thd_lo: ::core::option::Option<super::commonmodule::ControlApc>,
    /// VAr limit (boolean field)
    #[prost(message, optional, tag="12")]
    pub v_ar_lmt: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// High VAr threshold
    #[prost(message, optional, tag="13")]
    pub v_ar_thd_hi: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Low VAr threshold
    #[prost(message, optional, tag="14")]
    pub v_ar_thd_lo: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Voltage limit (boolean field)
    #[prost(message, optional, tag="15")]
    pub vol_lmt: ::core::option::Option<super::commonmodule::PhaseSpc>,
    /// High voltage threshold
    #[prost(message, optional, tag="16")]
    pub vol_thd_hi: ::core::option::Option<super::commonmodule::PhaseApc>,
    /// Low voltage threshold
    #[prost(message, optional, tag="17")]
    pub vol_thd_lo: ::core::option::Option<super::commonmodule::PhaseApc>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankPoint {
    /// Regulator control
    #[prost(message, optional, tag="1")]
    pub control: ::core::option::Option<CapBankControlYpsh>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<CapBankPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankControlScheduleFsch {
    /// Control value in CSG type
    #[prost(message, optional, tag="1")]
    pub val_csg: ::core::option::Option<CapBankCsg>,
}
/// Using 61850 FSCC for cap bank control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_control_schedule_fsch: ::core::option::Option<CapBankControlScheduleFsch>,
}
/// CapBank control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_control_fscc: ::core::option::Option<CapBankControlFscc>,
}
/// Cap bank control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_control: ::core::option::Option<CapBankControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::core::option::Option<CapBankSystem>,
}
/// OpenFMB specialization for cap bank discrete control:
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankDiscreteControlYpsh {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control: ::core::option::Option<CapBankControlYpsh>,
}
/// Cap bank discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_discrete_control_ypsh: ::core::option::Option<CapBankDiscreteControlYpsh>,
}
/// Cap bank discrete control profile.  Instructs an end device (or an end device group) to perform
/// a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_control: ::core::option::Option<CapBankDiscreteControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::core::option::Option<CapBankSystem>,
}
/// LN: Power cap bank  Name: YPSH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankEventAndStatusYpsh {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// True if current limit (high, low, or both) are set
    #[prost(message, optional, tag="2")]
    pub amp_lmt: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// Control mode
    #[prost(message, optional, tag="3")]
    pub ctl_mode: ::core::option::Option<super::commonmodule::OptionalControlModeKind>,
    /// True if power flow is reversed
    #[prost(message, optional, tag="4")]
    pub dir_rev: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// (controllable) Position of the switch of power shunt.
    #[prost(message, optional, tag="6")]
    pub pos: ::core::option::Option<super::commonmodule::PhaseDps>,
    /// True if temperature limit (high, low, or both) are set
    #[prost(message, optional, tag="7")]
    pub temp_lmt: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// True if VAr limit (high, low, or both) are set
    #[prost(message, optional, tag="8")]
    pub v_ar_lmt: ::core::option::Option<super::commonmodule::PhaseSps>,
    /// True if voltage limit (high, low, or both) are set
    #[prost(message, optional, tag="9")]
    pub vol_lmt: ::core::option::Option<super::commonmodule::PhaseSps>,
}
/// Cap bank event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_event_and_status_ypsh: ::core::option::Option<CapBankEventAndStatusYpsh>,
}
/// Cap bank status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_event: ::core::option::Option<CapBankEvent>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::core::option::Option<CapBankSystem>,
}
/// Cap bank reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankReading {
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
/// Cap bank reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_reading: ::core::option::Option<CapBankReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::core::option::Option<CapBankSystem>,
}
/// Cap bank status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_event_and_status_ypsh: ::core::option::Option<CapBankEventAndStatusYpsh>,
}
/// Cap bank status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapBankStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub cap_bank_status: ::core::option::Option<CapBankStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub cap_bank_system: ::core::option::Option<CapBankSystem>,
}
