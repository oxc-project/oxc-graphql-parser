#![no_main]
use libfuzzer_sys::fuzz_target;
use oxc_graphql::Lexer;
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
