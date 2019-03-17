#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXT"]
    pub ext: EXT,
}
#[doc = "EXT"]
pub struct EXT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "EXT"]
pub mod ext;
