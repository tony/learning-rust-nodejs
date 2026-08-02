use neon::prelude::*;

mod logic;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(logic::get_hello_message()))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
