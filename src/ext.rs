use std::os::raw::c_char;
use std::io::{Write, Result as IResult};

#[cfg(feature="wasm")]
extern {
    fn u_log(msg: c_char);
    fn u_set_size(size: usize);
    fn u_set(num: u8, draw_direct: bool, y: usize, x: usize);
    fn u_move(num: u8, y: usize, x: usize, y_to: usize, x_to: usize);
    fn u_lose();
    fn u_rand() -> f32;
}

pub fn log(msg: u8) {
    #[cfg(feature="wasm")]
    unsafe { u_log(msg as c_char); }
}

pub fn set_size(size: usize) {
    #[cfg(feature="wasm")]
    unsafe { u_set_size(size); }
}

pub fn set(num: u8, draw_direct: bool, y: usize, x: usize) {
    #[cfg(feature="wasm")]
    unsafe { u_set(num, draw_direct, y, x); }
}

pub fn move_tile(num: u8, pos: (usize, usize), to: (usize, usize)) {
    #[cfg(feature="wasm")]
    unsafe {
        u_move(num, pos.0, pos.1, to.0, to.1);
    }
}

pub fn lose() {
    #[cfg(feature="wasm")]
    unsafe { u_lose(); }
}

pub fn rand() -> f32 {
    #[cfg(feature="wasm")]
    unsafe { u_rand() }
    #[cfg(test)]
    0.
}


pub struct JSLog;

impl Write for JSLog {
    fn write(&mut self, buf: &[u8]) -> IResult<usize> {
        #[cfg(feature="wasm")]
        {
            for ch in buf {
                log(*ch);
            }
            Ok(buf.len())
        }
        #[cfg(not(feature="wasm"))]
        {
            use std::io::stdout;
            stdout().write(buf)
        }

    }


    fn flush(&mut self) -> IResult<()> {
        Ok(())
    }
}


