/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanControlGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::core::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
    /// (controllable) If true, generic single point controllable status output <i>n</i> has been
    /// enabled, otherwise it has been disabled.
    #[prost(message, optional, tag="3")]
    pub spcso: ::core::option::Option<super::commonmodule::ControlSpc>,
}
/// Status expressed in integer based on IEC61850 GGIO.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerControlGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// (controllable) Generic integer controllable status output <i>n</i>.
    #[prost(message, optional, tag="2")]
    pub iscso: ::core::option::Option<super::commonmodule::ControlInc>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::core::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringControlGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::core::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
    /// String control
    #[prost(message, optional, tag="3")]
    pub str_out: ::core::option::Option<super::commonmodule::Vsc>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalogControlGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<super::commonmodule::LogicalNode>,
    /// (controllable) Value of the generic controllable analogue output setpoint <i>n</i>. Analog value
    /// (MX) feeds back the setpoint of the output.
    #[prost(message, optional, tag="2")]
    pub an_out: ::core::option::Option<super::commonmodule::ControlApc>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::core::option::Option<super::commonmodule::OptionalPhaseCodeKind>,
}
/// Resource discrete control class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// IEC61850 enhanced control parameters.
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub analog_control_ggio: ::prost::alloc::vec::Vec<AnalogControlGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub boolean_control_ggio: ::prost::alloc::vec::Vec<BooleanControlGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="5")]
    pub integer_control_ggio: ::prost::alloc::vec::Vec<IntegerControlGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="6")]
    pub string_control_ggio: ::prost::alloc::vec::Vec<StringControlGgio>,
}
/// Instructs a resource to perform a specified action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub resource_discrete_control: ::core::option::Option<ResourceDiscreteControl>,
}
/// Resource reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReading {
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
/// Resource reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub resource_reading: ::core::option::Option<ResourceReading>,
}
/// Current event information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub analog_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub boolean_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub integer_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="5")]
    pub string_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::StringEventAndStatusGgio>,
}
/// Resource event module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub resource_event: ::core::option::Option<ResourceEvent>,
}
/// Current status information relevant to an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<super::commonmodule::IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="2")]
    pub analog_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::AnalogEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub boolean_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::BooleanEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="4")]
    pub integer_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::IntegerEventAndStatusGgio>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="5")]
    pub string_event_and_status_ggio: ::prost::alloc::vec::Vec<super::commonmodule::StringEventAndStatusGgio>,
}
/// Resource status module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub resource_status: ::core::option::Option<ResourceStatus>,
}
