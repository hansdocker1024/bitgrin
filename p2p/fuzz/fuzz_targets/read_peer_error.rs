#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate bitgrin_core;
extern crate bitgrin_p2p;

use bitgrin_core::ser;
use bitgrin_p2p::msg::PeerError;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<PeerError, ser::Error> = ser::deserialize(&mut d);
});
