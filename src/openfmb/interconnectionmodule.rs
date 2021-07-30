/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectionPoint {
    /// Black start enable
    #[prost(message, optional, tag="1")]
    pub black_start_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable frequency set point
    #[prost(message, optional, tag="2")]
    pub frequency_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Island control
    #[prost(message, optional, tag="3")]
    pub island: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Black start enable
    #[prost(message, optional, tag="4")]
    pub pct_hz_droop: ::core::option::Option<f32>,
    /// Black start enable
    #[prost(message, optional, tag="5")]
    pub pct_v_droop: ::core::option::Option<f32>,
    /// Ramp rates
    #[prost(message, optional, tag="6")]
    pub ramp_rates: ::core::option::Option<super::commonmodule::RampRate>,
    /// Enable reactive power set point
    #[prost(message, optional, tag="7")]
    pub reactive_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable real power set point
    #[prost(message, optional, tag="8")]
    pub real_pwr_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Enable voltage set point
    #[prost(message, optional, tag="9")]
    pub voltage_set_point_enabled: ::core::option::Option<super::commonmodule::ControlSpc>,
    /// Start time
    #[prost(message, optional, tag="10")]
    pub start_time: ::core::option::Option<super::commonmodule::Timestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectionCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<InterconnectionPoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectionControlScheduleFsch {
    /// Discrete value in InterconnectionCSG type
    #[prost(message, optional, tag="1")]
    pub val_dcsg: ::core::option::Option<InterconnectionCsg>,
}
/// Specialized 61850 FSCC class for interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectionScheduleFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_fscc: ::core::option::Option<super::commonmodule::ControlFscc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub interconnection_control_schedule_fsch: ::prost::alloc::vec::Vec<InterconnectionControlScheduleFsch>,
}
/// Interconnection schedule
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterconnectionSchedule {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule_fscc: ::core::option::Option<InterconnectionScheduleFscc>,
}
/// Planned interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedInterconnectionScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub requester_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::core::option::Option<InterconnectionSchedule>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub tie_point: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub responder_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
}
/// Requested interconnection schedule profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestedInterconnectionScheduleProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub requester_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub interconnection_schedule: ::core::option::Option<InterconnectionSchedule>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub tie_point: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub responder_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
}
