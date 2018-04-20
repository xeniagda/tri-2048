use std::os::raw::c_char;
use std::io::{Write, Result as IResult};

extern {
    fn u_log(msg: c_char);
    fn u_set(num: u8, y: usize, x: usize);
    fn u_rand() -> f32;
}

pub fn log(msg: u8) {
    unsafe { u_log(msg as c_char); }
}

pub fn set(num: u8, y: usize, x: usize) {
    unsafe { u_set(num, y, x); }
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

