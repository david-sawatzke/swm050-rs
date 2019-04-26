#![doc = "Peripheral access API for SWM050 microcontrollers (generated using svd2rust v0.14.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.14.0/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {
    fn TMRSE0();
    fn TMRSE1();
    fn WDT();
    fn ACMP();
    fn GPIOA0();
    fn GPIOA1();
    fn GPIOA2();
    fn GPIOA3();
    fn GPIOA4();
    fn GPIOA5();
    fn GPIOA6();
    fn GPIOA7();
    fn GPIOA9();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 13] = [
    Vector { _handler: TMRSE0 },
    Vector { _handler: TMRSE1 },
    Vector { _handler: WDT },
    Vector { _handler: ACMP },
    Vector { _handler: GPIOA0 },
    Vector { _handler: GPIOA1 },
    Vector { _handler: GPIOA2 },
    Vector { _handler: GPIOA3 },
    Vector { _handler: GPIOA4 },
    Vector { _handler: GPIOA5 },
    Vector { _handler: GPIOA6 },
    Vector { _handler: GPIOA7 },
    Vector { _handler: GPIOA9 },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - TMRSE0"]
    TMRSE0,
    #[doc = "1 - TMRSE1"]
    TMRSE1,
    #[doc = "2 - WDT"]
    WDT,
    #[doc = "3 - ACMP"]
    ACMP,
    #[doc = "4 - GPIOA0"]
    GPIOA0,
    #[doc = "5 - GPIOA1"]
    GPIOA1,
    #[doc = "6 - GPIOA2"]
    GPIOA2,
    #[doc = "7 - GPIOA3"]
    GPIOA3,
    #[doc = "8 - GPIOA4"]
    GPIOA4,
    #[doc = "9 - GPIOA5"]
    GPIOA5,
    #[doc = "10 - GPIOA6"]
    GPIOA6,
    #[doc = "11 - GPIOA7"]
    GPIOA7,
    #[doc = "12 - GPIOA9"]
    GPIOA9,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::TMRSE0 => 0,
            Interrupt::TMRSE1 => 1,
            Interrupt::WDT => 2,
            Interrupt::ACMP => 3,
            Interrupt::GPIOA0 => 4,
            Interrupt::GPIOA1 => 5,
            Interrupt::GPIOA2 => 6,
            Interrupt::GPIOA3 => 7,
            Interrupt::GPIOA4 => 8,
            Interrupt::GPIOA5 => 9,
            Interrupt::GPIOA6 => 10,
            Interrupt::GPIOA7 => 11,
            Interrupt::GPIOA9 => 12,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Registers group"]
pub struct SYS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYS {}
impl SYS {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sys::RegisterBlock {
        1074724864 as *const _
    }
}
impl Deref for SYS {
    type Target = sys::RegisterBlock;
    fn deref(&self) -> &sys::RegisterBlock {
        unsafe { &*SYS::ptr() }
    }
}
#[doc = "Registers group"]
pub mod sys;
#[doc = "Registers group"]
pub struct PORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT {}
impl PORT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const port::RegisterBlock {
        1073741872 as *const _
    }
}
impl Deref for PORT {
    type Target = port::RegisterBlock;
    fn deref(&self) -> &port::RegisterBlock {
        unsafe { &*PORT::ptr() }
    }
}
#[doc = "Registers group"]
pub mod port;
#[doc = "Registers group"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "Registers group"]
pub mod gpioa;
#[doc = "Registers group"]
pub struct ACMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACMP {}
impl ACMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const acmp::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for ACMP {
    type Target = acmp::RegisterBlock;
    fn deref(&self) -> &acmp::RegisterBlock {
        unsafe { &*ACMP::ptr() }
    }
}
#[doc = "Registers group"]
pub mod acmp;
#[doc = "Registers group"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073844224 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Registers group"]
pub mod wdt;
#[doc = "Registers group"]
pub struct TMRSE0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMRSE0 {}
impl TMRSE0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmrse0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TMRSE0 {
    type Target = tmrse0::RegisterBlock;
    fn deref(&self) -> &tmrse0::RegisterBlock {
        unsafe { &*TMRSE0::ptr() }
    }
}
#[doc = "Registers group"]
pub mod tmrse0;
#[doc = "Registers group"]
pub struct TMRSE1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TMRSE1 {}
impl TMRSE1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tmrse0::RegisterBlock {
        1073751040 as *const _
    }
}
impl Deref for TMRSE1 {
    type Target = tmrse0::RegisterBlock;
    fn deref(&self) -> &tmrse0::RegisterBlock {
        unsafe { &*TMRSE1::ptr() }
    }
}
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SYS"]
    pub SYS: SYS,
    #[doc = "PORT"]
    pub PORT: PORT,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "ACMP"]
    pub ACMP: ACMP,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "TMRSE0"]
    pub TMRSE0: TMRSE0,
    #[doc = "TMRSE1"]
    pub TMRSE1: TMRSE1,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SYS: SYS {
                _marker: PhantomData,
            },
            PORT: PORT {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            ACMP: ACMP {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            TMRSE0: TMRSE0 {
                _marker: PhantomData,
            },
            TMRSE1: TMRSE1 {
                _marker: PhantomData,
            },
        }
    }
}
