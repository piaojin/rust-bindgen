/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct bitfield {
    pub _bitfield_1: u8,
    pub e: ::std::os::raw::c_int,
    pub _bitfield_2: u8,
    pub _bitfield_3: u32,
}
#[test]
fn bindgen_test_layout_bitfield() {
    assert_eq!(::std::mem::size_of::<bitfield>() , 16usize , concat ! (
               "Size of: " , stringify ! ( bitfield ) ));
    assert_eq! (::std::mem::align_of::<bitfield>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( bitfield ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const bitfield ) ) . e as * const _ as usize }
                , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( bitfield ) , "::" ,
                stringify ! ( e ) ));
}
impl Clone for bitfield {
    fn clone(&self) -> Self { *self }
}
impl bitfield {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_ushort {
        let mask = 1usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 1usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_ushort {
        let mask = 2usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 1usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 2usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 1usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn c(&self) -> ::std::os::raw::c_ushort {
        let mask = 4usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 2usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_c(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 4usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 2usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_ushort {
        let mask = 192usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        let val = (unit_field_val & mask) >> 6usize;
        unsafe { ::std::mem::transmute(val as u16) }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_ushort) {
        let mask = 192usize as u8;
        let val = val as u16 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_1) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 6usize) & mask;
        self._bitfield_1 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_1(a: ::std::os::raw::c_ushort,
                          b: ::std::os::raw::c_ushort,
                          c: ::std::os::raw::c_ushort,
                          d: ::std::os::raw::c_ushort) -> u8 {
        let bitfield_unit_val =
            {
                let bitfield_unit_val =
                    {
                        let bitfield_unit_val =
                            {
                                let bitfield_unit_val = { 0 };
                                let a = a as u16 as u8;
                                let mask = 1usize as u8;
                                let a = (a << 0usize) & mask;
                                bitfield_unit_val | a
                            };
                        let b = b as u16 as u8;
                        let mask = 2usize as u8;
                        let b = (b << 1usize) & mask;
                        bitfield_unit_val | b
                    };
                let c = c as u16 as u8;
                let mask = 4usize as u8;
                let c = (c << 2usize) & mask;
                bitfield_unit_val | c
            };
        let d = d as u16 as u8;
        let mask = 192usize as u8;
        let d = (d << 6usize) & mask;
        bitfield_unit_val | d
    }
    #[inline]
    pub fn f(&self) -> ::std::os::raw::c_uint {
        let mask = 3usize as u8;
        let unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_2) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_f(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 3usize as u8;
        let val = val as u32 as u8;
        let mut unit_field_val: u8 =
            unsafe { ::std::mem::transmute(self._bitfield_2) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_2 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_2(f: ::std::os::raw::c_uint) -> u8 {
        let bitfield_unit_val = { 0 };
        let f = f as u32 as u8;
        let mask = 3usize as u8;
        let f = (f << 0usize) & mask;
        bitfield_unit_val | f
    }
    #[inline]
    pub fn g(&self) -> ::std::os::raw::c_uint {
        let mask = 4294967295usize as u32;
        let unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_3) };
        let val = (unit_field_val & mask) >> 0usize;
        unsafe { ::std::mem::transmute(val as u32) }
    }
    #[inline]
    pub fn set_g(&mut self, val: ::std::os::raw::c_uint) {
        let mask = 4294967295usize as u32;
        let val = val as u32 as u32;
        let mut unit_field_val: u32 =
            unsafe { ::std::mem::transmute(self._bitfield_3) };
        unit_field_val &= !mask;
        unit_field_val |= (val << 0usize) & mask;
        self._bitfield_3 = unsafe { ::std::mem::transmute(unit_field_val) };
    }
    #[inline]
    pub fn new_bitfield_3(g: ::std::os::raw::c_uint) -> u32 {
        let bitfield_unit_val = { 0 };
        let g = g as u32 as u32;
        let mask = 4294967295usize as u32;
        let g = (g << 0usize) & mask;
        bitfield_unit_val | g
    }
}