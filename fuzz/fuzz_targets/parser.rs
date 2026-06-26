#![no_main]
use apollo_parser::Parser;
use libfuzzer_sys::fuzz_target;
use std::panic;

fuzz_target!(|data: &str| {
    let _ = env_logger::try_init();

    let parser = match panic::catch_unwind(|| Parser::new(data)) {
        Err(err) => {
            panic!("error {err:?}");
        }
        Ok(p) => p,
    };

    let _tree = parser.parse();
});
