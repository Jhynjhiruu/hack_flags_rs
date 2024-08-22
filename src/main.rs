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
#![feature(pointer_byte_offsets)]
#![feature(panic_info_message)]

use core::arch::asm;
use core::ops::Range;
use core::ptr::from_raw_parts;

//extern crate alloc;

mod boot;
mod cop0;
mod joybus;
mod mi;
//mod n64_alloc;
mod pi;
mod si;
mod skapi;
mod text;
mod util;
mod vi;

use boot::globals::__osBbHackFlags;
use cop0::cop0;
use joybus::ControllerStatus;
use mi::mi;
use si::si;
use text::Colour;
use vi::vi;

#[macro_export]
macro_rules! io_ptr {
    (mut $e:expr) => {
        core::ptr::from_raw_parts_mut::<u32>($crate::util::phys_to_k1_u32($e) as *mut (), ())
    };
    (mut $e:expr; $n:expr) => {
        core::ptr::from_raw_parts_mut::<[u32]>($crate::util::phys_to_k1_u32($e) as *mut (), $n)
    };
}

macro_rules! cache {
    (data, $n:expr, $e:expr) => {
        unsafe {
            asm!(
                ".set noat",
                "cache {num}, 0({reg})",
                ".set at",
                num = const ($n << 2) | 1,
                reg = in(reg) $e
            )
        }
    };
    (instruction, $n:expr, $e:expr) => {
        unsafe {
            asm!(
                ".set noat",
                "cache {num}, 0({reg})",
                ".set at",
                num = const $n << 2,
                reg = in(reg) $e
            )
        }
    };
}

pub fn data_cache_writeback<T>(data: &[T]) {
    let Range { start, end } = data.as_ptr_range();

    for i in (start.addr()..end.addr()).step_by(0x10) {
        cache!(data, 6, i);
    }
}

pub fn data_cache_invalidate<T>(data: &[T]) {
    let Range { start, end } = data.as_ptr_range();

    for i in (start.addr()..end.addr()).step_by(0x10) {
        cache!(data, 4, i);
    }
}

pub fn instruction_cache_invalidate<T>(data: &[T]) {
    let Range { start, end } = data.as_ptr_range();

    for i in (start.addr()..end.addr()).step_by(0x20) {
        cache!(instruction, 4, i);
    }
}

macro_rules! print {
    ($vi:expr, $x:expr, $y:expr, $col:expr, $fmt:expr) => {
        $vi.print_string($x, $y, $col, $fmt)
    };
    ($vi:expr, $x:expr, $y:expr, $col:expr, $fmt:expr, $( $arg:tt )*) => {
        $vi.print_string($x, $y, $col, &alloc::format!($fmt, $( $arg ),*))
    };
}

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

        vi.print_string(
            4,
            3,
            Colour::WHITE,
            "Left/right to select which input\n\tto use, A to confirm",
        );

        vi.print_string(4, 5, Colour::WHITE, "Current selection: ");

        /*vi.print_u32(4, 6, Colour::GREY, cop0().status());
        vi.print_u32(4, 7, Colour::GREY, cop0().cause());
        vi.print_u32(4, 8, Colour::GREY, mi().interrupt());
        vi.print_u32(4, 9, Colour::GREY, mi().mask());
        vi.print_u32(4, 10, Colour::GREY, mi().bb_mask());
        vi.print_u32(4, 11, Colour::GREY, mi().bb_interrupt());*/

        let player_colour = if matches!(
            si.query_controllers()[*which as usize],
            ControllerStatus::StandardController(_)
        ) {
            Colour::WHITE
        } else {
            Colour::RED
        };

        vi.print_char(23, 5, player_colour, b'P');
        vi.print_nibble(24, 5, player_colour, *which as u8 + 1);

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
                vi.print_char(23, 5, Colour::GREEN, b'P');
                vi.print_nibble(24, 5, Colour::GREEN, *which as u8 + 1);

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
            vi.blank();

            break;
        }

        last_inputs = inputs;
    }
}

fn main() -> ! {
    //print!(vi, 2, 2, Colour::WHITE, "Hello, {}", "World!");

    let mut which = unsafe { __osBbHackFlags.read() };

    if which > 3 {
        which = 0;
    }

    do_selection(&mut which);

    unsafe { __osBbHackFlags.write(which) };

    skapi::exit();
}
