use crate::io_ptr;

const PI_BASE: u32 = 0x0460_0000;

const PI_DRAM_ADDR: *mut u32 = io_ptr!(mut PI_BASE + 0x00);
const PI_CART_ADDR: *mut u32 = io_ptr!(mut PI_BASE + 0x04);
const PI_RD_LEN: *mut u32 = io_ptr!(mut PI_BASE + 0x08);
const PI_WR_LEN: *mut u32 = io_ptr!(mut PI_BASE + 0x0C);
const PI_STATUS: *mut u32 = io_ptr!(mut PI_BASE + 0x10);

const PI_BB_ATB_UPPER: *mut u32 = io_ptr!(mut PI_BASE + 0x40);
const PI_BB_NAND_CTRL: *mut u32 = io_ptr!(mut PI_BASE + 0x48);
const PI_BB_NAND_CFG: *mut u32 = io_ptr!(mut PI_BASE + 0x4C);

const PI_BB_RD_LEN: *mut u32 = io_ptr!(mut PI_BASE + 0x58);
const PI_BB_WR_LEN: *mut u32 = io_ptr!(mut PI_BASE + 0x5C);
const PI_BB_GPIO: *mut u32 = io_ptr!(mut PI_BASE + 0x60);

const PI_BB_NAND_ADDR: *mut u32 = io_ptr!(mut PI_BASE + 0x70);

const PI_BB_ATB_LOWER: *mut [u32] = io_ptr!(mut PI_BASE + 0x500; 192);

#[repr(u32)]
pub enum LedValue {
    On = 0,
    Off = 1,
}

pub struct Pi;

impl Pi {
    const fn new() -> Self {
        Self {}
    }

    pub fn dram_addr(&self) -> u32 {
        unsafe { PI_DRAM_ADDR.read_volatile() }
    }

    pub fn bb_gpio(&self) -> u32 {
        unsafe { PI_BB_GPIO.read_volatile() }
    }

    pub fn set_dram_addr(&mut self, val: u32) {
        unsafe { PI_DRAM_ADDR.write_volatile(val) }
    }

    pub fn set_bb_gpio(&mut self, val: u32) {
        unsafe { PI_BB_GPIO.write_volatile(val) }
    }

    pub fn set_led(&mut self, val: LedValue) {
        let prev = self.bb_gpio() & !(1 << 1);
        let new = prev | (1 << 5) | ((val as u32) << 1);
        self.set_bb_gpio(new);
    }
}

static mut PI: Pi = Pi::new();

pub fn pi() -> &'static mut Pi {
    unsafe { &mut PI }
}
