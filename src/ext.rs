use std::os::raw::c_char;
use std::io::{Write, Result as IResult};

extern {
    fn u_log(msg: c_char);
}

pub fn log(msg: u8) {
    unsafe { u_log(msg as c_char) };
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

