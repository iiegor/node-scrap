#[macro_use]
extern crate neon;
extern crate scrap;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean};

mod internal;
pub use internal::*;

fn screenshot(call: Call) -> JsResult<JsBoolean> {
    let path_arg = call.arguments.require(call.scope, 0)?.check::<JsString>()?.value();
    Ok(JsBoolean::new(call.scope, internal::capture(&path_arg)))
}

fn start_recording(call: Call) -> JsResult<JsBoolean> {
    let path_arg = call.arguments.require(call.scope, 0)?.check::<JsString>()?.value();
    let fps_arg = call.arguments.require(call.scope, 1)?.check::<JsString>()?.value();
    internal::record(&path_arg, &fps_arg);
    Ok(JsBoolean::new(call.scope, true))
}

fn stop_recording(call: Call) -> JsResult<JsBoolean> {
    Ok(JsBoolean::new(call.scope, true))
}

register_module!(m, {
    m.export("screenshot", screenshot)?;
    m.export("startRecording", start_recording)?;
    m.export("stopRecording", stop_recording)?;
    Ok(())
});
