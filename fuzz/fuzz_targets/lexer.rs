#![no_main]
use apollo_parser::Lexer;
use libfuzzer_sys::fuzz_target;
use std::panic;

fuzz_target!(|data: &str| {
    let _ = env_logger::try_init();

    let (_tokens, _errors) = match panic::catch_unwind(|| Lexer::new(data).lex()) {
        Err(err) => {
            panic!("error {err:?}");
        }
        Ok(p) => p,
    };
});
