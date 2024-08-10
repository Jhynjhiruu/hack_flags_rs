#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]
#![feature(asm_const)]
#![feature(ptr_metadata)]
#![feature(strict_provenance)]
#![feature(ptr_as_uninit)]
#![feature(const_maybe_uninit_zeroed)]
#![feature(const_trait_impl)]
#![feature(naked_functions)]

use core::arch::asm;
use core::ops::Range;

//extern crate alloc;

mod boot;
//mod n64_alloc;
mod joybus;
mod pi;
mod si;
mod skapi;
mod text;
mod util;
mod vi;

use joybus::ControllerStatus;
use si::si;
use text::Colour;
use vi::vi;

#[macro_export]
macro_rules! io_ptr {
    (mut $e:expr) => {
        core::ptr::from_raw_parts_mut::<u32>(($e | 0xA0000000) as *mut (), ())
    };
    (mut $e:expr; $n:expr) => {
        core::ptr::from_raw_parts_mut::<[u32]>(($e | 0xA0000000) as *mut (), $n)
    };
}

pub fn data_cache_writeback<T>(data: &[T]) {
    let Range { start, end } = data.as_ptr_range();

    for i in (start.addr()..end.addr()).step_by(0x10) {
        unsafe { asm!("cache 0x19, 0({0})", in(reg) i) }
    }
}

pub fn data_cache_invalidate<T>(data: &[T]) {
    let Range { start, end } = data.as_ptr_range();

    for i in (start.addr()..end.addr()).step_by(0x10) {
        unsafe { asm!("cache 0x11, 0({0})", in(reg) i) }
    }
}

/*macro_rules! print {
    ($vi:expr, $x:expr, $y:expr, $col:expr, $fmt:expr) => {
        $vi.print_string($x, $y, $col, $fmt)
    };
    ($vi:expr, $x:expr, $y:expr, $col:expr, $fmt:expr, $( $arg:tt )*) => {
        $vi.print_string($x, $y, $col, &alloc::format!($fmt, $( $arg ),*))
    };
}*/

const STICK_DIR_CUTOFF: i8 = 40;

fn do_selection(which: &mut u32) {
    let vi = vi();

    vi.init();

    let si = si();

    let status = si.query_controllers();

    assert!(matches!(status[0], ControllerStatus::StandardController(_)));

    let mut last_inputs = si.read_controllers()[0]
        .expect("P1 controller should always be read successfully")
        .expect("P1 controller should always be present");

    loop {
        vi.clear_framebuffer();

        vi.print_string(3, 3, Colour::WHITE, "Left/right to select which input");
        vi.print_string(3, 4, Colour::WHITE, "to use, A to confirm");

        vi.print_string(3, 5, Colour::WHITE, "Current selection: ");

        let player_colour = if matches!(
            si.query_controllers()[*which as usize],
            ControllerStatus::StandardController(_)
        ) {
            Colour::WHITE
        } else {
            Colour::RED
        };

        vi.print_char(22, 5, player_colour, b'P');
        vi.print_nibble(23, 5, player_colour, *which as u8 + 1);

        vi.wait_vsync();
        vi.next_framebuffer();

        let inputs = si.read_controllers()[0]
            .expect("P1 controller should always be read successfully")
            .expect("P1 controller should always be present");

        if ((inputs.x() > STICK_DIR_CUTOFF && last_inputs.x() <= STICK_DIR_CUTOFF)
            || (inputs.d_right() && !last_inputs.d_right()))
            && *which < 3
        {
            *which += 1;
        }

        if ((inputs.x() < -STICK_DIR_CUTOFF && last_inputs.x() >= -STICK_DIR_CUTOFF)
            || (inputs.d_left() && !last_inputs.d_left()))
            && *which > 0
        {
            *which -= 1;
        }

        if inputs.a() && !last_inputs.a() {
            for _ in 0..2 {
                vi.print_char(22, 5, Colour::GREEN, b'P');
                vi.print_nibble(23, 5, Colour::GREEN, *which as u8 + 1);

                vi.wait_vsync();
                vi.next_framebuffer();
            }

            for _ in 0..30 {
                vi.wait_vsync();
                vi.next_framebuffer();
            }

            vi.clear_framebuffer();
            vi.wait_vsync();
            vi.next_framebuffer();
            vi.set_h_video(0);

            break;
        }

        last_inputs = inputs;
    }
}

#[link_section = ".text"]
fn main() -> ! {
    //print!(vi, 2, 2, Colour::WHITE, "Hello, {}", "World!");

    let hack_flags_ptr = core::ptr::from_raw_parts_mut::<u32>(0x8000038C as *mut _, ());

    let mut which = unsafe { hack_flags_ptr.read_volatile() };

    if which > 3 {
        which = 0;
    }

    do_selection(&mut which);

    unsafe { hack_flags_ptr.write_volatile(which) };

    skapi::exit();
}
