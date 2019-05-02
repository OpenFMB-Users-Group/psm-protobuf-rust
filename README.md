[![Documentation](https://docs.rs/rust-openfmb-ops-protobuf/badge.svg)](https://docs.rs/rust-openfmb-ops-protobuf/)
[![Crate](https://img.shields.io/crates/v/rust-openfmb-ops-protobuf.svg)](https://crates.io/crates/rust-openfmb-ops-protobuf)

# Rust language protobuf for OpenFMB operational use cases

[Rust](https://www.rust-lang.org/) programming language Protocol Buffer (protobuf) definitions based on the OpenFMB operational use case data model located [here](https://gitlab.com/openfmb/data-models/ops).

## Including in your project

There are a couple of methods for adding these definitions to your project in your Cargo.toml file.

### From crates.io

```toml
[dependencies]
# Rust defintions for OpenFMB data model
rust-openfmb-ops-protobuf = "*" # <- Change to the version you prefer
```

### From the GitLab repository

```toml
[dependencies]
# Rust defintions for OpenFMB data model
rust-openfmb-ops-protobuf = { git = "https://gitlab.com/openfmb/psm/ops/protobuf/rust-openfmb-ops-protobuf.git", tag = "<release-tag-label>" }
```

## Using

After adding the depencency in your project's Cargo.toml file, you are ready to include the protobuf definitions into your source files like this:

```rust
extern crate rust_openfmb_ops_protobuf;
use rust_openfmb_ops_protobuf::*;
```

After importing the crate, you can now start using the protobuf definitions like this:

```rust
fn main() {
    // Create a new MeterReadingProfile message
    let mrp = openfmb::metermodule::MeterReadingProfile::default();

    // Set the time quality for this message
    let mut tq = openfmb::commonmodule::TimeQuality::default();
    tq.clock_failure = false;
    tq.clock_not_synchronized = false;
    tq.leap_seconds_known = true;
    tq.time_accuracy = openfmb::commonmodule::TimeAccuracyKind::Undefined as i32;

    // Create the TimeStamp
    let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut ts = openfmb::commonmodule::Timestamp::default();
    ts.seconds = current_time.as_secs();
    ts.fraction = current_time.subsec_nanos() * (1 / 2 & 32);
    ts.tq = Some(tq);
    
    // Create the IdentifiedObject for this mesage
    let mut mi_id = openfmb::commonmodule::IdentifiedObject::default();
    mi_id.m_rid = Some(Uuid::new_v4().to_hyphenated().to_string());

    // Create the MessageInfo
    let mut mi = openfmb::commonmodule::MessageInfo::default();
    mi.identified_object = Some(mi_id);
    mi.message_time_stamp = Some(ts);

    // Create the ReadingMessageInfo
    let mut rmi = openfmb::commonmodule::ReadingMessageInfo::default();
    rmi.message_info = Some(mi);

    // Set the ReadingMessageInfo for the profile
    mrp.reading_message_info = Some(rmi);
    
    //
    // Continue populating the message
    //

    // Encode the message into the protobuf byte format
    let mut bmsg: Vec<u8> = Vec::new();
    mrp.encode(&mut bmsg).unwrap();

    // Do what you want with the bytes now
}
```

## Copyright

See the COPYRIGHT file for copyright information of information contained in this repository.

## License

Unless otherwise noted, all files in this repository are distributed under the Apache Version 2.0 license found in the LICENSE file.
