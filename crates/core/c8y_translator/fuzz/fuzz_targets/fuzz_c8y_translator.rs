#![no_main]
use libfuzzer_sys::fuzz_target;

use c8y_translator::json;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = json::from_thin_edge_json(s);
    }
});
