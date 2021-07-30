/// OpenFMB specialization for switch control:  LN: Circuit switch   Name: XSWI
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchDiscreteControlXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<super::commonmodule::LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::core::option::Option<super::commonmodule::PhaseDpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub reset_protection_pickup: ::core::option::Option<super::commonmodule::ControlSpc>,
}
/// Switch discrete control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchDiscreteControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_value: ::core::option::Option<super::commonmodule::ControlValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub check: ::core::option::Option<super::commonmodule::CheckConditions>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_discrete_control_xswi: ::core::option::Option<SwitchDiscreteControlXswi>,
}
/// A ProtectedSwitch is a switching device that can be operated by ProtectionEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectedSwitch {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<super::commonmodule::ConductingEquipment>,
}
/// Switch control profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchDiscreteControlProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub control_message_info: ::core::option::Option<super::commonmodule::ControlMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::core::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_discrete_control: ::core::option::Option<SwitchDiscreteControl>,
}
/// OpenFMB specialization for SwitchEventProfile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchEventXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// Dynamic test status
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::core::option::Option<super::commonmodule::PhaseDps>,
}
/// Switch event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchEvent {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_value: ::core::option::Option<super::commonmodule::EventValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub switch_event_xswi: ::core::option::Option<SwitchEventXswi>,
}
/// Switch event profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchEventProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub event_message_info: ::core::option::Option<super::commonmodule::EventMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::core::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_event: ::core::option::Option<SwitchEvent>,
}
/// Switch reading value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchReading {
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
/// Switch reading profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchReadingProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub reading_message_info: ::core::option::Option<super::commonmodule::ReadingMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::core::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="3")]
    pub switch_reading: ::prost::alloc::vec::Vec<SwitchReading>,
}
/// OpenFMB specialization for SwitchStatusProfile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchStatusXswi {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<super::commonmodule::LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::core::option::Option<super::commonmodule::EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub pos: ::core::option::Option<super::commonmodule::PhaseDps>,
    /// Fault latch: LT01=51A OR 51B OR 51C
    #[prost(message, optional, tag="5")]
    pub protection_pickup: ::core::option::Option<super::commonmodule::PhaseSps>,
}
/// Switch status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_value: ::core::option::Option<super::commonmodule::StatusValue>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub switch_status_xswi: ::core::option::Option<SwitchStatusXswi>,
}
/// Switch status profile
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchStatusProfile {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub status_message_info: ::core::option::Option<super::commonmodule::StatusMessageInfo>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub protected_switch: ::core::option::Option<ProtectedSwitch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub switch_status: ::core::option::Option<SwitchStatus>,
}
