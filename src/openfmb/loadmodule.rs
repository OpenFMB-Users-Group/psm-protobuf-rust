/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadPoint {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    pub reset: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
    /// Start time
    #[prost(message, optional, tag="6")]
    pub start_time: ::core::option::Option<super::commonmodule::ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<LoadPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadControlScheduleFsch {
    /// Discrete value in LoadCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<LoadCsg>,
}
/// Specialized FSCC 61850 class.  LN: Schedule controller   Name: FSCC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub load_control_schedule_fsch: ::core::option::Option<LoadControlScheduleFsch>,
}
/// Load control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_control_fscc: ::core::option::Option<LoadControlFscc>,
}
/// Load control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::core::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_control: ::core::option::Option<LoadControl>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadPointStatus {
    /// Ramp rates
    #[prost(message, optional, tag="1")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="2")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Enable joint real power set point
    #[prost(message, optional, tag="3")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Reset device
    #[prost(message, optional, tag="4")]
    pub reset: ::core::option::Option<super::commonmodule::StatusSps>,
    /// ESS state
    #[prost(message, optional, tag="5")]
    pub state: ::core::option::Option<super::commonmodule::OptionalStateKind>,
}
/// Specialized 61850 ZGLD class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadEventAndStatusZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// Emergency stop
    #[prost(message, optional, tag="3")]
    pub emg_stop: ::core::option::Option<super::commonmodule::StatusSps>,
    /// Point status
    #[prost(message, optional, tag="4")]
    pub point_status: ::core::option::Option<LoadPointStatus>,
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadEventZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub load_event_and_status_zgld: ::core::option::Option<LoadEventAndStatusZgld>,
}
/// Load event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub load_event_zgld: ::core::option::Option<LoadEventZgld>,
}
/// Load event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::core::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_event: ::core::option::Option<LoadEvent>,
}
/// Load reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadReading {
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
/// Load reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::core::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_reading: ::core::option::Option<LoadReading>,
}
/// Specialized 61850 ZGLD LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStatusZgld {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub load_event_and_status_zgld: ::core::option::Option<LoadEventAndStatusZgld>,
}
/// Load status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// True if the load is uncontrollable.
    #[prost(message, optional, tag="2")]
    pub is_uncontrollable: ::core::option::Option<bool>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_status_zgld: ::core::option::Option<LoadStatusZgld>,
}
/// Load status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub energy_consumer: ::core::option::Option<super::commonmodule::EnergyConsumer>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub load_status: ::core::option::Option<LoadStatus>,
}
