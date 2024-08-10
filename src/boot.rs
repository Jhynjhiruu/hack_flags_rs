use core::arch::asm;
use core::ffi::{c_int, c_void};
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, from_raw_parts, from_raw_parts_mut};

use crate::main;
//use crate::n64_alloc::ALLOCATOR;

#[link_section = ".boot"]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[link_section = ".boot"]
#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void {
    /*{
        let dest = dest as *mut u32;
        let src = src as *const u32;
        let u32_len = ((n / 4) * 4);
        for i in 0..u32_len / 4 {
            dest.add(i).write(src.add(i).read())
        }
        if n - u32_len > 0 {
            let s = src.add(u32_len / 4).read()
                & (match n - u32_len {
                    1 => 0xFF000000,
                    2 => 0xFFFF0000,
                    3 => 0xFFFFFF00,
                    _ => unreachable!(),
                });
            let d = dest.add(u32_len / 4).read()
                & (match n - u32_len {
                    1 => 0x00FFFFFF,
                    2 => 0x0000FFFF,
                    3 => 0x000000FF,
                    _ => unreachable!(),
                });
            dest.add(u32_len / 4).write(d | s)
        }
    }
    dest*/
    {
        let dst = from_raw_parts_mut::<[u8]>(dest.cast(), n).cast::<u8>();
        let src = from_raw_parts::<[u8]>(src.cast(), n).cast::<u8>();
        for i in 0..n {
            dst.add(i).write(src.add(i).read())
        }
    }
    dest
}

#[link_section = ".boot"]
#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: usize) -> *mut c_void {
    {
        let ptr = from_raw_parts_mut::<[u8]>(dest.cast(), n).cast::<u8>();
        let c = c as u8;
        for i in 0..n {
            ptr.add(i).write(c)
        }
    }
    dest
}

extern "C" {
    static mut __bss_start: c_void;
    static __bss_size: c_void;

    static mut _heap_start: c_void;
    static _heap_len: c_void;
}

#[link_section = ".boot"]
unsafe fn clear_bss() {
    let start = addr_of_mut!(__bss_start).cast();
    let size = addr_of!(__bss_size).addr();
    memset(start, 0, size);
}

/*#[link_section = ".entry"]
#[no_mangle]
pub unsafe fn start() -> ! {
    clear_bss();
    //ALLOCATOR.init(addr_of_mut!(_heap_start), addr_of!(_heap_len).addr());
    main();
}*/

#[link_section = ".entry"]
#[no_mangle]
#[naked]
pub unsafe extern "C" fn _startup() {
    asm!(
        ".set   noreorder  ",
        "  jal  {clear_bss}",
        "   lui $sp, 0x8040",
        "  j    {main}     ",
        "   nop            ",
        ".set   reorder    ",
        clear_bss = sym clear_bss,
        main = sym main,
        options(noreturn),
    )
}
