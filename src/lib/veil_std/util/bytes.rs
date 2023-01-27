pub unsafe fn putb(at: *mut u8, val: u8) {
    *at = val;
}
pub unsafe fn putw<T>(at: *mut T, val: u16) {
    (at as *mut u16).write_unaligned(val);
}
pub unsafe fn putd<T>(at: *mut T, val: u32) {
    (at as *mut u32).write_unaligned(val);
}
pub unsafe fn putq<T>(at: *mut T, val: u64) {
    (at as *mut u64).write_unaligned(val);
}

pub unsafe fn put<A, T>(at: *mut A, val: T) {
   (at as *mut T).write_unaligned(val)
}

pub unsafe fn getb(at: *const u8) -> u8 {
    *at
}
pub unsafe fn getw(at: *const u8) -> u16 {
    (at as *const u16).read_unaligned()
}
pub unsafe fn getd(at: *const u8) -> u32 {
    (at as *const u32).read_unaligned()
}
pub unsafe fn getq(at: *const u8) -> u64 {
    (at as *const u64).read_unaligned()
}

pub unsafe fn get<A, T>(at: *const A) -> T {
    (at as *const T).read_unaligned()
}
