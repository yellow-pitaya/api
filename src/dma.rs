pub enum Ctrl {
    Single,
    Cyclic,
    StopRx,
}

pub struct Handle {
    pub(crate) size: usize,
    pub(crate) fd: std::fs::File,
}

impl Handle {
    pub fn new(dev: &str) -> crate::Result<Self> {
        const RP_SGMNT_CNT: usize = 8;
        const RP_SGMNT_SIZE: usize = 256 * 1_024;

        let mut handle = Self {
            fd: std::fs::OpenOptions::new()
                .read(true)
                .append(true)
                .open(dev)?,
            size: 0,
        };

        handle.set_size(RP_SGMNT_CNT, RP_SGMNT_SIZE)?;

        Ok(handle)
    }

    pub fn ctrl(&mut self, ctrl: Ctrl) -> crate::Result {
        const STOP_RX: std::ffi::c_ulong = 1;
        const CYCLIC_RX: std::ffi::c_ulong = 3;

        match ctrl {
            Ctrl::Single => return Err(crate::Error::Eoor),
            Ctrl::Cyclic => {
                const CNT: iocuddle::Ioctl<iocuddle::Write, &std::ffi::c_ulong> =
                    unsafe { iocuddle::Ioctl::classic(CYCLIC_RX) };

                CNT.ioctl(&mut self.fd, &0)?;
            }
            Ctrl::StopRx => {
                const CNT: iocuddle::Ioctl<iocuddle::Write, &std::ffi::c_ulong> =
                    unsafe { iocuddle::Ioctl::classic(STOP_RX) };

                CNT.ioctl(&mut self.fd, &0)?;
            }
        }

        Ok(())
    }

    fn set_sgmnt_c(&mut self, no: usize) -> std::io::Result<u32> {
        const SET_RX_SGMNT_CNT: std::ffi::c_ulong = 8;

        const CNT: iocuddle::Ioctl<iocuddle::Write, &std::ffi::c_ulong> =
            unsafe { iocuddle::Ioctl::classic(SET_RX_SGMNT_CNT) };

        CNT.ioctl(&mut self.fd, &(no as std::ffi::c_ulong))
    }

    fn set_sgmnt_s(&mut self, no: usize) -> std::io::Result<u32> {
        const SET_RX_SGMNT_SIZE: std::ffi::c_ulong = 256 * 1_024;

        const CNT: iocuddle::Ioctl<iocuddle::Write, &std::ffi::c_ulong> =
            unsafe { iocuddle::Ioctl::classic(SET_RX_SGMNT_SIZE) };

        CNT.ioctl(&mut self.fd, &(no as std::ffi::c_ulong))
    }

    pub fn read(&mut self) -> crate::Result {
        use std::io::Read;

        let mut contents = [0; 1];

        self.fd.read_exact(&mut contents)?;

        Ok(())
    }

    pub fn set_size(&mut self, sgmnt_cnt: usize, sgmnt_size: usize) -> crate::Result {
        // TODO: check for max. memory size..

        self.set_sgmnt_c(sgmnt_cnt)?;
        self.set_sgmnt_s(sgmnt_size)?;

        self.size = sgmnt_cnt * sgmnt_size;

        Ok(())
    }
}
