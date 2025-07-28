/// Environment sensor reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentReading {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment_terminal_reading: ::core::option::Option<super::commonmodule::ConductingEquipmentTerminalReading>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub reading_menv: ::core::option::Option<super::commonmodule::ReadingMenv>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reading_mmet: ::core::option::Option<super::commonmodule::ReadingMmet>,
}
/// Environment reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub sensor: ::core::option::Option<super::commonmodule::Sensor>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub environment_reading: ::core::option::Option<EnvironmentReading>,
}
