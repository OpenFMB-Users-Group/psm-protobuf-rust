/// [OpenFMB LN extension] Reserve margin in A, W, VAr and VA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveMargin {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// Phase to ground/phase to neutral three phase currents.
    #[prost(message, optional, tag="2")]
    pub a: ::core::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral apparent powers S.
    #[prost(message, optional, tag="3")]
    pub va: ::core::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral reactive powers Q.
    #[prost(message, optional, tag="4")]
    pub v_ar: ::core::option::Option<super::commonmodule::Pmg>,
    /// Phase to ground/phase to neutral real powers P.
    #[prost(message, optional, tag="5")]
    pub w: ::core::option::Option<super::commonmodule::Pmg>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveAvailability {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub incremental_margin: ::core::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub margin: ::core::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub standby_margin: ::core::option::Option<ReserveMargin>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocatedMargin {
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub allocated_margin: ::core::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub allocated_standby_margin: ::core::option::Option<ReserveMargin>,
}
/// Reserve availability profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveAvailabilityProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub allocated_margin: ::core::option::Option<AllocatedMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub requester_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reserve_availability: ::core::option::Option<ReserveAvailability>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub responder_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub tie_point: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRequest {
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="1")]
    pub request_id: ::prost::alloc::string::String,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub margin: ::core::option::Option<ReserveMargin>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub standby_margin: ::core::option::Option<ReserveMargin>,
}
/// Reserve availability profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReserveRequestProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub requester_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reserve_request: ::core::option::Option<ReserveRequest>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub responder_circuit_segment_service: ::core::option::Option<super::commonmodule::ApplicationSystem>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub tie_point: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
