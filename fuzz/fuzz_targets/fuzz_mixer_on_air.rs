#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    prodjlink_rs::fuzz::parse_mixer_on_air(data);
});
