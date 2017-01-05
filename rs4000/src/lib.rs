
type PicoStatus = u32;

const PICO_OK : PicoStatus = 0x00;
const PICO_OPERATION_FAILED : PicoStatus = 0x06;
const PICO_INVALID_HANDLE : PicoStatus = 0x0C;

#[link(name = "ps4000")]
extern "system" {
    fn ps4000OpenUnit(handle: *mut i16) -> PicoStatus;
    fn ps4000CloseUnit(handle: i16) -> PicoStatus;   
}

pub struct Picoscope {
    handle: i16,
}

impl Default for Picoscope {
    fn default() -> Picoscope {
        Picoscope {
            handle: 0,
        }
    }
}

pub fn open_unit() -> Picoscope {
    let mut picoscope = Picoscope::default();
    let status = unsafe { ps4000OpenUnit(&mut picoscope.handle) };
    return picoscope;
}

pub fn close_unit(picoscope: Picoscope) -> PicoStatus {
    unsafe { ps4000CloseUnit(picoscope.handle) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_unit_return_picoscope() {
        println!("hop!");
        let picoscope = open_unit();
        close_unit(picoscope);
    }
}