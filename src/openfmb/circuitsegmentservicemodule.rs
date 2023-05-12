#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalCircuitSegmentServiceModeKind {
    #[prost(enumeration="CircuitSegmentServiceModeKind", tag="1")]
    pub value: i32,
}
/// Circuit Segment service mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngCircuitSegmentServiceModeKind {
    /// The value of the coordination service mode.
    #[prost(enumeration="CircuitSegmentServiceModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::core::option::Option<::prost::alloc::string::String>,
}
/// OpenFMB specialization for circuit segment service control, DCSC (Distributed Coordination
/// Service Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentControlDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub circuit_segment_service_mode: ::core::option::Option<EngCircuitSegmentServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::core::option::Option<super::commonmodule::ControlDpc>,
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub circuit_segment_control_dcsc: ::core::option::Option<CircuitSegmentControlDcsc>,
}
/// Switch control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub circuit_segment_control: ::core::option::Option<CircuitSegmentControl>,
}
/// OpenFMB specialization for circuit segment service control, DCSC (Distributed Coordination
/// Service Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentEventDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub circuit_segment_service_mode: ::core::option::Option<EngCircuitSegmentServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub permissible_auto: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub permissible_manual: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub permissible_netzero: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub permissible_start: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub permissible_stop: ::core::option::Option<super::commonmodule::StatusSps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub circuit_segment_event_dcsc: ::core::option::Option<CircuitSegmentEventDcsc>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub circuit_segment_event: ::core::option::Option<CircuitSegmentEvent>,
}
/// OpenFMB specialization for coordination service control, DCSC (Distributed Coordination Service
/// Control), following 61850 naming convention.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentStatusDcsc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub circuit_segment_service_mode: ::core::option::Option<EngCircuitSegmentServiceModeKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island: ::core::option::Option<super::commonmodule::StatusDps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub permissible_auto: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub permissible_manual: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub permissible_netzero: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="7")]
    pub permissible_start: ::core::option::Option<super::commonmodule::StatusSps>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="8")]
    pub permissible_stop: ::core::option::Option<super::commonmodule::StatusSps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub circuit_segment_status_dcsc: ::core::option::Option<CircuitSegmentStatusDcsc>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitSegmentStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub application_system: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub circuit_segment_status: ::core::option::Option<CircuitSegmentStatus>,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CircuitSegmentServiceModeKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    None = 1,
    /// MISSING DOCUMENTATION!!!
    Auto = 2,
    /// MISSING DOCUMENTATION!!!
    Manual = 3,
    /// MISSING DOCUMENTATION!!!
    Netzero = 4,
    /// MISSING DOCUMENTATION!!!
    Start = 5,
    /// MISSING DOCUMENTATION!!!
    Stop = 6,
}
