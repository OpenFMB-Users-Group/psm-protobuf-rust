/// Protection mode such as a group setting or pre-defined curve profile. It is usually pre-defined
/// by a circuit segment service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserDiscreteControlXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub discrete_control_xcbr: ::core::option::Option<super::commonmodule::DiscreteControlXcbr>,
}
/// Recloser discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub recloser_discrete_control_xcbr: ::core::option::Option<RecloserDiscreteControlXcbr>,
}
/// Pole-mounted fault interrupter with built-in phase and ground relays, current transformer (CT),
/// and supplemental controls.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recloser {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub normal_open: ::core::option::Option<bool>,
}
/// Recloser control profile.  Instructs an end device (or an end device group) to perform a
/// specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub recloser: ::core::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub recloser_discrete_control: ::core::option::Option<RecloserDiscreteControl>,
}
/// Recloser event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::core::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Recloser event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub recloser: ::core::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub recloser_event: ::core::option::Option<RecloserEvent>,
}
/// Recloser reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserReading {
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
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub reading_mmdc: ::core::option::Option<super::commonmodule::ReadingMmdc>,
}
/// Recloser reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub recloser: ::core::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub recloser_reading: ::prost::alloc::vec::Vec<RecloserReading>,
}
/// Recloser status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub status_and_event_xcbr: ::core::option::Option<super::commonmodule::StatusAndEventXcbr>,
}
/// Recloser status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecloserStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub recloser: ::core::option::Option<Recloser>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub recloser_status: ::core::option::Option<RecloserStatus>,
}
