use anyhow::{anyhow, Result};
use libftdi1_sys::*;
use std::ffi::CString;
use std::os::raw::c_uchar;

pub struct SainSmartFourChannelRelay {
    context: *mut ftdi_context,
}

impl SainSmartFourChannelRelay {
    pub const N_CHANNELS: u8 = 4;
    const MASK: u8 = (1 << Self::N_CHANNELS) - 1;

    pub fn new(ftdi_device_string: &str) -> Result<Self> {
        unsafe {
            let context = ftdi_new();
            if context.is_null() {
                return Err(anyhow!("Failed to cons up new context"));
            }

            let ftdi_device_cstring = CString::new(ftdi_device_string)?.into_raw();
            let rc = ftdi_usb_open_string(context, ftdi_device_cstring);
            drop(CString::from_raw(ftdi_device_cstring));
            if rc != 0 {
                ftdi_free(context);
                return Err(anyhow!("No device found using '{ftdi_device_string}'"));
            }

            if ftdi_set_bitmode(
                context,
                Self::MASK,
                ftdi_mpsse_mode::BITMODE_BITBANG.0 as c_uchar,
            ) != 0
            {
                ftdi_free(context);
                return Err(anyhow!("Failed to set bitmode"));
            }
            if (*context).type_ != ftdi_chip_type::TYPE_R {
                ftdi_free(context);
                return Err(anyhow!("Chip type is not type R as expected"));
            }
            Ok(Self { context })
        }
    }

    pub fn read(&self) -> Result<u8> {
        let mut pins: u8 = 0;
        match unsafe { ftdi_read_pins(self.context, &mut pins) } {
            0 => Ok(pins & Self::MASK),
            _ => Err(anyhow!("Failed to read pins")),
        }
    }

    pub fn set(&self, pins: u8) -> Result<()> {
        let pins = pins & Self::MASK;
        match unsafe { ftdi_write_data(self.context, &pins, 1) } {
            1 => Ok(()),
            _ => Err(anyhow!("Failed to set pins")),
        }
    }
}

impl Drop for SainSmartFourChannelRelay {
    fn drop(&mut self) {
        unsafe {
            ftdi_free(self.context);
        }
    }
}
