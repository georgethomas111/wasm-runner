use std::ptr;

use mozjs::rooted;

use mozjs::jsapi::JS_NewGlobalObject;
use mozjs::jsapi::OnNewGlobalHookOption;

use mozjs::jsval::UndefinedValue;


use mozjs::rust::JSEngine;
use mozjs::rust::RealmOptions;
use mozjs::rust::Runtime;
use mozjs::rust::SIMPLE_GLOBAL_CLASS;

fn run(rt: Runtime) {
  let context = rt.cx();
  let options = RealmOptions::default();

  rooted!(in(context) let global = unsafe {
    JS_NewGlobalObject(context, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(),
                           OnNewGlobalHookOption::FireOnNewGlobalHook,
                           &*options)
  });

  rooted!(in(context) let mut rval = UndefinedValue());
  rt.evaluate_script(global.handle(), "123+123", "test", 1, rval.handle_mut());
  let res = rval.get().to_int32();
  println!("Result of script = {res}")
}

fn main() {

  let engine = JSEngine::init().expect("failed to initalize JS engine");
  let runtime = Runtime::new(engine.handle());
  
  let mut guess = String::new();
  run(runtime);
}
