use std::os::raw::c_char;
use std::io::{Write, Result as IResult};

extern {
    fn u_log(msg: c_char);
    fn u_set(num: u8, y: f32, x: f32);
    fn u_clear();
    fn u_rand() -> f32;
}

pub fn log(msg: u8) {
    unsafe { u_log(msg as c_char); }
}

pub fn set(num: u8, y: f32, x: f32) {
    unsafe { u_set(num, y, x); }
}

pub fn clear() {
    unsafe { u_clear(); }
}

pub fn rand() -> f32 {
    unsafe { u_rand() }
}


pub struct JSLog;

impl Write for JSLog {
    fn write(&mut self, buf: &[u8]) -> IResult<usize> {
        for ch in buf {
            log(*ch);
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> IResult<()> {
        Ok(())
    }
}


