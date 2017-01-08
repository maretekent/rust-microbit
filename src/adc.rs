use gpio::PinNumber;
use core::ptr::read_volatile;

#[allow(non_snake_case)]
#[repr(C)]
struct NRF_ADC_Type {
    TASKS_START: u32,           // OUT, Start an ADC conversion.
    TASKS_STOP: u32,            // OUT, Stop ADC.
    RESERVED0: [u32; 62],       // IN
    EVENTS_END: u32,            // IN OUT, ADC conversion complete.
    RESERVED1: [u32; 128],      // IN
    INTENSET: u32,              // IN OUT, Interrupt enable set register.
    INTENCLR: u32,              // IN OUT, Interrupt enable clear register.
    RESERVED2: [u32; 61],       // IN
    BUSY: u32,                  // IN, ADC busy register.
    RESERVED3: [u32; 63],       // IN
    ENABLE: u32,                // IN OUT, ADC enable.
    CONFIG: u32,                // IN OUT, ADC configuration register.
    RESULT: u32,                // IN, Result of ADC conversion.
    RESERVED4: [u32; 700],      // IN
    POWER: u32,                 // IN OUT, Peripheral power control.
}

// ADC is enabled. If an analog input pin is selected as source of the conversion, the selected pin is configured as an analog input.
const ADC_ENABLE_ENABLE_Enabled: u32 = 0x01;

// ADC busy register.
const ADC_BUSY_BUSY_Pos: u32 = 0; // Position of BUSY field.
const ADC_BUSY_BUSY_Msk: u32 = 0x1 << ADC_BUSY_BUSY_Pos; // Bit mask of BUSY field.
const ADC_BUSY_BUSY_Ready: u32 = 0; // No ongoing ADC conversion is taking place. ADC is ready.
const ADC_BUSY_BUSY_Busy: u32 = 1; // An ADC conversion is taking place. ADC is busy.

// Bits 1..0: ADC resolution
const ADC_CONFIG_RES_Pos: u32   = 0x0;  // Position of RES field.
const ADC_CONFIG_RES_Msk: u32   = 0b11 << ADC_CONFIG_RES_Pos;  // Bit mask of RES field.
const ADC_CONFIG_RES_8bit: u32  = 0x0; // 8bit ADC resolution.
const ADC_CONFIG_RES_9bit: u32  = 0x1; // 9bit ADC resolution.
const ADC_CONFIG_RES_10bit: u32 = 0x2; // 10bit ADC resolution.

// Bits 4..2 : ADC input selection.
const ADC_CONFIG_INPSEL_Pos: u32 = 0x2; // Position of INPSEL field.
const ADC_CONFIG_INPSEL_Msk: u32 = 0b111 << ADC_CONFIG_INPSEL_Pos; // Bit mask of INPSEL field.

#[repr(u32)]
enum ADC_CONFIG_INPSEL {
    AnalogInputNoPrescaling = 0x00, // Analog input specified by PSEL with no prescaling used as input for the conversion.
    AnalogInputTwoThirdsPrescaling = 0x01, // Analog input specified by PSEL with 2/3 prescaling used as input for the conversion.
    AnalogInputOneThirdPrescaling = 0x02, // Analog input specified by PSEL with 1/3 prescaling used as input for the conversion.
    SupplyTwoThirdsPrescaling = 0x05, // Supply voltage with 2/3 prescaling used as input for the conversion.
    SupplyOneThirdPrescaling = 0x06, // Supply voltage with 1/3 prescaling used as input for the conversion.
}

// Bits 6..5 : ADC reference selection.
const ADC_CONFIG_REFSEL_Pos: u32 = 5; // Position of REFSEL field.
const ADC_CONFIG_REFSEL_Msk: u32 = 0b11 << ADC_CONFIG_REFSEL_Pos; // Bit mask of REFSEL field.

#[repr(u32)]
enum ADC_CONFIG_REFSEL {
    VBG = 0x00, // Use internal 1.2V bandgap voltage as reference for conversion.
    External = 0x01, // Use external source configured by EXTREFSEL as reference for conversion.
    SupplyOneHalfPrescaling = 0x02, // Use supply voltage with 1/2 prescaling as reference for conversion. Only usable when supply voltage is between 1.7V and 2.6V.
    SupplyOneThirdPrescaling = 0x03, // Use supply voltage with 1/3 prescaling as reference for conversion. Only usable when supply voltage is between 2.5V and 3.6V.
}

// Bits 15..8 : ADC analog pin selection.
const ADC_CONFIG_PSEL_Pos: u32 = 8; // Position of PSEL field.
const ADC_CONFIG_PSEL_Msk: u32 = 0xFF << ADC_CONFIG_PSEL_Pos; // Bit mask of PSEL field.
#[repr(u32)]
enum ADC_CONFIG_PSEL {
    Disabled = 0,       // Analog input pins disabled.
    AnalogInput0 = 1,   // Use analog input 0 as analog input.
    AnalogInput1 = 2,   // Use analog input 1 as analog input.
    AnalogInput2 = 4,   // Use analog input 2 as analog input.
    AnalogInput3 = 8,   // Use analog input 3 as analog input.
    AnalogInput4 = 16,  // Use analog input 4 as analog input.
    AnalogInput5 = 32,  // Use analog input 5 as analog input.
    AnalogInput6 = 64,  // Use analog input 6 as analog input.
    AnalogInput7 = 128, // Use analog input 7 as analog input.
}
 
// Bits 17..16 : ADC external reference pin selection.
const ADC_CONFIG_EXTREFSEL_Pos: u32 = 16; // Position of EXTREFSEL field.
const ADC_CONFIG_EXTREFSEL_Msk: u32 = 0b11 << ADC_CONFIG_EXTREFSEL_Pos; // Bit mask of EXTREFSEL field.
#[repr(u32)]
enum ADC_CONFIG_EXTREFSEL {
    None             = 0, // Analog external reference inputs disabled.
    AnalogReference0 = 1, // Use analog reference 0 as reference.
    AnalogReference1 = 2, // Use analog reference 1 as reference.
}
 
const NRF_ADC_BASE: *mut NRF_ADC_Type = 0x40007000 as *mut _;

const ADC0_0: *mut NRF_ADC_Type = NRF_ADC_BASE;

fn init_adc(adc: *mut NRF_ADC_Type, analog_input_pin: ADC_CONFIG_PSEL) {
    unsafe {
        (*adc).ENABLE = ADC_ENABLE_ENABLE_Enabled;
        (*adc).CONFIG = (ADC_CONFIG_RES_10bit << ADC_CONFIG_RES_Pos) |
                        ((ADC_CONFIG_INPSEL::AnalogInputOneThirdPrescaling as u32) << ADC_CONFIG_INPSEL_Pos) |
                        ((ADC_CONFIG_REFSEL::SupplyOneThirdPrescaling as u32) << ADC_CONFIG_REFSEL_Pos) |
                        ((analog_input_pin as u32) << ADC_CONFIG_PSEL_Pos) |
                        ((ADC_CONFIG_EXTREFSEL::None as u32) << ADC_CONFIG_EXTREFSEL_Pos);

    }
}

fn read_adc(adc: *mut NRF_ADC_Type, analog_input_pin: ADC_CONFIG_PSEL) -> u16 {
    unsafe {
        let old_config = (*adc).CONFIG;
        let new_config = (old_config & (!ADC_CONFIG_PSEL_Msk)) |
                         ((analog_input_pin as u32) << ADC_CONFIG_PSEL_Pos);
        (*adc).CONFIG = new_config;
        (*adc).TASKS_START = 1;
        while ((read_volatile(&(*adc).BUSY) & ADC_BUSY_BUSY_Msk) >> ADC_BUSY_BUSY_Pos) == ADC_BUSY_BUSY_Busy {}

        read_volatile(&(*adc).RESULT) as u16
    }
}

pub fn adc_init() {
    // pin 1, 4, P2
    // pin 2, 8, P1
    // pin 3, 16, P0
    // pin 4, 32, P3
    // pin 5, 64, P4
    // pin 6, 128, P10
    init_adc(ADC0_0, ADC_CONFIG_PSEL::AnalogInput4);
}

pub fn adc_read() -> u16 {
    read_adc(ADC0_0, ADC_CONFIG_PSEL::AnalogInput4)
}
