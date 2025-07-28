#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalFaultDirectionKind {
    #[prost(enumeration="FaultDirectionKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalPhaseFaultDirectionKind {
    #[prost(enumeration="PhaseFaultDirectionKind", tag="1")]
    pub value: i32,
}
/// Directional protection indication information (ACD)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acd {
    /// General direction of the fault. If the faults of individual phases have different directions,
    /// this attribute shall be set to 'dirGeneral'='both'.
    #[prost(enumeration="FaultDirectionKind", tag="1")]
    pub dir_general: i32,
    /// Direction of the fault for earth current.
    #[prost(message, optional, tag="2")]
    pub dir_neut: ::core::option::Option<OptionalPhaseFaultDirectionKind>,
    /// Direction of the fault for phase A.
    #[prost(message, optional, tag="3")]
    pub dir_phs_a: ::core::option::Option<OptionalPhaseFaultDirectionKind>,
    /// Direction of the fault for phase B.
    #[prost(message, optional, tag="4")]
    pub dir_phs_b: ::core::option::Option<OptionalPhaseFaultDirectionKind>,
    /// Direction of the fault for phase C.
    #[prost(message, optional, tag="5")]
    pub dir_phs_c: ::core::option::Option<OptionalPhaseFaultDirectionKind>,
    /// General indication of a protection activation (e.g. by the fault). Depending on the function,
    /// 'general' may or may not be resulting from the phase attributes (phsA', 'phsB', 'phsC', 'neut').
    #[prost(bool, tag="6")]
    pub general: bool,
    /// See 'ACT.neut'.
    #[prost(message, optional, tag="7")]
    pub neut: ::core::option::Option<bool>,
    /// Value true indicates a trip or a start event of phase A.
    #[prost(message, optional, tag="8")]
    pub phs_a: ::core::option::Option<bool>,
    /// Value true indicates a trip or a start event of phase B.
    #[prost(message, optional, tag="9")]
    pub phs_b: ::core::option::Option<bool>,
    /// Value true indicates a trip or a start event of phase C.
    #[prost(message, optional, tag="10")]
    pub phs_c: ::core::option::Option<bool>,
}
/// This is a root class to provide common identification for all classes needing identification and
/// naming attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedObject {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Master resource identifier issued by a model authority. The mRID must semantically be a UUID as
    /// specified in RFC 4122. The mRID is globally unique.
    #[prost(message, optional, tag="2")]
    pub m_rid: ::core::option::Option<::prost::alloc::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag="3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// An electrical connection point (AC or DC) to a piece of conducting equipment. Terminals are
/// connected at physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcdcTerminal {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// The connected status is related to a bus-branch model and the topological node to terminal
    /// relation.  True implies the terminal is connected to the related topological node and false implies
    /// it is not.  In a bus-branch model, the connected status is used to tell if equipment is disconnected
    /// without having to change the connectivity described by the topological node to terminal relation. A
    /// valid case is that conducting equipment can be connected in one end and open in the other. In
    /// particular for an AC line segment, where the reactive line charging can be significant, this is a
    /// relevant case.
    #[prost(message, optional, tag="2")]
    pub connected: ::core::option::Option<bool>,
    /// The orientation of the terminal connections for a multiple terminal conducting equipment.  The
    /// sequence numbering starts with 1 and additional terminals should follow in increasing order.   The
    /// first terminal is the "starting point" for a two terminal branch.
    #[prost(message, optional, tag="3")]
    pub sequence_number: ::core::option::Option<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalUnitSymbolKind {
    #[prost(enumeration="UnitSymbolKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalUnitMultiplierKind {
    #[prost(enumeration="UnitMultiplierKind", tag="1")]
    pub value: i32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActivePower {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub multiplier: ::core::option::Option<OptionalUnitMultiplierKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub unit: ::core::option::Option<OptionalUnitSymbolKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub value: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalPhaseCodeKind {
    #[prost(enumeration="PhaseCodeKind", tag="1")]
    pub value: i32,
}
/// Unit definition (Unit)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Unit {
    /// (default='') Unit multiplier.
    #[prost(message, optional, tag="1")]
    pub multiplier: ::core::option::Option<OptionalUnitMultiplierKind>,
    /// SI unit of measure.
    #[prost(enumeration="UnitSymbolKind", tag="2")]
    pub si_unit: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalValidityKind {
    #[prost(enumeration="ValidityKind", tag="1")]
    pub value: i32,
}
/// Describes some reasons in case 'validity' is not 'good'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailQual {
    /// (default=false) If true, the value may not be a correct value due to a reference being out of
    /// calibration. The server shall decide if validity shall be set to invalid or questionable (used for
    /// measurand information and binary counter information only).
    #[prost(bool, tag="1")]
    pub bad_reference: bool,
    /// (default=false) If true, a supervision function has detected an internal or external failure.
    #[prost(bool, tag="2")]
    pub failure: bool,
    /// (default=false) If true, the value does not meet the stated accuracy of the source. EXAMPLE The
    /// measured value of power factor may be noisy (inaccurate) when the current is very small.
    #[prost(bool, tag="3")]
    pub inaccurate: bool,
    /// (default=false) If true, an evaluation function has detected an inconsistency.
    #[prost(bool, tag="4")]
    pub inconsistent: bool,
    /// (default=false) If true, an update is not made during a specific time interval. The value may be
    /// an old value that may have changed in the meantime. This specific time interval may be defined by an
    /// allowed-age attribute. NOTE "Fail silent" errors, where the equipment stops sending data, will cause
    /// setting this flag to true. In this case, the last received information was correct.
    #[prost(bool, tag="5")]
    pub old_data: bool,
    /// (default=false) To prevent overloading of event driven communication channels, it is desirable
    /// to detect and suppress oscillating (fast changing) binary inputs. If a signal changes in a defined
    /// time (tosc) twice in the same direction (from 0 to 1 or from 1 to 0), then it shall be defined as an
    /// oscillation and this attribute shall be set to true. If a configured number of transient changes is
    /// detected, they shall be suppressed. In this time, the 'Quality.validity' shall be set to
    /// 'questionable'. If the signal is still in the oscillating state after the defined number of changes,
    /// the value shall be left in the state it was in when this flag was set. In this case, the 'Quality
    /// validity' shall be changed from 'questionable' to 'invalid' and kept so as long as the signal is
    /// oscillating.  If the configuration is such that all transient changes should be suppressed, the
    /// 'Quality.validity' shall be set immediately to 'invalid' and this flag to true (used for status
    /// information only).
    #[prost(bool, tag="6")]
    pub oscillatory: bool,
    /// (default=false) If true, the attribute to which the quality has been associated is beyond a
    /// predefined range of values. The server shall decide if validity shall be set to invalid or
    /// questionable (used for measurand information only). EXAMPLE A measured value may exceed a predefined
    /// range, however the selected data type can still represent the value, for example the data type is a
    /// 16-bit unsigned integer, the predefined range is 0 to 40 000, if the value is between 40 001 and 65
    /// 535 it is considered to be out of range.
    #[prost(bool, tag="7")]
    pub out_of_range: bool,
    /// (default=false) If true, the value of the attribute to which the quality has been associated is
    /// beyond the capability of being represented properly (used for measurand information only). EXAMPLE A
    /// measured value may exceed the range that may be represented by the selected data type, for example
    /// the data type is a 16-bit unsigned integer and the value exceeds 65 535.
    #[prost(bool, tag="8")]
    pub overflow: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalSourceKind {
    #[prost(enumeration="SourceKind", tag="1")]
    pub value: i32,
}
/// Quality contains data that describe the quality of the data from the server. Quality of the data
/// is also related to the mode of a logical node. Further details can be found in IEC 61850-7-4. The
/// different quality attributes are <i>not</i> independent.The default value shall be applied if the
/// functionality of the related attribute is not supported. The mapping may specify to exclude the
/// attribute from the message if it is not supported or if the default value applies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quality {
    /// Describes some reasons in case 'validity' is not 'good'.
    #[prost(message, optional, tag="1")]
    pub detail_qual: ::core::option::Option<DetailQual>,
    /// (default=false) If true, further update of the value has been blocked by an operator. The value
    /// shall be the information that was acquired before blocking. If this flag is set, then the
    /// 'detailQual.oldData' shall also be set. The operator shall use the data attribute 'CDC.blkEna' to
    /// block the update of the value. NOTE Both an operator as well as an automatic function may freeze
    /// communication updating as well as input updating. In both cases, 'detailQual.oldData' will be set.
    /// If the blocking is done by an operator, then this flag is set additionally, and an operator activity
    /// is required to clear the condition.
    #[prost(bool, tag="2")]
    pub operator_blocked: bool,
    /// (default=process) Defines the source of a value. NOTE 1 Substitution may be done locally or via
    /// the communication services. In the second case, specific attributes with a FC=SV are used. NOTE 2
    /// There are various means to clear a substitution. As an example, a substitution that was done
    /// following an invalid condition may be cleared automatically if the invalid condition is cleared.
    /// However, this is a local issue and therefore not within the scope of this standard.
    #[prost(enumeration="SourceKind", tag="3")]
    pub source: i32,
    /// (default=false) If true, the value is a test value. The processing of the test quality in the
    /// client shall be as described in IEC 61850-7-4. This bit shall be completely independent from the
    /// other bits within the quality descriptor.
    #[prost(bool, tag="4")]
    pub test: bool,
    /// Validity of the value, as condensed information for the client. In case this value is not
    /// 'good', some reasons may be found in the 'detailQual'.
    #[prost(enumeration="ValidityKind", tag="5")]
    pub validity: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalTimeAccuracyKind {
    #[prost(enumeration="TimeAccuracyKind", tag="1")]
    pub value: i32,
}
/// Information about the quality of the time source of the sending IED.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeQuality {
    /// If true, the time source of the sending device is unreliable and the value of the time stamp
    /// shall be ignored.
    #[prost(bool, tag="1")]
    pub clock_failure: bool,
    /// If true, the time source of the sending device is not synchronised with the external UTC time.
    #[prost(bool, tag="2")]
    pub clock_not_synchronized: bool,
    /// If true, the value in 'P_Timestamp.SecondSinceEpoch' contains all leap seconds occurred.
    /// Otherwise, it does not take into account the leap seconds that occurred before the initialization of
    /// the time source of the device. Instead, the seconds since start of the epoch are calculated from the
    /// current date assuming a constant day length of 86 400 seconds. Note: If a UTC time master clock is
    /// used and accessible, this value should always be true.
    #[prost(bool, tag="3")]
    pub leap_seconds_known: bool,
    /// Information about the quality of the time source of the sending IED.
    #[prost(enumeration="TimeAccuracyKind", tag="4")]
    pub time_accuracy: i32,
}
/// UTC time with the epoch of midnight (00:00:00) of 1970-01-01. The presentation is defined in the
/// SCSMs.The NULL time stamp has all fields set to 0 (zero).The relation between a timestamp value, the
/// synchronization of an internal time with an external time source (for example, UTC time), and other
/// information related to time model are available as requirements in Clause 21.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Timestamp {
    /// Second since epoch (1970-01-01T00:00:00Z)
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// IEC61850 time quality
    #[prost(message, optional, tag="3")]
    pub tq: ::core::option::Option<TimeQuality>,
    /// Partial (sub) second expressed in nanoseconds (10<sup>-9</sup> second).
    #[prost(uint32, tag="4")]
    pub nanoseconds: u32,
}
/// Measured value (MV)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mv {
    /// Value of the magnitude based on a deadband calculation from the instantaneous value 'instMag'.
    /// The value of 'mag' shall be updated to the current instantaneous value 'instMag' when the value has
    /// changed according to the configuration parameter 'db'. If 'db'=0, 'mag'='instMag'.NOTE 1 This value
    /// is typically used to create reports for analogue values. Such a report sent "by exception" is not
    /// comparable to the transfer of sampled measured values as supported by the CDC SAV.NOTE 2 This 'mag'
    /// is not the same as 'mag' of the constructed attribute class 'Vector'.
    #[prost(double, tag="1")]
    pub mag: f64,
    /// Quality of the values in 'instMag', 'mag', 'range'.
    #[prost(message, optional, tag="2")]
    pub q: ::core::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'mag' or of the last change of the value in any of
    /// 'range' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
    /// Unit for: 'instMag', 'mag', 'subMag', 'rangeC'.
    #[prost(message, optional, tag="4")]
    pub units: ::core::option::Option<Unit>,
}
/// IEC61850 logical node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalNode {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalogEventAndStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Generic analogue input <i>n</i>.
    #[prost(message, optional, tag="2")]
    pub an_in: ::core::option::Option<Mv>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::core::option::Option<OptionalPhaseCodeKind>,
}
/// This is a root class similar to IdentifiedObject but without the mRID. The reason to separate
/// the two classes is because the mRID may need to be defined as a separate key field for technology
/// such as the DDS implementation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedObject {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The name is any free human readable and possibly non unique text naming the object.
    #[prost(message, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// The parts of a power system that are physical devices, electronic or mechanical.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationSystem {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub named_object: ::core::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub m_rid: ::prost::alloc::string::String,
}
/// Analogue setting (FC=SP) (ASG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asg {
    /// The value of the analogue setting.
    #[prost(double, tag="1")]
    pub set_mag: f64,
}
/// Binary counter reading (BCR)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bcr {
    /// Binary counter status represented as an integer value; wraps to 0 at the maximum or minimum
    /// value of INT64.
    #[prost(int64, tag="1")]
    pub act_val: i64,
    /// Quality of the values in 'actVal', 'frVal'.
    #[prost(message, optional, tag="2")]
    pub q: ::core::option::Option<Quality>,
    /// Timestamp of the last change of value in 'actVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// Specialized 61850 SPS class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusSps {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="2")]
    pub st_val: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanEventAndStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// If true, indication <i>n</i> is present.
    #[prost(message, optional, tag="2")]
    pub ind: ::core::option::Option<StatusSps>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::core::option::Option<OptionalPhaseCodeKind>,
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub message_time_stamp: ::core::option::Option<Timestamp>,
}
/// Generic capability message info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapabilityMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// ESS inverter high level function to reduce (smooth) charging or discharging rate of change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityFirming {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub capacity_firming_ctl: ::core::option::Option<bool>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="2")]
    pub limit_negative_dp_dt: ::core::option::Option<f32>,
    /// uint/1kW/min  If the supervised power increases at a rate higher that the rate defined by these
    /// limits, the ESS will discharge/charge at an opposite dp/dt to reduce (smooth) the rate of change at
    /// the PCC
    #[prost(message, optional, tag="3")]
    pub limit_positive_dp_dt: ::core::option::Option<f32>,
}
/// IEC61850-7-2 Service parameter for conditions checking
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConditions {
    /// InterlockCheck is used for the device to be controlled to check if other devices are in proper
    /// state for the control command.  One example is that 2 circuit breakers on a busbar need to be
    /// interlocked so one is open and one is closed, but not both on.
    #[prost(message, optional, tag="1")]
    pub interlock_check: ::core::option::Option<bool>,
    /// To check if a device is synchrous to the system.
    #[prost(message, optional, tag="2")]
    pub synchro_check: ::core::option::Option<bool>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearingTime {
    /// MISSING DOCUMENTATION!!!
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// Partial (sub) second expressed in nanoseconds (10<sup>-9</sup> second).
    #[prost(uint32, tag="3")]
    pub nanoseconds: u32,
}
/// Vector definition (Vector)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector {
    /// (range=\[-180...180\]) Angle of the complex value (Unit.SIUnit='deg' and Unit.multiplier='');
    /// angle reference is defined in the context where this type is used.
    #[prost(message, optional, tag="1")]
    pub ang: ::core::option::Option<f64>,
    /// Magnitude of the complex value.
    #[prost(double, tag="2")]
    pub mag: f64,
}
/// Complex measured value (CMV)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cmv {
    /// Complex value based on a deadband calculation from the instantaneous value 'instCVal.mag'. The
    /// deadband calculation is done both on 'instCVal.mag' (based on 'db') and on 'instCVal.ang' (based on
    /// 'dbAng'), independently. See  'MV.mag'.
    #[prost(message, optional, tag="1")]
    pub c_val: ::core::option::Option<Vector>,
    /// Quality of the values in 'instCVal', 'cVal', 'range', ‘rangeAng’.
    #[prost(message, optional, tag="2")]
    pub q: ::core::option::Option<Quality>,
    /// Timestamp of the last refresh of the value in 'cVal' or of the last change of the value in any
    /// of 'range', 'rangeAng' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// Asset representation of a ConductingEquipment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConductingEquipment {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub named_object: ::core::option::Option<NamedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub m_rid: ::prost::alloc::string::String,
}
/// An AC electrical connection point to a piece of conducting equipment. Terminals are connected at
/// physical connection points called connectivity nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Terminal {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub a_cdc_terminal: ::core::option::Option<AcdcTerminal>,
    /// Represents the normal network phasing condition. If the attribute is missing three phases (ABC
    /// or ABCN) shall be assumed.
    #[prost(message, optional, tag="2")]
    pub phases: ::core::option::Option<OptionalPhaseCodeKind>,
}
/// Reading associated with an equipment such as a recloser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConductingEquipmentTerminalReading {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub terminal: ::core::option::Option<Terminal>,
}
/// <<statistics>> Controllable analogue process value (APC)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlApc {
    /// Service parameter that determines the control activity.
    #[prost(double, tag="1")]
    pub ctl_val: f64,
}
/// Specialized DPC 61850 CDC class  Because objects in OpenFMB are optional fields, OpenFMB has
/// elected to send a "True" or "False" control state when a DPC state is sent. In a poll-based system,
/// the DPC will have two Booleans for "close" and "open", allowing a 'no-op' state if neither are true,
/// and a winning state (usually open) if both are true. OpenFMB simply elects to not populate the
/// control when no op is required. All state being sent will either be commanding a close or open for
/// the PhaseDPS &amp; StatusDPS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlDpc {
    /// Service parameter that determines the control activity ('false' for off, 'true' for on).
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
}
/// UTC time with the epoch of midnight (00:00:00) of 1970-01-01. The presentation is defined in the
/// SCSMs.The NULL time stamp has all fields set to 0 (zero).The relation between a timestamp value, the
/// synchronization of an internal time with an external time source (for example, UTC time), and other
/// information related to time model are available as requirements in Clause 21.  ControlTimestamp is a
/// timestamp for future time point so it does not contain the time quality as the one contained in the
/// normal Timestamp data type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlTimestamp {
    /// Second since epoch (1970-01-01T00:00:00Z)
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// Partial (sub) second expressed in nanoseconds (10<sup>-9</sup> second).
    #[prost(uint32, tag="3")]
    pub nanoseconds: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalScheduleParameterKind {
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub value: i32,
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngScheduleParameter {
    /// Schedule parameter type
    #[prost(enumeration="ScheduleParameterKind", tag="1")]
    pub schedule_parameter_type: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(double, tag="2")]
    pub value: f64,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulePoint {
    /// Schedule parameter
    #[prost(message, repeated, tag="1")]
    pub schedule_parameter: ::prost::alloc::vec::Vec<EngScheduleParameter>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleCsg {
    /// The array with the points specifying a time schedule
    #[prost(message, repeated, tag="1")]
    pub sch_pts: ::prost::alloc::vec::Vec<SchedulePoint>,
}
/// OpenFMB specialization for control schedule using:  LN: Schedule   Name: FSCH
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlScheduleFsch {
    /// Analog CSG
    #[prost(message, optional, tag="1")]
    pub val_acsg: ::core::option::Option<ScheduleCsg>,
}
/// OpenFMB specialization for logical node control
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalNodeForControl {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
}
/// LN: Schedule controller   Name: FSCC  F:    Function (generic) SC:  Schedule Controller C:   
/// Control (execution)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlFscc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub control_schedule_fsch: ::core::option::Option<ControlScheduleFsch>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub island_control_schedule_fsch: ::core::option::Option<ControlScheduleFsch>,
}
/// &lt;&lt;statistics&gt;&gt; Controllable integer status (INC)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlInc {
    /// Service parameter that determines the control activity.
    #[prost(int32, tag="1")]
    pub ctl_val: i32,
}
/// Integer status setting (FC=SP) (ING_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlIng {
    /// The value of the status setting.
    #[prost(int32, tag="1")]
    pub set_val: i32,
    /// Unit for 'setVal', 'minVal', 'maxVal', 'stepSize'.
    #[prost(message, optional, tag="2")]
    pub units: ::core::option::Option<Unit>,
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlIsc {
    /// Service parameter that determines the control activity.
    #[prost(int32, tag="1")]
    pub ctl_val: i32,
}
/// Generic control message info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// Controllable single point (SPC)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlSpc {
    /// Service parameter that determines the control activity ('false' for off or deactivation, 'true'
    /// for on or activation).
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
}
/// The value of a control command which could either be a setpoint or a control schedule in curve. 
/// The attribute modBlk is used to tag out a device. if it is TRUE, any setpoints and control schedule
/// in a message payload should be ignored.   It should also be presented in a status profile.  Any
/// modBlk value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ControlValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// The attribute modBlk is used to tag out a device. If it is TRUE, any setpoints and control in a
    /// message payload should be ignored.   It should also be presented in a status profile.  Any modBlk
    /// value change (i.e. TRUE to FALSE and vice versa) should trigger an event.
    #[prost(message, optional, tag="3")]
    pub mod_blk: ::core::option::Option<bool>,
    /// If true, reset the device before executing any other controls.
    #[prost(message, optional, tag="4")]
    pub reset: ::core::option::Option<bool>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CumulativeTime {
    /// MISSING DOCUMENTATION!!!
    #[prost(uint64, tag="2")]
    pub seconds: u64,
    /// Partial (sub) second expressed in nanoseconds (10<sup>-9</sup> second).
    #[prost(uint32, tag="3")]
    pub nanoseconds: u32,
}
/// Interval between two date and time points.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateTimeInterval {
    /// End date and time of this interval.
    #[prost(message, optional, tag="1")]
    pub end: ::core::option::Option<i64>,
    /// Start date and time of this interval.
    #[prost(message, optional, tag="2")]
    pub start: ::core::option::Option<i64>,
}
/// Phase to phase related measured values of a three-phase system (DEL)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Del {
    /// Value of phase A to phase B measurement.
    #[prost(message, optional, tag="1")]
    pub phs_ab: ::core::option::Option<Cmv>,
    /// Value of phase B to phase C measurement.
    #[prost(message, optional, tag="2")]
    pub phs_bc: ::core::option::Option<Cmv>,
    /// Value of phase C to phase A measurement.
    #[prost(message, optional, tag="3")]
    pub phs_ca: ::core::option::Option<Cmv>,
}
/// [OpenFMB CDC extension] Per Phase DPC. Because objects in OpenFMB are optional fields, OpenFMB
/// has elected to send a "True" or "False" control state when a DPC state is sent. In a poll-based
/// system, the DPC will have two Booleans for "close" and "open", allowing a 'no-op' state if neither
/// are true, and a winning state (usually open) if both are true. OpenFMB simply elects to not populate
/// the control when no op is required. All state being sent will either be commanding a close or open
/// for the PhaseDPS &amp; StatusDPS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseDpc {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<ControlDpc>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<ControlDpc>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<ControlDpc>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<ControlDpc>,
}
/// Reclose enabled
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscreteControlXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_control: ::core::option::Option<LogicalNodeForControl>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub pos: ::core::option::Option<PhaseDpc>,
    /// Protection mode such as a group setting or pre-defined curve profile. It is usually pre-defined
    /// by a circuit segment service.
    #[prost(message, optional, tag="3")]
    pub protection_mode: ::core::option::Option<ControlInc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub reclose_enabled: ::core::option::Option<ControlSpc>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub reset_protection_pickup: ::core::option::Option<ControlSpc>,
}
/// Generic user of energy - a  point of consumption on the power system model.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnergyConsumer {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<ConductingEquipment>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub operating_limit: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalCalcMethodKind {
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub value: i32,
}
/// Calc method kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngCalcMethodKind {
    /// The value of the status setting.
    #[prost(enumeration="CalcMethodKind", tag="1")]
    pub set_val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalGridConnectModeKind {
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub value: i32,
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngGridConnectModeKind {
    /// The value of the status setting.
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub set_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub set_val_extension: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalPfSignKind {
    #[prost(enumeration="PfSignKind", tag="1")]
    pub value: i32,
}
/// Enumerated status setting (FC=SP) (ENG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngPfSignKind {
    /// The value of the status setting.
    #[prost(enumeration="PfSignKind", tag="1")]
    pub set_val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalBehaviourModeKind {
    #[prost(enumeration="BehaviourModeKind", tag="1")]
    pub value: i32,
}
/// Behavior mode kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsBehaviourModeKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="BehaviourModeKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalDerGeneratorStateKind {
    #[prost(enumeration="DerGeneratorStateKind", tag="1")]
    pub value: i32,
}
/// DER generation state kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsDerGeneratorStateKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="DerGeneratorStateKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalDynamicTestKind {
    #[prost(enumeration="DynamicTestKind", tag="1")]
    pub value: i32,
}
/// Dynamic test kind. ENS stands for Enumerated status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsDynamicTestKind {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(enumeration="DynamicTestKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// Grid connect event &amp; status mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsGridConnectModeKind {
    /// Actual Grid Connection Mode
    #[prost(enumeration="GridConnectModeKind", tag="1")]
    pub st_val: i32,
    /// MISSING DOCUMENTATION!!!
    #[prost(string, tag="2")]
    pub st_val_extension: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalHealthKind {
    #[prost(enumeration="HealthKind", tag="1")]
    pub value: i32,
}
/// &lt;&gt; Enumerated status (ENS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsHealthKind {
    /// Textual description of the data. In case it is used within the CDC LPL, the description refers
    /// to the logical node.
    #[prost(message, optional, tag="1")]
    pub d: ::core::option::Option<::prost::alloc::string::String>,
    /// Value of the data.
    #[prost(enumeration="HealthKind", tag="2")]
    pub st_val: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalSwitchingCapabilityKind {
    #[prost(enumeration="SwitchingCapabilityKind", tag="1")]
    pub value: i32,
}
/// <<abstract>> Enumerated status (ENS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnsSwitchingCapabilityKind {
    /// If true, 'q.operatorBlocked'=true, and the process value is no longer updated.
    #[prost(message, optional, tag="1")]
    pub blk_ena: ::core::option::Option<bool>,
    /// Value of the data.
    #[prost(enumeration="SwitchingCapabilityKind", tag="2")]
    pub st_val: i32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDcte {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub rnd_dl_tmms: ::core::option::Option<ControlIng>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub rtn_dl_tmms: ::core::option::Option<ControlIng>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub rtn_rmp_tmms: ::core::option::Option<ControlIng>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnterServiceApc {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub enter_service_parameter: ::core::option::Option<OperationDcte>,
    /// This field is for an absolute value. ES Frequency High
    #[prost(float, tag="2")]
    pub hz_hi_lim: f32,
    /// This field is for an absolute value. ES Frequency Low
    #[prost(float, tag="3")]
    pub hz_lo_lim: f32,
    /// Permit service. If (RtnSrvAuto) true, the DER is authorized to automatically return to service;
    /// if false, the DER must wait until an external RtnSrvAuth is received to allow it to return to
    /// service.
    #[prost(bool, tag="4")]
    pub rtn_srv_auto: bool,
    /// This is an absolute value field.  ES Voltage High
    #[prost(float, tag="5")]
    pub v_hi_lim: f32,
    /// This is an absolute value field.  ES Voltage Low
    #[prost(float, tag="6")]
    pub v_lo_lim: f32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ess {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<ConductingEquipment>,
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// Event value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub mod_blk: ::core::option::Option<bool>,
}
/// The source where a forecast value is issued.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastValueSource {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
}
/// Intelligent Electronic Device is a device with a microprocessor that can contain one or more
/// (IEC61850) SERVERs. In the context of IEC61850, IED could be an electronic protection device, a
/// controller or even a laptop/desktop computer. <b>Modelling note</b>: This class is not explicitly
/// defined in IEC61850-7-2 (but only in SCL: IEC61850-6). However, it is an important concept that
/// deserves its place in the meta-model. When the meta-model gets instantiated from a direct link to an
/// IED with an IEC61850 SERVER, we typically create an instance of the meta-model IED per connection.
/// When the meta-model gets instantiated from an SCL file, there is the full description of IED and its
/// functions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastIed {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub forecast_value_source: ::core::option::Option<ForecastValueSource>,
    /// For control, this is an application ID, unique within communication system, and if the message
    /// is transformed between gateway the original source application ID should be kept.
    #[prost(string, tag="2")]
    pub source_application_id: ::prost::alloc::string::String,
    /// Message publication date time
    #[prost(int64, tag="3")]
    pub source_date_time: i64,
}
/// Forecast value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
}
/// ESS inverter high level function to maintain frequency within dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyRegulation {
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="1")]
    pub frequency_dead_band_minus: ::core::option::Option<f32>,
    /// uint/0.01Hz  Frequency regulation is performed when the grid frequency goes beyond the dead
    /// bands. The dead bands are defined as follows: Upper DB = frequency set point + dead band plus Lower
    /// DB = frequency set point – dead band minus
    #[prost(message, optional, tag="2")]
    pub frequency_dead_band_plus: ::core::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub frequency_regulation_ctl: ::core::option::Option<bool>,
    /// uint/0.01Hz  Target frequency
    #[prost(message, optional, tag="4")]
    pub frequency_set_point: ::core::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
    pub grid_frequency_stable_band_minus: ::core::option::Option<f32>,
    /// uint/0.01Hz  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="6")]
    pub grid_frequency_stable_band_plus: ::core::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over frequency events. A droop of
    /// 1% means that the PCS will output 100% power if the frequency is 1% of the nominal frequency away
    /// from the upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="7")]
    pub over_frequency_droop: ::core::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="8")]
    pub under_frequency_droop: ::core::option::Option<f32>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDhfw {
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// Open Loop Response Time
    #[prost(message, optional, tag="2")]
    pub opl_tmms_max: ::core::option::Option<ClearingTime>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDlfw {
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub opl_tmms_max: ::core::option::Option<ClearingTime>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HzWPoint {
    /// This is an absolute value field
    #[prost(float, tag="1")]
    pub deadband_hz_val: f32,
    /// MISSING DOCUMENTATION!!!
    #[prost(float, tag="2")]
    pub slope_val: f32,
}
/// Frequency Droop Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HzWapc {
    /// Overfrequency Droop <i>db</i>OF  The frequency which must be higher than the reference frequency
    /// at which to start reducing power output.
    #[prost(message, optional, tag="1")]
    pub over_hz_w_pt: ::core::option::Option<HzWPoint>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub over_hz_w_parameter: ::core::option::Option<OperationDhfw>,
    /// Underfrequency Droop <i>db</i>UF  The frequency which must be lower than the reference frequency
    /// at which to start increasing power output.
    #[prost(message, optional, tag="3")]
    pub under_hz_w_pt: ::core::option::Option<HzWPoint>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub under_hz_w_parameter: ::core::option::Option<OperationDlfw>,
}
/// <<statistics>> Integer status (INS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusIns {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// Status expressed in integer based on IEC61850 GGIO.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntegerEventAndStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Generic integer status input <i>n</i>.
    #[prost(message, optional, tag="2")]
    pub int_in: ::core::option::Option<StatusIns>,
    /// Phase code
    #[prost(message, optional, tag="3")]
    pub phase: ::core::option::Option<OptionalPhaseCodeKind>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDwmx {
    /// Limit Active Power Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDwmn {
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LimitWapc {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub max_lim_parameter: ::core::option::Option<OperationDwmx>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub min_lim_parameter: ::core::option::Option<OperationDwmn>,
    /// This is an absolute value field.  Maximum Active Power
    #[prost(float, tag="3")]
    pub w_max_spt_val: f32,
    /// This is an absolute value field.  Minimum Active Power
    #[prost(float, tag="4")]
    pub w_min_spt_val: f32,
}
/// Logical node for event and status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalNodeForEventAndStatus {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Behavior of the function
    #[prost(message, optional, tag="2")]
    pub beh: ::core::option::Option<EnsBehaviourModeKind>,
    /// Asset health
    #[prost(message, optional, tag="3")]
    pub ee_health: ::core::option::Option<EnsHealthKind>,
    /// Hot line tag.
    #[prost(message, optional, tag="4")]
    pub hot_line_tag: ::core::option::Option<StatusSps>,
    /// Remote control block.
    #[prost(message, optional, tag="5")]
    pub remote_blk: ::core::option::Option<StatusSps>,
}
/// The current state for a measurement. A state value is an instance of a measurement from a
/// specific source. Measurements can be associated with many state values, each representing a
/// different source for the measurement.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeasurementValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
}
/// Physical asset that performs the metering role of the usage point. Used for measuring
/// consumption and detection of events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meter {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<ConductingEquipment>,
}
/// Generic nameplate value such as name and description.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NameplateValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// Model
    #[prost(message, optional, tag="2")]
    pub model: ::core::option::Option<::prost::alloc::string::String>,
    /// Serial Number
    #[prost(message, optional, tag="3")]
    pub sernum: ::core::option::Option<::prost::alloc::string::String>,
    /// Version
    #[prost(message, optional, tag="4")]
    pub sw_rev: ::core::option::Option<::prost::alloc::string::String>,
    /// Manufacturer
    #[prost(message, optional, tag="5")]
    pub vendor: ::core::option::Option<::prost::alloc::string::String>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDfpf {
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// Constant power factor excitation setting  PFExtSet: PFExtSet set to true = overexcited; PFExtSet
    /// set to false = underexcited
    #[prost(bool, tag="2")]
    pub p_f_ext_set: bool,
    /// Applies when generating.  Target power factor setting when generating.  The power factor target
    /// is a number between -1 and 1, and is used in conjunction with PFExtSet to indicate whether it to
    /// make it over or under excited.
    #[prost(float, tag="3")]
    pub p_f_gn_tgt_mx_val: f32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDvar {
    /// This is an absolute value field.  Constant Reactive Power
    #[prost(float, tag="1")]
    pub var_tgt_spt: f32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDvvr {
    /// Voltage-Reactive Power Mode Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// Open Loop Response Time
    #[prost(message, optional, tag="2")]
    pub opl_tmms_max: ::core::option::Option<ClearingTime>,
    /// V<sub>Ref</sub> Reference voltage
    #[prost(float, tag="3")]
    pub v_ref: f32,
    /// Autonomous <i>V</i><i><sub>Ref</sub></i><i>  </i>adjustment enable
    #[prost(bool, tag="4")]
    pub v_ref_adj_ena: bool,
    /// <i>V</i><i><sub>Ref</sub></i><i>  </i>adjustment time constant
    #[prost(message, optional, tag="5")]
    pub v_ref_tmms: ::core::option::Option<ControlIng>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDvwc {
    /// Voltage-Active Power Mode Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// Open Loop Response Time
    #[prost(message, optional, tag="2")]
    pub opl_tmms_max: ::core::option::Option<ClearingTime>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDwgc {
    /// Active power setpoint. Its mxVal attribute reflects the value of the setpoint that is requested.
    #[prost(float, tag="1")]
    pub w_spt: f32,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationDwvr {
    /// Active-Reactive Power Mode Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
}
/// Generic event message information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizationMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// ESS inverter high level function to maintain power level by charging or discharging
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeakShaving {
    /// uint/1kW  If the supervised power goes below this limit, the ESS will charge to maintain this limit.
    #[prost(message, optional, tag="1")]
    pub base_shaving_limit: ::core::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="2")]
    pub peak_shaving_ctl: ::core::option::Option<bool>,
    /// uint/1kW  If the supervised power goes above this limit, the ESS will discharge to maintain this
    /// limit.
    #[prost(message, optional, tag="3")]
    pub peak_shaving_limit: ::core::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="4")]
    pub soc_management_allowed_high_limit: ::core::option::Option<f32>,
    /// uint/1kW  If the supervised power is between the band defined by these two limits then SOC
    /// management is allowed.
    #[prost(message, optional, tag="5")]
    pub soc_management_allowed_low_limit: ::core::option::Option<f32>,
}
/// Constant power factor control function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pfspc {
    /// MISSING DOCUMENTATION!!!
    #[prost(bool, tag="1")]
    pub ctl_val: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub p_f_parameter: ::core::option::Option<OperationDfpf>,
}
/// [OpenFMB CDC extension] Per Phase ISC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseApc {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<ControlApc>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<ControlApc>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<ControlApc>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<ControlApc>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalDbPosKind {
    #[prost(enumeration="DbPosKind", tag="1")]
    pub value: i32,
}
/// Specialized 61850 DPS class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusDps {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Status value of the controllable data object.
    #[prost(enumeration="DbPosKind", tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// [OpenFMB CDC extension] Per Phase DPS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseDps {
    /// 3 Phase status.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<StatusDps>,
    /// Phase A status.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<StatusDps>,
    /// Phase B status.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<StatusDps>,
    /// Phase C status.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<StatusDps>,
}
/// [OpenFMB CDC extension] Per Phase INS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseIns {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<StatusIns>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<StatusIns>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<StatusIns>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<StatusIns>,
}
/// [OpenFMB CDC extension] Per Phase ISC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseIsc {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<ControlIsc>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<ControlIsc>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<ControlIsc>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<ControlIsc>,
}
/// Specialized 61850 MMTN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMmtn {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Apparent energy demand (direction: from busbar).
    #[prost(message, optional, tag="2")]
    pub dmd_v_ah: ::core::option::Option<Bcr>,
    /// Reactive energy demand (direction: from busbar).
    #[prost(message, optional, tag="3")]
    pub dmd_v_arh: ::core::option::Option<Bcr>,
    /// Real energy demand (direction: from busbar).
    #[prost(message, optional, tag="4")]
    pub dmd_wh: ::core::option::Option<Bcr>,
    /// Apparent energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="5")]
    pub sup_v_ah: ::core::option::Option<Bcr>,
    /// Reactive energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="6")]
    pub sup_v_arh: ::core::option::Option<Bcr>,
    /// Real energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="7")]
    pub sup_wh: ::core::option::Option<Bcr>,
    /// Net apparent energy since last reset.
    #[prost(message, optional, tag="8")]
    pub tot_v_ah: ::core::option::Option<Bcr>,
    /// Net reactive energy since last reset.
    #[prost(message, optional, tag="9")]
    pub tot_v_arh: ::core::option::Option<Bcr>,
    /// Net real energy since last reset.
    #[prost(message, optional, tag="10")]
    pub tot_wh: ::core::option::Option<Bcr>,
}
/// Specialized 61850 MMTN LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseMmtn {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub phs_a: ::core::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub phs_ab: ::core::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub phs_bc: ::core::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="5")]
    pub phs_c: ::core::option::Option<ReadingMmtn>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="6")]
    pub phs_ca: ::core::option::Option<ReadingMmtn>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalRecloseActionKind {
    #[prost(enumeration="RecloseActionKind", tag="1")]
    pub value: i32,
}
/// [OpenFMB CDC extension] Per Phase reclose action kind.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseRecloseAction {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<OptionalRecloseActionKind>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<OptionalRecloseActionKind>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<OptionalRecloseActionKind>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<OptionalRecloseActionKind>,
}
/// [OpenFMB CDC extension] Per Phase SPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseSpc {
    /// 3 Phase control.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<ControlSpc>,
    /// Phase A control.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<ControlSpc>,
    /// Phase B control.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<ControlSpc>,
    /// Phase C control.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<ControlSpc>,
}
/// [OpenFMB CDC extension] Per Phase DPS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PhaseSps {
    /// 3 Phase status.
    #[prost(message, optional, tag="1")]
    pub phs3: ::core::option::Option<StatusSps>,
    /// Phase A status.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<StatusSps>,
    /// Phase B status.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<StatusSps>,
    /// Phase C status.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<StatusSps>,
}
/// [OpenFMB CDC extension] Phase magnitude (PMG). Phase to ground/neutral related per-phase
/// measured values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pmg {
    /// Net current, as the algebraic sum of the instantaneous values of currents flowing through all
    /// live conductors and the neutral of a circuit at one point of the electrical installation ('phsA
    /// instCVal'+'phsB.instCVal'+'phsC.instCVal'+'neut.instCVal').
    #[prost(message, optional, tag="1")]
    pub net: ::core::option::Option<Mv>,
    /// Value of phase A.
    #[prost(message, optional, tag="2")]
    pub phs_a: ::core::option::Option<Mv>,
    /// Value of phase B.
    #[prost(message, optional, tag="3")]
    pub phs_b: ::core::option::Option<Mv>,
    /// Value of phase C.
    #[prost(message, optional, tag="4")]
    pub phs_c: ::core::option::Option<Mv>,
}
/// Grid connect mode kind
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RampRate {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub negative_reactive_power_kv_ar_per_min: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub negative_real_power_kw_per_min: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub positive_reactive_power_kv_ar_per_min: ::core::option::Option<f32>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="4")]
    pub positive_real_power_kw_per_min: ::core::option::Option<f32>,
}
/// Specialized 61850 MENV LN class  LN: Environmental information   Name: MENV
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMenv {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Emission level of CO2.
    #[prost(message, optional, tag="2")]
    pub co2_em: ::core::option::Option<Mv>,
    /// Emission level of CO.
    #[prost(message, optional, tag="3")]
    pub co_em: ::core::option::Option<Mv>,
    /// Emission level of NO<sub>x</sub>.
    #[prost(message, optional, tag="4")]
    pub n_ox_em: ::core::option::Option<Mv>,
    /// Emission level of SO<sub>x</sub>.
    #[prost(message, optional, tag="5")]
    pub s_ox_em: ::core::option::Option<Mv>,
    /// Amount of dust particles suspended in air.
    #[prost(message, optional, tag="6")]
    pub dust: ::core::option::Option<Mv>,
    /// Sound pressure level.
    #[prost(message, optional, tag="7")]
    pub snd: ::core::option::Option<Mv>,
    /// Amount of oxygen in combustion gases.
    #[prost(message, optional, tag="8")]
    pub o2_cmbu_gas: ::core::option::Option<Mv>,
    /// Amount of ozone in the air.
    #[prost(message, optional, tag="9")]
    pub o3_air: ::core::option::Option<Mv>,
}
/// Generic reading message information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// Specialized 61850 MMDC LN: DC measurement   Name: MMDC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMmdc {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// DC power.
    #[prost(message, optional, tag="2")]
    pub watt: ::core::option::Option<Mv>,
    /// DC current.
    #[prost(message, optional, tag="3")]
    pub amp: ::core::option::Option<Mv>,
    /// DC voltage.
    #[prost(message, optional, tag="4")]
    pub vol: ::core::option::Option<Mv>,
    /// DC voltage between positive pole and earth.
    #[prost(message, optional, tag="5")]
    pub vol_ps_gnd: ::core::option::Option<Mv>,
    /// DC voltage between negative pole and earth.
    #[prost(message, optional, tag="6")]
    pub vol_ng_gnd: ::core::option::Option<Mv>,
    /// Resistance in DC circuit.
    #[prost(message, optional, tag="7")]
    pub ris: ::core::option::Option<Mv>,
    /// DC resistance between positive pole and earth.
    #[prost(message, optional, tag="8")]
    pub ris_ps_gnd: ::core::option::Option<Mv>,
    /// DC resistance between negative pole and earth.
    #[prost(message, optional, tag="9")]
    pub ris_ng_gnd: ::core::option::Option<Mv>,
}
/// LN: Meteorological information   Name: MMET
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMmet {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Temperature of environment \[&#176;C\].
    #[prost(message, optional, tag="2")]
    pub env_tmp: ::core::option::Option<Mv>,
    /// Wet bulb temperature (typically in &#176;C).
    #[prost(message, optional, tag="3")]
    pub wet_blb_tmp: ::core::option::Option<Mv>,
    /// Cloud cover level.
    #[prost(message, optional, tag="4")]
    pub cloud_cvr: ::core::option::Option<Mv>,
    /// Humidity of environment (typically in %).
    #[prost(message, optional, tag="5")]
    pub env_hum: ::core::option::Option<Mv>,
    /// Dew point.
    #[prost(message, optional, tag="6")]
    pub dew_pt: ::core::option::Option<Mv>,
    /// Diffuse insolation (insolation SIUnit \[W/m<sup>2</sup>\]).
    #[prost(message, optional, tag="7")]
    pub dff_insol: ::core::option::Option<Mv>,
    /// Direct normal insolation (insolation SIUnit \[W/m<sup>2</sup>\]).
    #[prost(message, optional, tag="8")]
    pub dct_insol: ::core::option::Option<Mv>,
    /// Daylight duration (time elapsed between sunrise and sunset).
    #[prost(message, optional, tag="9")]
    pub dl_dur: ::core::option::Option<Mv>,
    /// Total horizontal insolation (insolation SIUnit \[W/m<sup>2</sup>\]).
    #[prost(message, optional, tag="10")]
    pub hor_insol: ::core::option::Option<Mv>,
    /// Total horizontal wind direction.
    #[prost(message, optional, tag="11")]
    pub hor_wd_dir: ::core::option::Option<Mv>,
    /// Average horizontal wind speed.
    #[prost(message, optional, tag="12")]
    pub hor_wd_spd: ::core::option::Option<Mv>,
    /// Vertical wind direction.
    #[prost(message, optional, tag="13")]
    pub ver_wd_dir: ::core::option::Option<Mv>,
    /// Average vertical wind speed.
    #[prost(message, optional, tag="14")]
    pub ver_wd_spd: ::core::option::Option<Mv>,
    /// Maximum wind gust speed.
    #[prost(message, optional, tag="15")]
    pub wd_gust_spd: ::core::option::Option<Mv>,
    /// Barometric pressure of environment.
    #[prost(message, optional, tag="16")]
    pub env_pres: ::core::option::Option<Mv>,
    /// Rainfall (typically in mm - length SIUnit \[m\]).
    #[prost(message, optional, tag="17")]
    pub rn_fll: ::core::option::Option<Mv>,
    /// Snowfall density (typically in g/cm<sup>3</sup> - density SIUnit \[kg/m<sup>3</sup>\]).
    #[prost(message, optional, tag="18")]
    pub snw_den: ::core::option::Option<Mv>,
    /// Snowfall temperature (typically in &#176;C).
    #[prost(message, optional, tag="19")]
    pub snw_tmp: ::core::option::Option<Mv>,
    /// Snow cover (typically in mm - length SIUnit \[m\]).
    #[prost(message, optional, tag="20")]
    pub snw_cvr: ::core::option::Option<Mv>,
    /// Snowfall (typically in mm - length SIUnit \[m\]).
    #[prost(message, optional, tag="21")]
    pub snw_fll: ::core::option::Option<Mv>,
    /// Water equivalent of snowfall (typically in mm - length SIUnit \[m\]).
    #[prost(message, optional, tag="22")]
    pub snw_eq: ::core::option::Option<Mv>,
}
/// Specialized 61850 MMTR class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMmtr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Apparent energy demand (direction: from busbar).
    #[prost(message, optional, tag="2")]
    pub dmd_v_ah: ::core::option::Option<Bcr>,
    /// Reactive energy demand (direction: from busbar).
    #[prost(message, optional, tag="3")]
    pub dmd_v_arh: ::core::option::Option<Bcr>,
    /// Real energy demand (direction: from busbar).
    #[prost(message, optional, tag="4")]
    pub dmd_wh: ::core::option::Option<Bcr>,
    /// Apparent energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="5")]
    pub sup_v_ah: ::core::option::Option<Bcr>,
    /// Reactive energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="6")]
    pub sup_v_arh: ::core::option::Option<Bcr>,
    /// Real energy supply (default direction: towards busbar).
    #[prost(message, optional, tag="7")]
    pub sup_wh: ::core::option::Option<Bcr>,
    /// Net apparent energy since last reset.
    #[prost(message, optional, tag="8")]
    pub tot_v_ah: ::core::option::Option<Bcr>,
    /// Net reactive energy since last reset.
    #[prost(message, optional, tag="9")]
    pub tot_v_arh: ::core::option::Option<Bcr>,
    /// Net real energy since last reset.
    #[prost(message, optional, tag="10")]
    pub tot_wh: ::core::option::Option<Bcr>,
}
/// Phase to ground/neutral related measured values of a three-phase system (WYE)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wye {
    /// Net current, as the algebraic sum of the instantaneous values of currents flowing through all
    /// live conductors and the neutral of a circuit at one point of the electrical installation ('phsA
    /// instCVal'+'phsB.instCVal'+'phsC.instCVal'+'neut.instCVal').
    #[prost(message, optional, tag="1")]
    pub net: ::core::option::Option<Cmv>,
    /// Value of the measured phase neutral. If a direct measurement of this value is not available, it
    /// is acceptable to substitute an estimate computed by creating the algebraic sum of the instantaneous
    /// values of currents flowing through all live conductors ('phsA.instCVal'+'phsB.instCVal'+'phsC
    /// instCVal'); in that case, 'neut'='res'.
    #[prost(message, optional, tag="2")]
    pub neut: ::core::option::Option<Cmv>,
    /// Value of phase A.
    #[prost(message, optional, tag="3")]
    pub phs_a: ::core::option::Option<Cmv>,
    /// Value of phase B.
    #[prost(message, optional, tag="4")]
    pub phs_b: ::core::option::Option<Cmv>,
    /// Value of phase C.
    #[prost(message, optional, tag="5")]
    pub phs_c: ::core::option::Option<Cmv>,
    /// Residual current, as the algebraic sum of the instantaneous values of currents flowing through
    /// all live conductors of a circuit at one point of the electrical installation ('phsA.instCVal'+'phsB
    /// instCVal'+'phsC.instCVal').
    #[prost(message, optional, tag="6")]
    pub res: ::core::option::Option<Cmv>,
}
/// Specialized 61850 MMXU LN class
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadingMmxu {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Phase to ground/phase to neutral three phase currents.
    #[prost(message, optional, tag="2")]
    pub a: ::core::option::Option<Wye>,
    /// Kind of statistical calculation, specifying how the data attributes that represent analogue
    /// values have been calculated. The calculation method shall be the same for all data objects of the
    /// logical node instance.If the value is 'PEAK_FUNDAMENTAL', angle may be present in a data object of
    /// complex measured value type (CMV, such as in WYE, DEL, etc.), otherwise angle is not used (if
    /// ‘TRUE_RMS’ and ‘RMS_FUNDAMENTAL’).If the value is 'unspecified', the dependent data objects may be
    /// meaningless.
    #[prost(message, optional, tag="3")]
    pub clc_mth: ::core::option::Option<EngCalcMethodKind>,
    /// Frequency \[Hz\].
    #[prost(message, optional, tag="4")]
    pub hz: ::core::option::Option<Mv>,
    /// Phase to ground/phase to neutral power factors.The power factor is defined as P (active power) /
    /// S (apparent power), so the value range is 0...1. If current (I) and voltage (U) are sinusoidal and
    /// displaced by the angle phi, then the power factor is |cos phi|, again with the value range 0...1.
    /// Therefore, for the power factor per phase, value is contained in 'mag' and 'ang' is not used.
    #[prost(message, optional, tag="5")]
    pub pf: ::core::option::Option<Wye>,
    /// Sign convention for power factor 'PF' (and reactive power 'VAr').
    #[prost(message, optional, tag="6")]
    pub pf_sign: ::core::option::Option<EngPfSignKind>,
    /// Phase to ground (line) voltages.
    #[prost(message, optional, tag="7")]
    pub ph_v: ::core::option::Option<Wye>,
    /// Phase to phase voltages.
    #[prost(message, optional, tag="8")]
    pub ppv: ::core::option::Option<Del>,
    /// Phase to ground/phase to neutral apparent powers S.
    #[prost(message, optional, tag="9")]
    pub va: ::core::option::Option<Wye>,
    /// Phase to ground/phase to neutral reactive powers Q.
    #[prost(message, optional, tag="10")]
    pub v_ar: ::core::option::Option<Wye>,
    /// Phase to ground/phase to neutral real powers P.
    #[prost(message, optional, tag="11")]
    pub w: ::core::option::Option<Wye>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sensor {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub conducting_equipment: ::core::option::Option<ConductingEquipment>,
}
/// ESS inverter high level function to shut down ESS if SOC exceeds high or low limits.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocLimit {
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="1")]
    pub soc_high_limit: ::core::option::Option<f32>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="2")]
    pub soc_high_limit_hysteresis: ::core::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub soc_limit_ctl: ::core::option::Option<bool>,
    /// uint/1%  These limits define the operational range of the battery. If a lineup reaches the SOC
    /// high limit, the inverter’s output is reduced to 0. Charging is then blocked until the hysteresis is
    /// overcome. The same logic applies to the SOC low limit, except that after the ramp down is complete,
    /// discharging is blocked until the hysteresis is overcome.
    #[prost(message, optional, tag="4")]
    pub soc_low_limit: ::core::option::Option<f32>,
    /// uint/1%  These hysteresis define the release conditions for the block charge or discharge
    /// initiated by the SOC limits.For example, assume a SOC low limit of 10% and a SOC low limit
    /// hysteresis of 2% and that discharging is blocked because the batteries SOC reached the SOC low
    /// limit, discharging will only be allowed again after the battery’s SOC reaches 13%.
    #[prost(message, optional, tag="5")]
    pub soc_low_limit_hysteresis: ::core::option::Option<f32>,
}
/// ESS inverter high level function to maintain SOC within dead bands
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SocManagement {
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="1")]
    pub soc_dead_band_minus: ::core::option::Option<f32>,
    /// uint/1%  Define a dead band (DB) around the SOC set point. When the battery SOC goes outside the
    /// dead band, the SOC management executes and bring the SOC back to the set point. Upper DB = set point
    /// + dead band plus Lower DB = set point – dead band minus
    #[prost(message, optional, tag="2")]
    pub soc_dead_band_plus: ::core::option::Option<f32>,
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="3")]
    pub soc_management_ctl: ::core::option::Option<bool>,
    /// uint/1kW  Set point used for SOC maintenance
    #[prost(message, optional, tag="4")]
    pub soc_power_set_point: ::core::option::Option<f32>,
    /// uint/1%  SOC Target in percentage (%).
    #[prost(message, optional, tag="5")]
    pub soc_set_point: ::core::option::Option<f32>,
}
/// Source configured setting
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCapabilityConfiguration {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Amp Maximum configured value
    #[prost(message, optional, tag="2")]
    pub a_max: ::core::option::Option<Asg>,
    /// Apparent Power Maximum configured value
    #[prost(message, optional, tag="3")]
    pub va_max: ::core::option::Option<Asg>,
    /// Reactive Power Absorbed Maximum configured value
    #[prost(message, optional, tag="4")]
    pub var_max_abs: ::core::option::Option<Asg>,
    /// Reactive Power Injected Maximum configured value
    #[prost(message, optional, tag="5")]
    pub var_max_inj: ::core::option::Option<Asg>,
    /// AC voltage maximum configured value
    #[prost(message, optional, tag="6")]
    pub v_max: ::core::option::Option<Asg>,
    /// AC voltage minimum configured value
    #[prost(message, optional, tag="7")]
    pub v_min: ::core::option::Option<Asg>,
    /// AC voltage nominal configured value
    #[prost(message, optional, tag="8")]
    pub v_nom: ::core::option::Option<Asg>,
    /// Active Power Max configured value
    #[prost(message, optional, tag="9")]
    pub w_max: ::core::option::Option<Asg>,
    /// Active Power (Over-Excited) configured value
    #[prost(message, optional, tag="10")]
    pub w_ovr_ext: ::core::option::Option<Asg>,
    /// Active power configured value at specified over-excited power factor
    #[prost(message, optional, tag="11")]
    pub w_ovr_ext_pf: ::core::option::Option<Asg>,
    /// Active Power (Under-Excited) configured value
    #[prost(message, optional, tag="12")]
    pub w_und_ext: ::core::option::Option<Asg>,
    /// Active power configured value at specified under-excited power factor
    #[prost(message, optional, tag="13")]
    pub w_und_ext_pf: ::core::option::Option<Asg>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalNorOpCatKind {
    #[prost(enumeration="NorOpCatKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalAbnOpCatKind {
    #[prost(enumeration="AbnOpCatKind", tag="1")]
    pub value: i32,
}
/// Source capability ratings
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceCapabilityRatings {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Abnormal Operating Performance Category  Text stating: “IEEE 1547:2018 Either “I”, “II”, or
    /// “III” is stated for the Abnormal Category
    #[prost(enumeration="AbnOpCatKind", tag="2")]
    pub abn_op_cat_rtg: i32,
    /// Amp Maximum Rating  SunSpec has this field but not in IEEE1547
    #[prost(message, optional, tag="3")]
    pub a_max_rtg: ::core::option::Option<Asg>,
    /// Frequency nominal rating, default to 60 hz if not provide and cannot be configured.
    #[prost(message, optional, tag="4")]
    pub freq_nom_rtg: ::core::option::Option<Asg>,
    /// Normal operating performance category
    #[prost(enumeration="NorOpCatKind", tag="5")]
    pub nor_op_cat_rtg: i32,
    /// Reactive susceptance that remains connected to the Area EPS in the cease to energize and trip state
    #[prost(message, optional, tag="6")]
    pub react_suscept_rtg: ::core::option::Option<Asg>,
    /// Apparent Power Maximum Rating
    #[prost(message, optional, tag="7")]
    pub va_max_rtg: ::core::option::Option<Asg>,
    /// Reactive Power Absorbed Maximum Rating
    #[prost(message, optional, tag="8")]
    pub var_max_abs_rtg: ::core::option::Option<Asg>,
    /// Reactive Power Injected Maximum Rating
    #[prost(message, optional, tag="9")]
    pub var_max_inj_rtg: ::core::option::Option<Asg>,
    /// AC voltage maximum rating
    #[prost(message, optional, tag="10")]
    pub v_max_rtg: ::core::option::Option<Asg>,
    /// AC voltage minimum rating
    #[prost(message, optional, tag="11")]
    pub v_min_rtg: ::core::option::Option<Asg>,
    /// AC voltage nominal rating
    #[prost(message, optional, tag="12")]
    pub v_nom_rtg: ::core::option::Option<Asg>,
    /// Active Power Max Rating
    #[prost(message, optional, tag="13")]
    pub w_max_rtg: ::core::option::Option<Asg>,
    /// Active Power (Over-Excited) Rating
    #[prost(message, optional, tag="14")]
    pub w_ovr_ext_rtg: ::core::option::Option<Asg>,
    /// Active power rating at specified over-excited power factor
    #[prost(message, optional, tag="15")]
    pub w_ovr_ext_rtg_pf: ::core::option::Option<Asg>,
    /// Active Power (Under-Excited) Rating
    #[prost(message, optional, tag="16")]
    pub w_und_ext_rtg: ::core::option::Option<Asg>,
    /// Active power rating at specified under-excited power factor
    #[prost(message, optional, tag="17")]
    pub w_und_ext_rtg_pf: ::core::option::Option<Asg>,
}
/// Single point setting (FC=SP) (SPG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spg {
    /// The value of the status setting (false is off, true is on).
    #[prost(bool, tag="1")]
    pub set_val: bool,
}
/// OpenFMB specialization for breaker, recloser and switch status and event profiles:  LN: Circuit
/// breaker   Name: XCBR
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusAndEventXcbr {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node_for_event_and_status: ::core::option::Option<LogicalNodeForEventAndStatus>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub dynamic_test: ::core::option::Option<EnsDynamicTestKind>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="3")]
    pub pos: ::core::option::Option<PhaseDps>,
    /// Fault latch: LT01=51A OR 51B OR 51C
    #[prost(message, optional, tag="4")]
    pub protection_pickup: ::core::option::Option<Acd>,
    /// Protection mode such as a group setting or pre-defined curve profile. It is usually pre-defined
    /// by a circuit segment service.
    #[prost(message, optional, tag="5")]
    pub protection_mode: ::core::option::Option<StatusIns>,
    /// Reclose enabled
    #[prost(message, optional, tag="6")]
    pub reclose_enabled: ::core::option::Option<PhaseSps>,
    /// Reclose mode such idle, cycling and lockout.
    #[prost(message, optional, tag="7")]
    pub reclosing_action: ::core::option::Option<PhaseRecloseAction>,
}
/// Integer control status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusInc {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change or update event of 'stVal' or the last change of value in 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// &lt;&lt;statistics&gt;&gt; Integer controlled step position information (ISC)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusIsc {
    /// Quality of the value in 'valWTr'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Status value
    #[prost(int32, tag="2")]
    pub st_val: i32,
    /// Timestamp of the last change of the value in any of 'valWTr' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// Generic status message information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusMessageInfo {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub message_info: ::core::option::Option<MessageInfo>,
}
/// Status value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusValue {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub identified_object: ::core::option::Option<IdentifiedObject>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub mod_blk: ::core::option::Option<bool>,
}
/// Visible string status (VSS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vss {
    /// Quality of the value in 'stVal'.
    #[prost(message, optional, tag="1")]
    pub q: ::core::option::Option<Quality>,
    /// Value of the data.
    #[prost(string, tag="2")]
    pub st_val: ::prost::alloc::string::String,
    /// Timestamp of the last change of the value in any of 'stVal' or 'q'.
    #[prost(message, optional, tag="3")]
    pub t: ::core::option::Option<Timestamp>,
}
/// LN: Generic process I/O   Name: GGIO
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringEventAndStatusGgio {
    /// UML inherited base object
    #[prost(message, optional, tag="1")]
    pub logical_node: ::core::option::Option<LogicalNode>,
    /// Phase code
    #[prost(message, optional, tag="2")]
    pub phase: ::core::option::Option<OptionalPhaseCodeKind>,
    /// String status
    #[prost(message, optional, tag="3")]
    pub str_in: ::core::option::Option<Vss>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchPoint {
    /// Switch position
    #[prost(message, optional, tag="1")]
    pub pos: ::core::option::Option<ControlDpc>,
    /// Start time
    #[prost(message, optional, tag="2")]
    pub start_time: ::core::option::Option<ControlTimestamp>,
}
/// Curve shape setting (FC=SP) (CSG_SP)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwitchCsg {
    /// The array with the points specifying a curve shape.
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<SwitchPoint>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TmHzPoint {
    /// This is an absolute value field.
    #[prost(float, tag="1")]
    pub hz_val: f32,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub tm_val: ::core::option::Option<ClearingTime>,
}
/// Frequency-Trip Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TmHzCsg {
    /// HF Trip Curve Points
    #[prost(message, repeated, tag="1")]
    pub over_crv_pts: ::prost::alloc::vec::Vec<TmHzPoint>,
    /// LF Trip Curve Points
    #[prost(message, repeated, tag="2")]
    pub under_crv_pts: ::prost::alloc::vec::Vec<TmHzPoint>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TmVoltPoint {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="1")]
    pub tm_val: ::core::option::Option<ClearingTime>,
    /// This is an absolute value field.
    #[prost(float, tag="2")]
    pub volt_val: f32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TmVoltCsg {
    /// HV Trip Curve Points
    #[prost(message, repeated, tag="1")]
    pub over_crv_pts: ::prost::alloc::vec::Vec<TmVoltPoint>,
    /// LV Trip Curve Points
    #[prost(message, repeated, tag="2")]
    pub under_crv_pts: ::prost::alloc::vec::Vec<TmVoltPoint>,
}
/// Constant Reactive Power (Fixed VAr) Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VarSpc {
    /// Constant Reactive Power Mode Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub var_parameter: ::core::option::Option<OperationDvar>,
}
/// Voltage regulation function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltageRegulation {
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="1")]
    pub over_voltage_droop: ::core::option::Option<f32>,
    /// uint/0.1%  The droops define the reaction of the PCS to under/over voltage events. A droop of 1%
    /// means that the PCS will output 100% power if the voltage is 1% of the nominal voltage away from the
    /// upper or lower dead band. The minimum droop value possible is 0.8%.
    #[prost(message, optional, tag="2")]
    pub under_voltage_droop: ::core::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="3")]
    pub voltage_dead_band_minus: ::core::option::Option<f32>,
    /// uint/0.1V  Voltage regulation is performed when the grid voltage goes beyond the dead bands. The
    /// dead bands are defined as follows: Upper DB = voltage set point + dead band plus Lower DB = voltage
    /// set point – dead band minus
    #[prost(message, optional, tag="4")]
    pub voltage_dead_band_plus: ::core::option::Option<f32>,
    /// uint/0.1V  Other modes of operation, such as peak shaving, smoothing or SOC management may
    /// operate if the grid frequency is within the stable band. Upper stable band = frequency set point +
    /// band plus Lower stable band = frequency set point – band minus
    #[prost(message, optional, tag="5")]
    pub voltage_set_point: ::core::option::Option<f32>,
}
/// ESS inverter high level function to maintain voltage within droop dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltageDroop {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub voltage_droop_ctl: ::core::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    pub voltage_regulation: ::core::option::Option<VoltageRegulation>,
}
/// ESS inverter high level function to maintain voltage within dead bands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltagePi {
    /// Control value (TRUE or FALSE)
    #[prost(message, optional, tag="1")]
    pub voltage_pi_ctl: ::core::option::Option<bool>,
    /// Voltage regulation
    #[prost(message, optional, tag="2")]
    pub voltage_regulation: ::core::option::Option<VoltageRegulation>,
}
/// Point definition (Point)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltVarPoint {
    /// This is an absolute value field.
    #[prost(float, tag="1")]
    pub var_val: f32,
    /// This is an absolute value field.
    #[prost(float, tag="2")]
    pub volt_val: f32,
}
/// Voltage-Reactive Power (Volt-VAr) Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltVarCsg {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<VoltVarPoint>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub v_var_parameter: ::core::option::Option<OperationDvvr>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltWPoint {
    /// This is an absolute value field.
    #[prost(float, tag="1")]
    pub volt_val: f32,
    /// This is an absolute value field.
    #[prost(float, tag="2")]
    pub w_val: f32,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoltWcsg {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<VoltWPoint>,
    /// Voltage-Active Power (Volt-Watt) Function
    #[prost(message, optional, tag="2")]
    pub volt_w_parameter: ::core::option::Option<OperationDvwc>,
}
/// Visible string status (VSS)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vsc {
    /// [OpenFMB Extension]  String control value.
    #[prost(string, tag="1")]
    pub ctl_val: ::prost::alloc::string::String,
}
/// Constant Power (Fixed W) Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Wspc {
    /// Constant Reactive Power Mode Enable
    #[prost(bool, tag="1")]
    pub mod_ena: bool,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub w_parameter: ::core::option::Option<OperationDwgc>,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WVarPoint {
    /// This is an absolute value field.
    #[prost(float, tag="1")]
    pub var_val: f32,
    /// This is an absolute value field.
    #[prost(float, tag="2")]
    pub w_val: f32,
}
/// Active Power-Reactive Power (Watt-VAr) Function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WVarCsg {
    /// MISSING DOCUMENTATION!!!
    #[prost(message, repeated, tag="1")]
    pub crv_pts: ::prost::alloc::vec::Vec<WVarPoint>,
    /// MISSING DOCUMENTATION!!!
    #[prost(message, optional, tag="2")]
    pub w_var_parameter: ::core::option::Option<OperationDwvr>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalAlrmKind {
    #[prost(enumeration="AlrmKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalControlModeKind {
    #[prost(enumeration="ControlModeKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalDirectionModeKind {
    #[prost(enumeration="DirectionModeKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalGridConnectionStateKind {
    #[prost(enumeration="GridConnectionStateKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalOperatingStateKind {
    #[prost(enumeration="OperatingStateKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalReactivePowerControlKind {
    #[prost(enumeration="ReactivePowerControlKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalRealPowerControlKind {
    #[prost(enumeration="RealPowerControlKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalStateKind {
    #[prost(enumeration="StateKind", tag="1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionalVoltLimitModeKind {
    #[prost(enumeration="VoltLimitModeKind", tag="1")]
    pub value: i32,
}
/// Reclose action kind such as idle, cycling, or lockout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FaultDirectionKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Unknown = 1,
    /// MISSING DOCUMENTATION!!!
    Forward = 2,
    /// MISSING DOCUMENTATION!!!
    Backward = 3,
    /// MISSING DOCUMENTATION!!!
    Both = 4,
}
/// Reclose action kind such as idle, cycling, or lockout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhaseFaultDirectionKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Unknown = 1,
    /// MISSING DOCUMENTATION!!!
    Forward = 2,
    /// MISSING DOCUMENTATION!!!
    Backward = 3,
}
/// The units defined for usage in the CIM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnitSymbolKind {
    /// Dimension less quantity, e.g. count, per unit, etc.
    None = 0,
    /// Length in meter.
    Meter = 2,
    /// Mass in gram.
    Gram = 3,
    /// Current in ampere.
    Amp = 5,
    /// Plane angle in degrees.
    Deg = 9,
    /// Plane angle in radians.
    Rad = 10,
    /// Relative temperature in degrees Celsius. In the SI unit system the symbol is ºC. Electric charge
    /// is measured in coulomb that has the unit symbol C. To distinguish degree Celsius form coulomb the
    /// symbol used in the UML is degC. Reason for not using ºC is the special character º is difficult to
    /// manage in software.
    DegC = 23,
    /// Capacitance in farad.
    Farad = 25,
    /// Time in seconds.
    Sec = 27,
    /// Inductance in Henry.
    Henry = 28,
    /// Voltage in volt.
    V = 29,
    /// Resistance in ohm.
    Ohm = 30,
    /// Energy in joule.
    Joule = 31,
    /// Force in newton.
    Newton = 32,
    /// Frequency in hertz.
    Hz = 33,
    /// Active power in watt.
    W = 38,
    /// Pressure in pascal (n/m2).
    Pa = 39,
    /// Area in square meters.
    M2 = 41,
    /// Conductance in siemens.
    Siemens = 53,
    /// Apparent power in volt ampere.
    Va = 61,
    /// Reactive power in volt ampere reactive.
    VAr = 63,
    /// Power factor
    WPerVa = 65,
    /// Apparent energy in volt ampere hours.
    VAh = 71,
    /// Real energy in what hours.
    Wh = 72,
    /// Reactive energy in volt ampere reactive hours.
    VArh = 73,
    /// MISSING DOCUMENTATION!!!
    HzPerS = 75,
    /// MISSING DOCUMENTATION!!!
    WPerS = 81,
    /// Other enum not listed
    Other = 100,
    /// Amp hour
    Ah = 106,
    /// Time in minutes.
    Min = 159,
    /// Time in hours.
    Hour = 160,
    /// Volume in cubic meters.
    M3 = 166,
    /// Watts per square meter
    WPerM2 = 179,
    /// Relative temperature in degree fahrenheit.
    DegF = 279,
    /// Mile per hour
    Mph = 500,
}
/// The unit multipliers defined for the CIM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UnitMultiplierKind {
    /// Undefined
    Undefined = 0,
    /// No multiplier or equivalently multiply by 1.
    None = 1,
    /// Other enum not listed
    Other = 2,
    /// Centi 10**-2.
    Centi = 3,
    /// Deci 10**-1.
    Deci = 4,
    /// Giga 10**9.
    Giga = 5,
    /// Kilo 10**3.
    Kilo = 6,
    /// Mega 10**6.
    Mega = 7,
    /// Micro 10**-6.
    Micro = 8,
    /// Milli 10**-3.
    Milli = 9,
    /// Nano 10**-9.
    Nano = 10,
    /// Pico 10**-12.
    Pico = 11,
    /// Tera 10**12.
    Tera = 12,
}
/// Enumeration of phase identifiers. Allows designation of phases for both transmission and
/// distribution equipment, circuits and loads. Residential and small commercial loads are often served
/// from single-phase, or split-phase, secondary circuits. For example of s12N, phases 1 and 2 refer to
/// hot wires that are 180 degrees out of phase, while N refers to the neutral wire. Through single
/// phase transformer connections, these secondary circuits may be served from one or two of the primary
/// phases A, B, and C. For three-phase loads, use the A, B, C phase codes instead of s12N.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PhaseCodeKind {
    /// Not applicable
    None = 0,
    /// Other enum not listed
    Other = 1,
    /// Neutral phase.
    N = 16,
    /// Phase C.
    C = 32,
    /// Phases C and neutral.
    Cn = 33,
    /// Phases A and C.
    Ac = 40,
    /// Phases A, C and neutral.
    Acn = 41,
    /// Phase B.
    B = 64,
    /// Phases B and neutral.
    Bn = 65,
    /// Phases B and C.
    Bc = 66,
    /// Phases B, C, and neutral.
    Bcn = 97,
    /// Phase A.
    A = 128,
    /// Phases A and neutral.
    An = 129,
    /// Phases A and B.
    Ab = 132,
    /// Phases A, B, and neutral.
    Abn = 193,
    /// Phases A, B, and C.
    Abc = 224,
    /// Phases A, B, C, and N.
    Abcn = 225,
    /// Secondary phase 2.
    S2 = 256,
    /// Secondary phase 2 and neutral.
    S2N = 257,
    /// Secondary phase 1.
    S1 = 512,
    /// Secondary phase 1 and neutral.
    S1N = 513,
    /// Secondary phase 1 and 2.
    S12 = 768,
    /// Secondary phases 1, 2, and neutral.
    S12N = 769,
}
/// Validity of the value, as condensed information for the client. In case this value is not
/// 'good', some reasons may be found in the 'detailQual'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ValidityKind {
    /// Undefined
    Undefined = 0,
    /// Supervision function has detected no abnormal condition of either the acquisition function or
    /// the information source.
    Good = 1,
    /// Supervision function has detected an abnormal condition of the acquisition function or the
    /// information source (missing or non-operating updating devices). The value is not defined under this
    /// condition. It shall be used to indicate to the client that the value may be incorrect and shall not
    /// be used.  EXAMPLE If an input unit detects an oscillation of one input it will mark the related
    /// information as invalid.
    Invalid = 2,
    /// Reserved
    Reserved = 3,
    /// Supervision function has detected any abnormal behaviour. However, the value could still be
    /// valid. It is client's responsibility to determine whether the values should be used.
    Questionable = 4,
}
/// (default=process) Defines the source of a value. NOTE 1 Substitution may be done locally or via
/// the communication services. In the second case, specific attributes with a FC=SV are used. NOTE 2
/// There are various means to clear a substitution. As an example, a substitution that was done
/// following an invalid condition may be cleared automatically if the invalid condition is cleared.
/// However, this is a local issue and therefore
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SourceKind {
    /// Undefined
    Undefined = 0,
    /// The value is provided by an input function from the process I/O or is calculated from some
    /// application function.
    Process = 1,
    /// The value is provided by an operator input or by an automatic source.
    Substituted = 2,
}
/// Validity of the value, as condensed information for the client. In case this value is not
/// 'good', some reasons may be found in the 'detailQual'.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeAccuracyKind {
    /// Undefined
    Undefined = 0,
    /// 10 ms (class T0)
    T0 = 7,
    /// 1 ms (class T1)
    T1 = 10,
    /// 100 us (class T2)
    T2 = 14,
    /// 25 us (class T3)
    T3 = 16,
    /// 4 us (class T4)
    T4 = 18,
    /// 1 us (class T5)
    T5 = 20,
    /// Undefined
    Unspecified = 31,
}
/// Schedule parameter kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScheduleParameterKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    None = 1,
    /// Other enum not listed
    Other = 2,
    /// MISSING DOCUMENTATION!!!
    ANetMag = 3,
    /// MISSING DOCUMENTATION!!!
    ANeutMag = 4,
    /// MISSING DOCUMENTATION!!!
    APhsAMag = 5,
    /// MISSING DOCUMENTATION!!!
    APhsBMag = 6,
    /// MISSING DOCUMENTATION!!!
    APhsCMag = 7,
    /// MISSING DOCUMENTATION!!!
    HzMag = 8,
    /// MISSING DOCUMENTATION!!!
    PfNetMag = 9,
    /// MISSING DOCUMENTATION!!!
    PfNeutMag = 10,
    /// MISSING DOCUMENTATION!!!
    PfPhsAMag = 11,
    /// MISSING DOCUMENTATION!!!
    PfPhsBMag = 12,
    /// MISSING DOCUMENTATION!!!
    PfPhsCMag = 13,
    /// MISSING DOCUMENTATION!!!
    PhVNetAng = 14,
    /// MISSING DOCUMENTATION!!!
    PhVNetMag = 15,
    /// MISSING DOCUMENTATION!!!
    PhVNeutAng = 16,
    /// MISSING DOCUMENTATION!!!
    PhVNeutMag = 17,
    /// MISSING DOCUMENTATION!!!
    PhVPhsAAng = 18,
    /// MISSING DOCUMENTATION!!!
    PhVPhsAMag = 19,
    /// MISSING DOCUMENTATION!!!
    PhVPhsBAng = 20,
    /// MISSING DOCUMENTATION!!!
    PhVPhsBMag = 21,
    /// MISSING DOCUMENTATION!!!
    PhVPhsCAng = 22,
    /// MISSING DOCUMENTATION!!!
    PhVPhsCMag = 23,
    /// MISSING DOCUMENTATION!!!
    PpvPhsAbAng = 24,
    /// MISSING DOCUMENTATION!!!
    PpvPhsAbMag = 25,
    /// MISSING DOCUMENTATION!!!
    PpvPhsBcAng = 26,
    /// MISSING DOCUMENTATION!!!
    PpvPhsBcMag = 27,
    /// MISSING DOCUMENTATION!!!
    PpvPhsCaAng = 28,
    /// MISSING DOCUMENTATION!!!
    PpvPhsCaMag = 29,
    /// MISSING DOCUMENTATION!!!
    VaNetMag = 30,
    /// MISSING DOCUMENTATION!!!
    VaNeutMag = 31,
    /// MISSING DOCUMENTATION!!!
    VaPhsAMag = 32,
    /// MISSING DOCUMENTATION!!!
    VaPhsBMag = 33,
    /// MISSING DOCUMENTATION!!!
    VaPhsCMag = 34,
    /// MISSING DOCUMENTATION!!!
    VArNetMag = 35,
    /// MISSING DOCUMENTATION!!!
    VArNeutMag = 36,
    /// MISSING DOCUMENTATION!!!
    VArPhsAMag = 37,
    /// MISSING DOCUMENTATION!!!
    VArPhsBMag = 38,
    /// MISSING DOCUMENTATION!!!
    VArPhsCMag = 39,
    /// MISSING DOCUMENTATION!!!
    WNetMag = 40,
    /// MISSING DOCUMENTATION!!!
    WNeutMag = 41,
    /// MISSING DOCUMENTATION!!!
    WPhsAMag = 42,
    /// MISSING DOCUMENTATION!!!
    WPhsBMag = 43,
    /// MISSING DOCUMENTATION!!!
    WPhsCMag = 44,
}
/// Calculation method (CalcMethodKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CalcMethodKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for P-CLASS.
    PClass = 11,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for M-CLASS.
    MClass = 12,
    /// All analogue values are \[F(t+T)-F(t)\] for a calculation interval T (in the same unit as the
    /// original entity). Note: The client can still calculate rate so: RATE = DIFF/T.
    Diff = 13,
}
/// Power system connect modes to the power grid (GridConnectModeKind)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GridConnectModeKind {
    /// Undefined
    Undefined = 0,
    /// Current-source inverter (CSI)
    Csi = 1,
    /// Voltage-controlled voltage-source inverter (VC-VSI)
    VcVsi = 2,
    /// Current-controlled voltage-source inverter (CC-VSI)
    CcVsi = 3,
    /// Not applicable / Unknown
    None = 98,
    /// MISSING DOCUMENTATION!!!
    Other = 99,
    /// Voltage source inverter regulating to P and Q references (VSI PQ)
    VsiPq = 2000,
    /// Voltage source inverter regulating to voltage and frequency references paralleling other
    /// generation and not grid forming (VSI VF).
    VsiVf = 2001,
    /// Voltage source inverter regulating to voltage and frequency references as primary grid forming
    /// generation (VSI ISO).
    VsiIso = 2002,
}
/// Power factor sign (PFSignKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PfSignKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// All analogue values are \[F(t+T)-F(t)\] for a calculation interval T (in the same unit as the
    /// original entity). Note: The client can still calculate rate so: RATE = DIFF/T.
    Iec = 1,
    /// All analogue values (i.e., all common attributes 'i' and 'f') meet the sampling and filtering
    /// characteristics specified in IEEE C37.118.1 for M-CLASS.
    Eei = 2,
}
/// Behaviour or mode (BehaviourModeKind enumeration)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BehaviourModeKind {
    /// Undefined
    Undefined = 0,
    /// Normal enabled state.
    On = 1,
    /// Process is passively supervised.
    Blocked = 2,
    /// Function is operated but results are indicated as test results.
    Test = 3,
    /// Function is operated in test mode, but with no impact to the process.
    TestBlocked = 4,
    /// Function is inactive but shows its configuration capability.
    Off = 5,
}
/// DER operational state (DERGeneratorStateKind)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DerGeneratorStateKind {
    /// Undefined enum value which can be used for Protobuf generation and be consistent with other
    /// technologies.
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    NotOperating = 1,
    /// MISSING DOCUMENTATION!!!
    Operating = 2,
    /// MISSING DOCUMENTATION!!!
    StartingUp = 3,
    /// MISSING DOCUMENTATION!!!
    ShuttingDown = 4,
    /// MISSING DOCUMENTATION!!!
    AtDisconnectLevel = 5,
    /// MISSING DOCUMENTATION!!!
    RampingInPower = 6,
    /// MISSING DOCUMENTATION!!!
    RampingInReactivePower = 7,
    /// MISSING DOCUMENTATION!!!
    Standby = 8,
    /// MISSING DOCUMENTATION!!!
    NotApplicableUnknown = 98,
    /// MISSING DOCUMENTATION!!!
    Other = 99,
}
/// Dynamic test status (see IEC61850-7-2 section 20.2.1 Direct control with normal security, state
/// machine diagram)   A simplified state machine diagram (from Herb F.) is provided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DynamicTestKind {
    /// Undefined
    Undefined = 0,
    /// None
    None = 1,
    /// Testing status
    Testing = 2,
    /// Operating status
    Operating = 3,
    /// Failed status
    Failed = 4,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HealthKind {
    /// No problems, normal operation ("green").
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    None = 1,
    /// No problems, normal operation ("green").
    Ok = 2,
    /// Minor problems, but in safe operating mode ("yellow"). The exact meaning is a local issue,
    /// depending on the dedicated function/device.
    Warning = 3,
    /// Severe problem, no operation possible ("red").
    Alarm = 4,
}
/// MISSING DOCUMENTATION!!!
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SwitchingCapabilityKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    None = 1,
    /// Open
    Open = 2,
    /// Close
    Close = 3,
    /// Open and Close
    OpenAndClose = 4,
}
/// Double point position status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DbPosKind {
    /// Undefined
    Undefined = 0,
    /// Transient status
    Transient = 1,
    /// Closed status
    Closed = 2,
    /// Open status
    Open = 3,
    /// Invalid status
    Invalid = 4,
}
/// Reclose action kind such as idle, cycling, or lockout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecloseActionKind {
    /// Undefined
    Undefined = 0,
    /// Idle state
    Idle = 1,
    /// Cycling state
    Cycling = 2,
    /// Lockout state
    Lockout = 3,
}
/// Abnormal Operating Performance Category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NorOpCatKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    A = 1,
    /// MISSING DOCUMENTATION!!!
    B = 2,
}
/// Normal Operating Performance Category
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AbnOpCatKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    I = 1,
    /// MISSING DOCUMENTATION!!!
    Ii = 2,
    /// MISSING DOCUMENTATION!!!
    Iii = 3,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlrmKind {
    /// MISSING DOCUMENTATION!!!
    GroundFault = 0,
    /// MISSING DOCUMENTATION!!!
    DcOverVoltage = 1,
    /// MISSING DOCUMENTATION!!!
    AcDisconnectOpen = 2,
    /// MISSING DOCUMENTATION!!!
    DcDisconnectOpen = 3,
    /// MISSING DOCUMENTATION!!!
    GridDisconnect = 4,
    /// MISSING DOCUMENTATION!!!
    CabinetOpen = 5,
    /// MISSING DOCUMENTATION!!!
    ManualShutdown = 6,
    /// MISSING DOCUMENTATION!!!
    OverTemperature = 7,
    /// MISSING DOCUMENTATION!!!
    FrequencyAboveLimit = 8,
    /// MISSING DOCUMENTATION!!!
    FrequencyUnderLimit = 9,
    /// MISSING DOCUMENTATION!!!
    AcVoltageAboveLimit = 10,
    /// MISSING DOCUMENTATION!!!
    AcVoltageUnderLimit = 11,
    /// MISSING DOCUMENTATION!!!
    BlownStringFuseOnInput = 12,
    /// MISSING DOCUMENTATION!!!
    UnderTemperature = 13,
    /// MISSING DOCUMENTATION!!!
    GenericMemoryOrCommunicationError = 14,
    /// MISSING DOCUMENTATION!!!
    HardwareTestFailure = 15,
    /// MISSING DOCUMENTATION!!!
    ManufacturerAlarm = 16,
}
/// Dynamic test status (see IEC61850-7-2 section 20.2.1 Direct control with normal security, state
/// machine diagram)   A simplified state machine diagram (from Herb F.) is provided.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ControlModeKind {
    /// Undefined
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Auto = 1,
    /// MISSING DOCUMENTATION!!!
    Manual = 2,
    /// MISSING DOCUMENTATION!!!
    Override = 3,
    /// MISSING DOCUMENTATION!!!
    Remote = 4,
}
/// The control characteristics for power flow operation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DirectionModeKind {
    /// Undefined
    Undefined = 0,
    /// Testing status
    LockedForward = 1,
    /// MISSING DOCUMENTATION!!!
    LockedReverse = 2,
    /// MISSING DOCUMENTATION!!!
    ReverseIdle = 3,
    /// MISSING DOCUMENTATION!!!
    Bidirectional = 4,
    /// MISSING DOCUMENTATION!!!
    NeutralIdle = 5,
    /// MISSING DOCUMENTATION!!!
    Cogeneration = 6,
    /// MISSING DOCUMENTATION!!!
    ReactiveBidirectional = 7,
    /// MISSING DOCUMENTATION!!!
    BiasBidirectional = 8,
    /// MISSING DOCUMENTATION!!!
    BiasCogeneration = 9,
    /// MISSING DOCUMENTATION!!!
    ReverseCogeneration = 10,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GridConnectionStateKind {
    /// MISSING DOCUMENTATION!!!
    Disconnected = 0,
    /// MISSING DOCUMENTATION!!!
    Connected = 1,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperatingStateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Off = 1,
    /// MISSING DOCUMENTATION!!!
    DisconnectedAndStandby = 2,
    /// MISSING DOCUMENTATION!!!
    DisconnectedAndAvailable = 3,
    /// MISSING DOCUMENTATION!!!
    DisconnectedAndAuthorized = 4,
    /// MISSING DOCUMENTATION!!!
    StartingAndSynchronizing = 5,
    /// MISSING DOCUMENTATION!!!
    ConnectedAndIdle = 6,
    /// MISSING DOCUMENTATION!!!
    ConnectedAndGenerating = 7,
    /// MISSING DOCUMENTATION!!!
    ConnectedAndConsuming = 8,
    /// MISSING DOCUMENTATION!!!
    Stopping = 9,
    /// MISSING DOCUMENTATION!!!
    DisconnectedAndBlocked = 10,
    /// MISSING DOCUMENTATION!!!
    DisconnectedAndInMaintenance = 11,
    /// MISSING DOCUMENTATION!!!
    CeasedToEnergize = 12,
    /// MISSING DOCUMENTATION!!!
    Failed = 13,
}
/// Real power control kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReactivePowerControlKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Advanced = 1,
    /// MISSING DOCUMENTATION!!!
    Droop = 2,
    /// Voltage setpoint
    Voltage = 3,
    /// Reactive power setpoint
    ReactivePower = 4,
    /// MISSING DOCUMENTATION!!!
    PowerFactor = 5,
}
/// Real power control kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RealPowerControlKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Advanced = 1,
    /// MISSING DOCUMENTATION!!!
    Droop = 2,
    /// MISSING DOCUMENTATION!!!
    Isochronous = 3,
    /// Real power setpoint
    RealPower = 4,
}
/// State kind
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StateKind {
    /// MISSING DOCUMENTATION!!!
    Undefined = 0,
    /// MISSING DOCUMENTATION!!!
    Off = 1,
    /// MISSING DOCUMENTATION!!!
    On = 2,
    /// MISSING DOCUMENTATION!!!
    Standby = 3,
}
/// Voltage-limiting types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VoltLimitModeKind {
    /// Undefined
    Undefined = 0,
    /// Testing status
    Off = 1,
    /// MISSING DOCUMENTATION!!!
    HighLimitOnly = 2,
    /// MISSING DOCUMENTATION!!!
    LowLimitOnly = 3,
    /// MISSING DOCUMENTATION!!!
    HighLowLimits = 4,
    /// MISSING DOCUMENTATION!!!
    IvvcHighLimitOnly = 5,
    /// MISSING DOCUMENTATION!!!
    IvvcLowLimitOnly = 6,
    /// MISSING DOCUMENTATION!!!
    IvvcHighLowLimits = 7,
}
