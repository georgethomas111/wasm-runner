use std::ptr;
use std::env;

use mozjs::rooted;

use mozjs::jsapi::JS_NewGlobalObject;
use mozjs::jsapi::OnNewGlobalHookOption;
use mozjs::jsapi::JS_GetStringLength;
use mozjs::jsapi::JS_GetStringCharAt;
use mozjs::jsval::UndefinedValue;


use mozjs::rust::JSEngine;
use mozjs::rust::RealmOptions;
use mozjs::rust::Runtime;
use mozjs::rust::SIMPLE_GLOBAL_CLASS;

fn run(rt: Runtime, js: &str) {
  let context = rt.cx();
  let options = RealmOptions::default();

  rooted!(in(context) let global = unsafe {
    JS_NewGlobalObject(context, &SIMPLE_GLOBAL_CLASS, ptr::null_mut(),
                           OnNewGlobalHookOption::FireOnNewGlobalHook,
                           &*options)
  });

  rooted!(in(context) let mut rval = UndefinedValue());
  rt.evaluate_script(global.handle(), js, "test", 1, rval.handle_mut());
  if rval.get().is_number() {
    let date_now = rval.get().to_number();
    println!("{}", date_now)
  }

  if rval.get().is_string() {
    let ptr_str = rval.get().to_string();
    unsafe {
      let length = JS_GetStringLength(ptr_str);
      let mut x:u16 = 0;
      let char_val = &mut x as *mut u16;
      let mut i = 0;
      let mut strs:Vec<u16> = vec![];
      loop {
        JS_GetStringCharAt(context, ptr_str, i, char_val);
        strs.push(*char_val);
        
        i+=1;
        if i == (length) {
          break;
        }
      }

      let s = String::from_utf16(&strs).unwrap();
      println!("{}", s)
    }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Please pass javascript to execute in quotes as the second argument");
    return
  }
  let js: &str = &args[1][..];
  println!("Input script -> {:?}", js);
  let engine = JSEngine::init().expect("failed to initalize JS engine");
  let runtime = Runtime::new(engine.handle());
  
  run(runtime, js);
}
