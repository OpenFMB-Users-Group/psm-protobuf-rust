/// Reclose enabled
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerDiscreteControlXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub discrete_control_xcbr: ::core::option::Option<super::commonmodule::DiscreteControlXcbr>,
}
/// Breaker discrete control class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// IEC61850 enhanced control parameters.
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_discrete_control_xcbr: ::core::option::Option<BreakerDiscreteControlXcbr>,
}
/// A mechanical switching device capable of making, carrying, and breaking currents under normal
/// circuit conditions and also making, carrying for a specified time, and breaking currents under
/// specified abnormal circuit conditions e.g.  those of short circuit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Breaker {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Instructs an end device (or an end device group) to perform a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::core::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_discrete_control: ::core::option::Option<BreakerDiscreteControl>,
}
/// Breaker event class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::core::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Breaker event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::core::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_event: ::core::option::Option<BreakerEvent>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::core::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub diff_reading_mmxu: ::core::option::Option<super::commonmodule::ReadingMmxu>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub phase_mmtn: ::core::option::Option<super::commonmodule::PhaseMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reading_mmtr: ::core::option::Option<super::commonmodule::ReadingMmtr>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub reading_mmxu: ::core::option::Option<super::commonmodule::ReadingMmxu>,
}
/// Breaker reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::core::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub breaker_reading: ::prost::alloc::vec::Vec<BreakerReading>,
}
/// Status of a breaker
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::core::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Breaker status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BreakerStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub breaker: ::core::option::Option<Breaker>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub breaker_status: ::core::option::Option<BreakerStatus>,
}
