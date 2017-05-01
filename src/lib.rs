#![no_std]
#![feature(lang_items, asm)]
#![feature(naked_functions, linkage, core_intrinsics)]
#![feature(compiler_builtins_lib)]

extern crate compiler_builtins;

mod busy_loop;
mod gpio;
mod pins;
#[macro_use] mod serial;
pub mod isr; // Needs to be public, otherwise linker removes symbols
pub mod display;
pub mod adc;

const PERIOD_MS: u32 = 1000;
const ON_MS: u32 = 50;

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> ()
{
    loop {}
}

#[allow(private_no_mangle_fns)]
#[lang = "panic_fmt"]
#[no_mangle]
extern fn panic_fmt(details: ::core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("Panic at {}:{}, {}", file, line, details);
    let row_2 = ::gpio::Pin::output(::pins::ROW_2);
    let col_3 = ::gpio::Pin::output(::pins::COL_3);
    row_2.set_high();
    loop {
        col_3.set_low();
        ::busy_loop::wait_approx_ms(5);
        col_3.set_high();
        ::busy_loop::wait_approx_ms(200);
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

fn led_lights(wait: u32) {
    for y in 0..5u8 {
        for x in 0..5u8 {
            println!("x: {}, y: {}", x, y);
            let (row, col) = display::map_coords(x, y);
            let row_output = gpio::Pin::output(row);
            let col_output = gpio::Pin::output(col);
            row_output.set_high();
            col_output.set_low();
            busy_loop::wait_approx_ms(wait);
            row_output.set_low();
            col_output.set_high();
        }
    }
}

#[inline(never)] // Keep this!
fn main() {
    serial::Serial::init();

    for _ in 0..10 {
        led_lights(5);
    }

    let btn_a = gpio::Pin::input(pins::BUTTON_A);
    let btn_b = gpio::Pin::input(pins::BUTTON_B);


    // adc. We use pins::P0. touch it!
    let adc_p0 = adc::ADC::new(pins::P0).unwrap();

    adc_p0.init();

    while btn_a.is_high() {
        let value = adc_p0.read();
        println!("{}", value);
        busy_loop::wait_approx_ms(100);
    }

    let mut uptime: u32 = 0;
    let row_2 = gpio::Pin::output(pins::ROW_2);
    let col_3 = gpio::Pin::output(pins::COL_3);
    row_2.set_high();

    loop {
        println!("Uptime: {}", uptime);

        // Button is low-active
        println!("Button A pressed: {}", btn_a.is_low());
        println!("Button B pressed: {}", btn_b.is_low());

        uptime += 1;

        col_3.set_low();
        busy_loop::wait_approx_ms(ON_MS);
        col_3.set_high();
        busy_loop::wait_approx_ms(PERIOD_MS - ON_MS);
    }
}
