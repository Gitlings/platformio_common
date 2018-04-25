#![feature(proc_macro)]

#[macro_use]
extern crate stdweb;

use stdweb::js_export;

#[js_export]
fn hello() {
    js!(
        console.log("Hello from Rust!");
    );
}
