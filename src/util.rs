pub fn k0_to_phys<T>(ptr: *const T) -> *const T {
    ptr.map_addr(k0_to_phys_usize)
}

pub fn k0_to_phys_mut<T>(ptr: *mut T) -> *mut T {
    ptr.map_addr(k0_to_phys_usize)
}

pub fn phys_to_k1<T>(ptr: *const T) -> *const T {
    ptr.map_addr(phys_to_k1_usize)
}

pub fn phys_to_k1_mut<T>(ptr: *mut T) -> *mut T {
    ptr.map_addr(phys_to_k1_usize)
}

pub const fn k0_to_phys_usize(addr: usize) -> usize {
    addr & !0xE0000000
}

pub const fn phys_to_k1_usize(addr: usize) -> usize {
    addr | 0xA0000000
}
