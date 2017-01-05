
type PicoStatus = u32;

const PICO_OK : PicoStatus = 0x00;
const PICO_OPERATION_FAILED : PicoStatus = 0x06;
const PICO_INVALID_HANDLE : PicoStatus = 0x0C;

pub enum Channel {
    A,
    B,
    C,
    D
}

pub enum Range {
    r10mV,
    r20mV,
    r50mV,
    r100mV,
    r200mV,
    r500mV,
    r1V,
    r2V,
    r5V,
    r10V,
    r20V,
    r50V,
    r100V,
    MAX_RANGES,
}

mod abi {
    use super::{PicoStatus, Channel, Range};

    #[link(name = "ps4000")]
    extern "C" {
        pub fn ps4000OpenUnit(handle: *mut i16) -> PicoStatus;
        pub fn ps4000CloseUnit(handle: i16) -> PicoStatus;
        pub fn ps4000SetChannel(handle: i16, channel: Channel, enabled: i16, dc: i16, range: Range) -> PicoStatus;
    }
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

/// Consctruct a Picoscope unit
impl Picoscope {
    pub fn open_unit() -> Picoscope {
        let mut picoscope = Picoscope::default();
        let status = unsafe { abi::ps4000OpenUnit(&mut picoscope.handle) };
        return picoscope;
    }

    pub fn set_channel(&self, channel: Channel, enabled: bool, is_dc: bool, range: Range) {
        unsafe {
            let ienabled : i16 = if enabled { 1 } else { 0 };
            let idc : i16 = if is_dc { 1 } else { 0 };
            abi::ps4000SetChannel(self.handle, channel, ienabled, idc, range);
        }
    }
}

impl Drop for Picoscope {
    fn drop(&mut self) {
        close_unit(self);
    }
}

/// Close a Picoscope unit, releasing any used resources
fn close_unit(picoscope: &mut Picoscope) -> PicoStatus {
    unsafe { abi::ps4000CloseUnit(picoscope.handle) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_unit_return_picoscope() {
        let picoscope = Picoscope::open_unit();
        assert!(picoscope.handle > 0);
    }

    #[test]
    fn set_channel_does_not_fail() {
        let picoscope = Picoscope::open_unit();
        picoscope.set_channel(Channel::A, true, true, Range::r1V);
        assert!(picoscope.handle > 0);
    }
}