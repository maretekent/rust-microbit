use core::ptr::read_volatile;
use nrf51::GPIO;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PinNumber(pub u8);

const CONFIG_INPUT: u32 = 0 << 0;
const CONFIG_OUTPUT: u32 = 1 << 0;
const CONFIG_PULLUP: u32 = 3 << 2;

impl PinNumber {

    fn mask(&self) -> u32 {
        1 << self.0
    }

    fn configure(&self, cfg: u32) {
        // XXX: Can we solve that better?
        unsafe {
            match (self.0) {
                 0 => (*GPIO.get()).pin_cnf0.write(|w| w.bits(cfg)),
                 1 => (*GPIO.get()).pin_cnf1.write(|w| w.bits(cfg)),
                 2 => (*GPIO.get()).pin_cnf2.write(|w| w.bits(cfg)),
                 3 => (*GPIO.get()).pin_cnf3.write(|w| w.bits(cfg)),
                 4 => (*GPIO.get()).pin_cnf4.write(|w| w.bits(cfg)),
                 5 => (*GPIO.get()).pin_cnf5.write(|w| w.bits(cfg)),
                 6 => (*GPIO.get()).pin_cnf6.write(|w| w.bits(cfg)),
                 7 => (*GPIO.get()).pin_cnf7.write(|w| w.bits(cfg)),
                 8 => (*GPIO.get()).pin_cnf8.write(|w| w.bits(cfg)),
                 9 => (*GPIO.get()).pin_cnf9.write(|w| w.bits(cfg)),
                10 => (*GPIO.get()).pin_cnf10.write(|w| w.bits(cfg)),
                11 => (*GPIO.get()).pin_cnf11.write(|w| w.bits(cfg)),
                12 => (*GPIO.get()).pin_cnf12.write(|w| w.bits(cfg)),
                13 => (*GPIO.get()).pin_cnf13.write(|w| w.bits(cfg)),
                14 => (*GPIO.get()).pin_cnf14.write(|w| w.bits(cfg)),
                15 => (*GPIO.get()).pin_cnf15.write(|w| w.bits(cfg)),
                16 => (*GPIO.get()).pin_cnf16.write(|w| w.bits(cfg)),
                17 => (*GPIO.get()).pin_cnf17.write(|w| w.bits(cfg)),
                18 => (*GPIO.get()).pin_cnf18.write(|w| w.bits(cfg)),
                19 => (*GPIO.get()).pin_cnf19.write(|w| w.bits(cfg)),
                20 => (*GPIO.get()).pin_cnf20.write(|w| w.bits(cfg)),
                21 => (*GPIO.get()).pin_cnf21.write(|w| w.bits(cfg)),
                22 => (*GPIO.get()).pin_cnf22.write(|w| w.bits(cfg)),
                23 => (*GPIO.get()).pin_cnf23.write(|w| w.bits(cfg)),
                24 => (*GPIO.get()).pin_cnf24.write(|w| w.bits(cfg)),
                25 => (*GPIO.get()).pin_cnf25.write(|w| w.bits(cfg)),
                26 => (*GPIO.get()).pin_cnf26.write(|w| w.bits(cfg)),
                27 => (*GPIO.get()).pin_cnf27.write(|w| w.bits(cfg)),
                28 => (*GPIO.get()).pin_cnf28.write(|w| w.bits(cfg)),
                29 => (*GPIO.get()).pin_cnf29.write(|w| w.bits(cfg)),
                30 => (*GPIO.get()).pin_cnf30.write(|w| w.bits(cfg)),
                31 => (*GPIO.get()).pin_cnf31.write(|w| w.bits(cfg)),
                _ => panic!("Invalid pin")
            }
        }
    }

    pub fn input_pullup(&self) {
        self.configure(CONFIG_INPUT | CONFIG_PULLUP);
    }

    pub fn output_pullup(&self) {
        self.configure(CONFIG_OUTPUT | CONFIG_PULLUP);
    }

    pub fn output(&self) {
        self.configure(CONFIG_OUTPUT);
    }

    pub fn input(&self) {
        self.configure(CONFIG_INPUT);
    }
}

pub struct Pin {
    mask: u32,
}

impl Pin {
    pub fn input(number: PinNumber) -> Self {
        number.input();
        Pin {
            mask: number.mask(),
        }
    }

    pub fn output(number: PinNumber) -> Self {
        number.output();
        Pin {
            mask: number.mask(),
        }
    }

    pub fn set_high(&self) {
        unsafe {
            (*GPIO.get()).outset.write(|w| w.bits(self.mask));
        }
    }

    pub fn set_low(&self) {
        unsafe {
            (*GPIO.get()).outclr.write(|w| w.bits(self.mask));
        }
    }

    pub fn is_high(&self) -> bool {
        let reg = unsafe { (*GPIO.get()).in_.read().bits() };
	    (reg & self.mask) == self.mask
    }

    pub fn is_low(&self) -> bool {
        !self.is_high()
    }
}
