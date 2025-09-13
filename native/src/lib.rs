// Only compile Neon-related code when not testing
#[cfg(not(test))]
use neon::prelude::*;

#[cfg(not(test))]
mod logic;

#[cfg(not(test))]
fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(logic::get_hello_message()))
}

#[cfg(not(test))]
register_module!(mut cx, { cx.export_function("hello", hello) });

// When testing, include logic module directly
#[cfg(test)]
#[path = "logic.rs"]
mod logic;
