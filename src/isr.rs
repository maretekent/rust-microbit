use core::option::Option;
use core::option::Option::{Some, None};

extern {
    fn __StackTop();
    fn Reset_Handler();
    fn NMI_Handler();
    fn HardFault_Handler();
    fn SVC_Handler();
    fn PendSV_Handler();
    fn SysTick_Handler();

    // External interrupts
    fn POWER_CLOCK_IRQHandler();
    fn RADIO_IRQHandler();
    fn UART0_IRQHandler();
    fn SPI0_TWI0_IRQHandler();
    fn SPI1_TWI1_IRQHandler();
    fn GPIOTE_IRQHandler();
    fn ADC_IRQHandler();
    fn TIMER0_IRQHandler();
    fn TIMER1_IRQHandler();
    fn TIMER2_IRQHandler();
    fn RTC0_IRQHandler();
    fn TEMP_IRQHandler();
    fn RNG_IRQHandler();
    fn ECB_IRQHandler();
    fn CCM_AAR_IRQHandler();
    fn WDT_IRQHandler();
    fn RTC1_IRQHandler();
    fn QDEC_IRQHandler();
    fn LPCOMP_IRQHandler();
    fn SWI0_IRQHandler();
    fn SWI1_IRQHandler();
    fn SWI2_IRQHandler();
    fn SWI3_IRQHandler();
    fn SWI4_IRQHandler();
    fn SWI5_IRQHandler();
}

#[allow(non_upper_case_globals)]
const ISRCount: usize = 16 + 32;

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
