#![allow(non_snake_case, non_upper_case_globals)]

use core::option::Option;
use core::option::Option::{Some, None};

extern "C" {
    fn __StackTop();
}

// Creates a dummy exception handler (infinite loop)
macro_rules! dummy_intr_handler {
    ( $name:ident ) => {
        #[linkage = "weak"]
        #[naked]
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            asm!("b .");
        }
    }
}

const NRF_POWER_RAMON: *mut usize = 0x40000524 as *mut usize;
const NRF_POWER_RAMON_RAMxON_ONMODE_Msk: usize = 0x3;

// Following symbols are defined by the linker script
extern "C" {
    // End of code section. This is from where we want to start to copy from.
    static __etext: *mut usize;

    // Where data is copied into.
    static __data_start__: *mut usize;
    static __data_end__: *mut usize;
}

#[naked]
#[no_mangle]
pub unsafe extern "C" fn Reset_Handler() {
    use core::intrinsics::{volatile_load, volatile_store};
    use core::slice::from_raw_parts_mut;
    use core::slice::from_raw_parts;

    // Turn all RAM banks on
    volatile_store(NRF_POWER_RAMON, volatile_load(NRF_POWER_RAMON) | NRF_POWER_RAMON_RAMxON_ONMODE_Msk);

    // Copy data from ROM to RAM.
    let len = ((__data_end__ as usize) - (__data_start__ as usize)) / 4;
    let src = from_raw_parts(__etext, len);
    let mut dst = from_raw_parts_mut(__data_start__, len);
    dst.copy_from_slice(src);

    super::main();
}

dummy_intr_handler!(NMI_Handler);
dummy_intr_handler!(HardFault_Handler);
dummy_intr_handler!(SVC_Handler);
dummy_intr_handler!(PendSV_Handler);
dummy_intr_handler!(SysTick_Handler);

#[naked]
#[no_mangle]
pub unsafe extern "C" fn Default_Handler() {
    asm!("b .");
}

extern "C" {
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn POWER_CLOCK_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn RADIO_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn UART0_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SPI0_TWI0_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SPI1_TWI1_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn GPIOTE_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn ADC_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn TIMER0_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn TIMER1_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn TIMER2_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn RTC0_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn TEMP_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn RNG_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn ECB_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn CCM_AAR_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn WDT_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn RTC1_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn QDEC_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn LPCOMP_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI0_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI1_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI2_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI3_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI4_IRQHandler();
    #[linkage = "weak"]
    #[link_name = "Default_Handler"]
    fn SWI5_IRQHandler();
}
 
#[allow(non_upper_case_globals)]
const ISRCount: usize = 16 + 32;

#[link_section = ".stack"]
pub static STACK: [usize; 512] = [0usize; 512];

#[link_section = ".heap"]
pub static HEAP: [usize; 512] = [0usize; 512];

#[link_section = ".Vectors"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static ISRVectors: [Option<unsafe extern fn()>; ISRCount] = [
    Some(__StackTop),
    Some(Reset_Handler),
    Some(NMI_Handler),
    Some(HardFault_Handler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(SVC_Handler),
    None,
    None,
    Some(PendSV_Handler),
    Some(SysTick_Handler),

    // External Interrupts
    Some(POWER_CLOCK_IRQHandler),
    Some(RADIO_IRQHandler),
    Some(UART0_IRQHandler),
    Some(SPI0_TWI0_IRQHandler),
    Some(SPI1_TWI1_IRQHandler),
    None,
    Some(GPIOTE_IRQHandler),
    Some(ADC_IRQHandler),
    Some(TIMER0_IRQHandler),
    Some(TIMER1_IRQHandler),
    Some(TIMER2_IRQHandler),
    Some(RTC0_IRQHandler),
    Some(TEMP_IRQHandler),
    Some(RNG_IRQHandler),
    Some(ECB_IRQHandler),
    Some(CCM_AAR_IRQHandler),
    Some(WDT_IRQHandler),
    Some(RTC1_IRQHandler),
    Some(QDEC_IRQHandler),
    Some(LPCOMP_IRQHandler),
    Some(SWI0_IRQHandler),
    Some(SWI1_IRQHandler),
    Some(SWI2_IRQHandler),
    Some(SWI3_IRQHandler),
    Some(SWI4_IRQHandler),
    Some(SWI5_IRQHandler),
    None,
    None,
    None,
    None,
    None,
    None
];
