#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

bitflags::bitflags! {
    pub struct Flags: libc::c_int {
        const EMPTY = 0;
    }
}
