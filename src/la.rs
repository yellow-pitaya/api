const LA_ACQ_BUF_SIZE: usize = 0x4000;
const LA_ACQ_BASE_SIZE: usize = 0x00010000;

const RP_LA_ACQ_CFG_TRIG_MAX: u32 = (1 << 26) - 1;
const RP_LA_ACQ_CFG_TRIG_MIN: u32 = 0;

const RP_CTL_RST_MASK: u32 = 1 << 0;
const RP_CTL_SWT_MASK: u32 = 1 << 1;
const RP_CTL_STA_MASK: u32 = 1 << 2;
const RP_CTL_STO_MASK: u32 = 1 << 3;

const RP_LA_ACQ_RLE_ENABLE_MASK: u32 = 1 << 0;

pub fn open(dev: &str) -> crate::Result<Handle> {
    Handle::open(dev)
}

struct AcqRegsetMap {
    map: mmap_rs::MmapMut,
}

impl AcqRegsetMap {
    fn from_memory(fd: std::fs::File) -> crate::Result<Self> {
        let map = unsafe {
            mmap_rs::MmapOptions::new(LA_ACQ_BASE_SIZE)
                .with_file(fd, 0x0)
                .map_mut()?
        };

        Ok(Self { map })
    }
}

impl std::ops::Deref for AcqRegsetMap {
    type Target = AcqRegset;

    fn deref(&self) -> &Self::Target {
        unsafe {
            &*(self.map.as_ptr() as *const AcqRegset)
        }
    }
}

impl std::ops::DerefMut for AcqRegsetMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            &mut *(self.map.as_mut_ptr() as *mut AcqRegset)
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CfgRegset {
    pub pre: u32, // pre-trigger [number of samples] ; 0 - means that no sample will be taken before trigger
    pub pst: u32, // post-trigger [number of samples] ; 0 - means that no sample will be taken after trigger
}

impl Default for CfgRegset {
    fn default() -> Self {
        Self {
            pre: 0,
            pst: LA_ACQ_BUF_SIZE as u32,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct TriggerRegset {
    cmp_msk: u32, // digital comparator mask
    cmp_val: u32, // digital comparator value
    edg_pos: u32, // digital edge positive mask
    edg_neg: u32, // digital edge negative mask
}

#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct DecimationRegset {
    dec: u32, //  sample rate = 125Msps/(dec+1)
}

#[repr(C)]
struct AcqRegset {
    ctl: u32,              // 0x00 control register
    cfg_aut_con: u32,      // 0x04 configuration (bits to enable automatic and continuous modes)
    trig_mask: u32,        // 0x08 global trigger registers
    reserved_0c: u32,
    cfg: CfgRegset,        // counter configuration registers
    sts: CfgRegset,        // counter status        registers
    cts_acq_lo: u32,       // acquire start time
    cts_acq_hi: u32,       // acquire start time
    cts_trg_lo: u32,       // trigger time
    cts_trg_hi: u32,       // trigger time
    cts_stp_lo: u32,       // acquire stop time
    cts_stp_hi: u32,       // acquire stop time
    reserved_38: u32,
    reserved_3c: u32,
    trg: TriggerRegset,    // trigger settings register
    dec: DecimationRegset, // decimation
    cfg_rle: u32,          // RLE configuration register
    sts_cur: u32,          //
    sts_lst: u32,          //
    cfg_pol: u32,          // bitwise input polarity
}

pub struct Map(mmap_rs::MmapMut);

impl std::ops::Deref for Map {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

pub struct CntStatus {
    pub trig_addr: u32,
    pub pst_length: u32,
    pub buf_ovfl: bool,
}

pub struct RleStatus {
    pub current: u32,
    pub last: u32,
    pub buf_ovfl: bool,
}

pub struct Handle {
    regset: AcqRegsetMap,
    pub dma: crate::dma::Handle,
}

impl Handle {
    fn open(dev: &str) -> crate::Result<Self> {
        let fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open(dev)?;

        let mut handle = Handle {
            regset: AcqRegsetMap::from_memory(fd)?,
            dma: crate::dma::Handle::new("/dev/amba_pl:rprx@2")?,
        };

        handle.reset();
        handle.default_settings()?;
        handle.stop_acq()?;

        Ok(handle)
    }

    pub fn default_settings(&mut self) -> crate::Result {
        const RP_TRG_ALL_MASK: u32 = 0;

        self.global_trig_set(RP_TRG_ALL_MASK);
        self.set_config(0);

        let cfg = CfgRegset::default();
        self.set_cnt_config(&cfg).unwrap();

        let trg = TriggerRegset::default();
        self.set_trig_setting(&trg);

        let dec = DecimationRegset::default();
        self.set_decimation(&dec);

        self.disable_rle();

        Ok(())
    }

    pub fn reset(&mut self) {
        self.set_control(RP_CTL_RST_MASK);
    }

    pub fn run_acq(&mut self) -> crate::Result {
        self.dma.ctrl(crate::dma::Ctrl::Cyclic)?;
        self.set_control(RP_CTL_STA_MASK);

        Ok(())
    }

    pub fn stop_acq(&mut self) -> crate::Result {
        self.dma.ctrl(crate::dma::Ctrl::StopRx)?;
        self.set_control(RP_CTL_STO_MASK);

        Ok(())
    }

    pub fn trigger_acq(&mut self) {
        self.set_control(RP_CTL_SWT_MASK);
    }

    pub fn acq_is_stopped(&self) -> bool {
        let ctl = self.control();

        ctl & RP_CTL_STO_MASK != 0
    }

    pub fn global_trig_set(&mut self, mask: u32) {
        self.regset.trig_mask = mask;
    }

    pub fn blocking_read(&mut self) -> crate::Result {
        self.dma.read()
    }

    pub fn set_config(&mut self, mask: u32) {
        self.regset.cfg_aut_con = mask;
    }

    pub fn set_cnt_config(&mut self, cfg: &CfgRegset) -> crate::Result {
        #[allow(clippy::absurd_extreme_comparisons)]
        if cfg.pre < RP_LA_ACQ_CFG_TRIG_MIN || cfg.pre > RP_LA_ACQ_CFG_TRIG_MAX {
            return Err(crate::Error::Eoor);
        }

        #[allow(clippy::absurd_extreme_comparisons)]
        if cfg.pst < RP_LA_ACQ_CFG_TRIG_MIN || cfg.pst > RP_LA_ACQ_CFG_TRIG_MAX {
            return Err(crate::Error::Eoor);
        }

        self.regset.cfg = cfg.clone();

        Ok(())
    }

    pub fn cnt_config(&self) -> &CfgRegset {
        &self.regset.cfg
    }

    pub fn cnt_status(&self, trig_addr: u32) -> crate::Result<CntStatus> {
        const TRIG_DELAY_SAMPLES: u32 = 1;

        let buf_len = self.buf_len_in_samples() as u32;
        let reg = self.regset.sts.clone();

        let buf_ovfl = trig_addr >= buf_len;

        let mut trig_addr = reg.pre % buf_len;
        let pst_length = reg.pst;

        if trig_addr < TRIG_DELAY_SAMPLES {
            trig_addr += buf_len - TRIG_DELAY_SAMPLES;
        } else {
            trig_addr -= TRIG_DELAY_SAMPLES;
        }

        if trig_addr >= buf_len - 1 {
            return Err(crate::Error::Eoor);
        }

        Ok(CntStatus {
            trig_addr,
            pst_length,
            buf_ovfl,
        })
    }

    pub fn set_trig_setting(&mut self, trg: &TriggerRegset) {
        self.regset.trg = trg.clone();
    }

    pub fn trig_setting(&self) -> &TriggerRegset {
        &self.regset.trg
    }

    pub fn set_decimation(&mut self, dec: &DecimationRegset) {
        self.regset.dec = dec.clone();
    }

    pub fn decimation(&self) -> &DecimationRegset {
        &self.regset.dec
    }

    pub fn set_polarity(&mut self, a_reg: u32) {
        self.regset.cfg_pol = a_reg;
    }

    /**
     * Set bitwise input polarity
     */
    pub fn polarity(&self) -> u32 {
        self.regset.cfg_pol
    }

    /**
     * Get bitwise input polarity
     */
    pub fn enable_rle(&mut self) {
        self.regset.cfg_rle = RP_LA_ACQ_RLE_ENABLE_MASK;
    }

    pub fn disable_rle(&mut self) {
        self.regset.cfg_rle = 0;
    }

    pub fn rle_status(&self, last: u32) -> crate::Result<RleStatus> {
        let current = self.regset.sts_cur;
        let buf_len = self.buf_len_in_samples() as u32;
        let buf_ovfl = last >= buf_len;
        let last = (self.regset.sts_lst % buf_len)
            .checked_sub(1)
            .ok_or(crate::Error::Eoor)?;

        Ok(RleStatus {
            current,
            last,
            buf_ovfl,
        })
    }

    pub fn is_rle(&self) -> bool {
        self.regset.cfg_rle & RP_LA_ACQ_RLE_ENABLE_MASK != 0
    }

    pub fn buf_len_in_samples(&self) -> usize {
        self.dma.size / std::mem::size_of::<i16>()
    }

    pub fn fpga_reg_dump(&self) -> crate::Result {
        crate::fpga_reg_dump("La acq reg", 0, self.regset.map.as_slice())
    }

    pub fn memory(&mut self) -> crate::Result<Map> {
        let map = unsafe {
            mmap_rs::MmapOptions::new(self.dma.size)
                .with_file(self.dma.fd.try_clone()?, 0x0)
                .map_mut()?
        };

        Ok(Map(map))

    }

    fn set_control(&mut self, ctl: u32) {
        self.regset.ctl = ctl;
    }

    fn control(&self) -> u32 {
        self.regset.ctl
    }
}
