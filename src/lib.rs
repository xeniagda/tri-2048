#![feature(box_syntax)]

#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::io::Write;

mod ext;

#[no_mangle]
pub fn start() {
    writeln!(ext::JSLog, "Starting");


}
