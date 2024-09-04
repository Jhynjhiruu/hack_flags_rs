#![no_main]
#![no_std]

use n64::boot::globals::__osBbHackFlags;
use n64::cop0::cop0;
use n64::joybus::ControllerStatus;
use n64::mi::mi;
use n64::si::si;
use n64::text::Colour;
use n64::vi::vi;

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

#[no_mangle]
fn main() -> ! {
    //print!(vi, 2, 2, Colour::WHITE, "Hello, {}", "World!");

    let mut which = unsafe { __osBbHackFlags.read() };

    if which > 3 {
        which = 0;
    }

    do_selection(&mut which);

    unsafe { __osBbHackFlags.write(which) };

    n64::skapi::exit();
}
