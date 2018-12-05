#![doc = "Peripheral access API for STM32F103XX microcontrollers (generated using svd2rust v0.13.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.13.1/svd2rust/#peripheral-api"]
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
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMPER();
    fn RTC();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CHANNEL1();
    fn DMA1_CHANNEL2();
    fn DMA1_CHANNEL3();
    fn DMA1_CHANNEL4();
    fn DMA1_CHANNEL5();
    fn DMA1_CHANNEL6();
    fn DMA1_CHANNEL7();
    fn ADC();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTCALARM();
    fn USB_FS_WKUP();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn ADC3();
    fn FSMC();
    fn SDIO();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6();
    fn TIM7();
    fn DMA2_CHANNEL1();
    fn DMA2_CHANNEL2();
    fn DMA2_CHANNEL3();
    fn DMA2_CHANNEL4_5();
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
pub static __INTERRUPTS: [Vector; 60] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: TAMPER },
    Vector { _handler: RTC },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_CHANNEL1,
    },
    Vector {
        _handler: DMA1_CHANNEL2,
    },
    Vector {
        _handler: DMA1_CHANNEL3,
    },
    Vector {
        _handler: DMA1_CHANNEL4,
    },
    Vector {
        _handler: DMA1_CHANNEL5,
    },
    Vector {
        _handler: DMA1_CHANNEL6,
    },
    Vector {
        _handler: DMA1_CHANNEL7,
    },
    Vector { _handler: ADC },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector { _handler: RTCALARM },
    Vector {
        _handler: USB_FS_WKUP,
    },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector { _handler: ADC3 },
    Vector { _handler: FSMC },
    Vector { _handler: SDIO },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6 },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_CHANNEL1,
    },
    Vector {
        _handler: DMA2_CHANNEL2,
    },
    Vector {
        _handler: DMA2_CHANNEL3,
    },
    Vector {
        _handler: DMA2_CHANNEL4_5,
    },
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper interrupt"]
    TAMPER,
    #[doc = "3 - RTC global interrupt"]
    RTC,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line1 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 interrupt"]
    EXTI2,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 Channel1 global interrupt"]
    DMA1_CHANNEL1,
    #[doc = "12 - DMA1 Channel2 global interrupt"]
    DMA1_CHANNEL2,
    #[doc = "13 - DMA1 Channel3 global interrupt"]
    DMA1_CHANNEL3,
    #[doc = "14 - DMA1 Channel4 global interrupt"]
    DMA1_CHANNEL4,
    #[doc = "15 - DMA1 Channel5 global interrupt"]
    DMA1_CHANNEL5,
    #[doc = "16 - DMA1 Channel6 global interrupt"]
    DMA1_CHANNEL6,
    #[doc = "17 - DMA1 Channel7 global interrupt"]
    DMA1_CHANNEL7,
    #[doc = "18 - ADC2 global interrupt"]
    ADC,
    #[doc = "19 - CAN1 TX interrupts"]
    CAN1_TX,
    #[doc = "20 - CAN1 RX0 interrupts"]
    CAN1_RX0,
    #[doc = "21 - CAN1 RX1 interrupt"]
    CAN1_RX1,
    #[doc = "22 - CAN1 SCE interrupt"]
    CAN1_SCE,
    #[doc = "23 - EXTI Line\\[9:5\\] interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break interrupt and TIM9 global interrupt"]
    TIM1_BRK_TIM9,
    #[doc = "25 - TIM1 Update interrupt and TIM10 global interrupt"]
    TIM1_UP_TIM10,
    #[doc = "26 - TIM1 Trigger and Commutation interrupts and TIM11 global interrupt"]
    TIM1_TRG_COM_TIM11,
    #[doc = "27 - TIM1 Capture Compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt"]
    I2C1_EV,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt"]
    I2C2_EV,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt"]
    USART1,
    #[doc = "38 - USART2 global interrupt"]
    USART2,
    #[doc = "39 - USART3 global interrupt"]
    USART3,
    #[doc = "40 - EXTI Line\\[15:10\\] interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC Alarms through EXTI line interrupt"]
    RTCALARM,
    #[doc = "42 - USB Device FS Wakeup through EXTI line interrupt"]
    USB_FS_WKUP,
    #[doc = "43 - TIM8 Break interrupt and TIM12 global interrupt"]
    TIM8_BRK_TIM12,
    #[doc = "44 - TIM8 Update interrupt and TIM13 global interrupt"]
    TIM8_UP_TIM13,
    #[doc = "45 - TIM8 Trigger and Commutation interrupts and TIM14 global interrupt"]
    TIM8_TRG_COM_TIM14,
    #[doc = "46 - TIM8 Capture Compare interrupt"]
    TIM8_CC,
    #[doc = "47 - ADC3 global interrupt"]
    ADC3,
    #[doc = "48 - FSMC global interrupt"]
    FSMC,
    #[doc = "49 - SDIO global interrupt"]
    SDIO,
    #[doc = "50 - TIM5 global interrupt"]
    TIM5,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "52 - UART4 global interrupt"]
    UART4,
    #[doc = "53 - UART5 global interrupt"]
    UART5,
    #[doc = "54 - TIM6 global interrupt"]
    TIM6,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 Channel1 global interrupt"]
    DMA2_CHANNEL1,
    #[doc = "57 - DMA2 Channel2 global interrupt"]
    DMA2_CHANNEL2,
    #[doc = "58 - DMA2 Channel3 global interrupt"]
    DMA2_CHANNEL3,
    #[doc = "59 - DMA2 Channel4 and DMA2 Channel5 global interrupt"]
    DMA2_CHANNEL4_5,
}
unsafe impl ::bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMPER => 2,
            Interrupt::RTC => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2 => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CHANNEL1 => 11,
            Interrupt::DMA1_CHANNEL2 => 12,
            Interrupt::DMA1_CHANNEL3 => 13,
            Interrupt::DMA1_CHANNEL4 => 14,
            Interrupt::DMA1_CHANNEL5 => 15,
            Interrupt::DMA1_CHANNEL6 => 16,
            Interrupt::DMA1_CHANNEL7 => 17,
            Interrupt::ADC => 18,
            Interrupt::CAN1_TX => 19,
            Interrupt::CAN1_RX0 => 20,
            Interrupt::CAN1_RX1 => 21,
            Interrupt::CAN1_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM9 => 24,
            Interrupt::TIM1_UP_TIM10 => 25,
            Interrupt::TIM1_TRG_COM_TIM11 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1 => 37,
            Interrupt::USART2 => 38,
            Interrupt::USART3 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTCALARM => 41,
            Interrupt::USB_FS_WKUP => 42,
            Interrupt::TIM8_BRK_TIM12 => 43,
            Interrupt::TIM8_UP_TIM13 => 44,
            Interrupt::TIM8_TRG_COM_TIM14 => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::ADC3 => 47,
            Interrupt::FSMC => 48,
            Interrupt::SDIO => 49,
            Interrupt::TIM5 => 50,
            Interrupt::SPI3 => 51,
            Interrupt::UART4 => 52,
            Interrupt::UART5 => 53,
            Interrupt::TIM6 => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CHANNEL1 => 56,
            Interrupt::DMA2_CHANNEL2 => 57,
            Interrupt::DMA2_CHANNEL3 => 58,
            Interrupt::DMA2_CHANNEL4_5 => 59,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[doc = "Flexible static memory controller"]
pub struct FSMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FSMC {}
impl FSMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const fsmc::RegisterBlock {
        2684354560 as *const _
    }
}
impl Deref for FSMC {
    type Target = fsmc::RegisterBlock;
    fn deref(&self) -> &fsmc::RegisterBlock {
        unsafe { &*FSMC::ptr() }
    }
}
#[doc = "Flexible static memory controller"]
pub mod fsmc;
#[doc = "Power control"]
pub struct PWR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWR {}
impl PWR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const pwr::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for PWR {
    type Target = pwr::RegisterBlock;
    fn deref(&self) -> &pwr::RegisterBlock {
        unsafe { &*PWR::ptr() }
    }
}
#[doc = "Power control"]
pub mod pwr;
#[doc = "Reset and clock control"]
pub struct RCC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCC {}
impl RCC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rcc::RegisterBlock {
        1073876992 as *const _
    }
}
impl Deref for RCC {
    type Target = rcc::RegisterBlock;
    fn deref(&self) -> &rcc::RegisterBlock {
        unsafe { &*RCC::ptr() }
    }
}
#[doc = "Reset and clock control"]
pub mod rcc;
#[doc = "General purpose I/O"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073809408 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General purpose I/O"]
pub mod gpioa;
#[doc = "GPIOB"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073810432 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "GPIOC"]
pub struct GPIOC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOC {}
impl GPIOC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for GPIOC {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOC::ptr() }
    }
}
#[doc = "GPIOD"]
pub struct GPIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOD {}
impl GPIOD {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073812480 as *const _
    }
}
impl Deref for GPIOD {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOD::ptr() }
    }
}
#[doc = "GPIOE"]
pub struct GPIOE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOE {}
impl GPIOE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073813504 as *const _
    }
}
impl Deref for GPIOE {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOE::ptr() }
    }
}
#[doc = "GPIOF"]
pub struct GPIOF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOF {}
impl GPIOF {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073814528 as *const _
    }
}
impl Deref for GPIOF {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOF::ptr() }
    }
}
#[doc = "GPIOG"]
pub struct GPIOG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOG {}
impl GPIOG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpioa::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for GPIOG {
    type Target = gpioa::RegisterBlock;
    fn deref(&self) -> &gpioa::RegisterBlock {
        unsafe { &*GPIOG::ptr() }
    }
}
#[doc = "Alternate function I/O"]
pub struct AFIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFIO {}
impl AFIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const afio::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for AFIO {
    type Target = afio::RegisterBlock;
    fn deref(&self) -> &afio::RegisterBlock {
        unsafe { &*AFIO::ptr() }
    }
}
#[doc = "Alternate function I/O"]
pub mod afio;
#[doc = "EXTI"]
pub struct EXTI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EXTI {}
impl EXTI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const exti::RegisterBlock {
        1073808384 as *const _
    }
}
impl Deref for EXTI {
    type Target = exti::RegisterBlock;
    fn deref(&self) -> &exti::RegisterBlock {
        unsafe { &*EXTI::ptr() }
    }
}
#[doc = "EXTI"]
pub mod exti;
#[doc = "DMA controller"]
pub struct DMA1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA1 {}
impl DMA1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073872896 as *const _
    }
}
impl Deref for DMA1 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA1::ptr() }
    }
}
#[doc = "DMA controller"]
pub mod dma1;
#[doc = "DMA2"]
pub struct DMA2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DMA2 {}
impl DMA2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dma1::RegisterBlock {
        1073873920 as *const _
    }
}
impl Deref for DMA2 {
    type Target = dma1::RegisterBlock;
    fn deref(&self) -> &dma1::RegisterBlock {
        unsafe { &*DMA2::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub struct SDIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SDIO {}
impl SDIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const sdio::RegisterBlock {
        1073840128 as *const _
    }
}
impl Deref for SDIO {
    type Target = sdio::RegisterBlock;
    fn deref(&self) -> &sdio::RegisterBlock {
        unsafe { &*SDIO::ptr() }
    }
}
#[doc = "Secure digital input/output interface"]
pub mod sdio;
#[doc = "Real time clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc::RegisterBlock {
        1073752064 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &rtc::RegisterBlock {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real time clock"]
pub mod rtc;
#[doc = "Backup registers"]
pub struct BKP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for BKP {}
impl BKP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const bkp::RegisterBlock {
        1073769476 as *const _
    }
}
impl Deref for BKP {
    type Target = bkp::RegisterBlock;
    fn deref(&self) -> &bkp::RegisterBlock {
        unsafe { &*BKP::ptr() }
    }
}
#[doc = "Backup registers"]
pub mod bkp;
#[doc = "Independent watchdog"]
pub struct IWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IWDG {}
impl IWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const iwdg::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for IWDG {
    type Target = iwdg::RegisterBlock;
    fn deref(&self) -> &iwdg::RegisterBlock {
        unsafe { &*IWDG::ptr() }
    }
}
#[doc = "Independent watchdog"]
pub mod iwdg;
#[doc = "Window watchdog"]
pub struct WWDG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WWDG {}
impl WWDG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wwdg::RegisterBlock {
        1073753088 as *const _
    }
}
impl Deref for WWDG {
    type Target = wwdg::RegisterBlock;
    fn deref(&self) -> &wwdg::RegisterBlock {
        unsafe { &*WWDG::ptr() }
    }
}
#[doc = "Window watchdog"]
pub mod wwdg;
#[doc = "Advanced timer"]
pub struct TIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM1 {}
impl TIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073818624 as *const _
    }
}
impl Deref for TIM1 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM1::ptr() }
    }
}
#[doc = "Advanced timer"]
pub mod tim1;
#[doc = "TIM8"]
pub struct TIM8 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM8 {}
impl TIM8 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim1::RegisterBlock {
        1073820672 as *const _
    }
}
impl Deref for TIM8 {
    type Target = tim1::RegisterBlock;
    fn deref(&self) -> &tim1::RegisterBlock {
        unsafe { &*TIM8::ptr() }
    }
}
#[doc = "General purpose timer"]
pub struct TIM2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM2 {}
impl TIM2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for TIM2 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM2::ptr() }
    }
}
#[doc = "General purpose timer"]
pub mod tim2;
#[doc = "TIM3"]
pub struct TIM3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM3 {}
impl TIM3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073742848 as *const _
    }
}
impl Deref for TIM3 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM3::ptr() }
    }
}
#[doc = "TIM4"]
pub struct TIM4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM4 {}
impl TIM4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073743872 as *const _
    }
}
impl Deref for TIM4 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM4::ptr() }
    }
}
#[doc = "TIM5"]
pub struct TIM5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM5 {}
impl TIM5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim2::RegisterBlock {
        1073744896 as *const _
    }
}
impl Deref for TIM5 {
    type Target = tim2::RegisterBlock;
    fn deref(&self) -> &tim2::RegisterBlock {
        unsafe { &*TIM5::ptr() }
    }
}
#[doc = "General purpose timer"]
pub struct TIM9 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM9 {}
impl TIM9 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073826816 as *const _
    }
}
impl Deref for TIM9 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM9::ptr() }
    }
}
#[doc = "General purpose timer"]
pub mod tim9;
#[doc = "TIM12"]
pub struct TIM12 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM12 {}
impl TIM12 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim9::RegisterBlock {
        1073747968 as *const _
    }
}
impl Deref for TIM12 {
    type Target = tim9::RegisterBlock;
    fn deref(&self) -> &tim9::RegisterBlock {
        unsafe { &*TIM12::ptr() }
    }
}
#[doc = "General purpose timer"]
pub struct TIM10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM10 {}
impl TIM10 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073827840 as *const _
    }
}
impl Deref for TIM10 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM10::ptr() }
    }
}
#[doc = "General purpose timer"]
pub mod tim10;
#[doc = "TIM11"]
pub struct TIM11 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM11 {}
impl TIM11 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073828864 as *const _
    }
}
impl Deref for TIM11 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM11::ptr() }
    }
}
#[doc = "TIM13"]
pub struct TIM13 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM13 {}
impl TIM13 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073748992 as *const _
    }
}
impl Deref for TIM13 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM13::ptr() }
    }
}
#[doc = "TIM14"]
pub struct TIM14 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM14 {}
impl TIM14 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim10::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for TIM14 {
    type Target = tim10::RegisterBlock;
    fn deref(&self) -> &tim10::RegisterBlock {
        unsafe { &*TIM14::ptr() }
    }
}
#[doc = "Basic timer"]
pub struct TIM6 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM6 {}
impl TIM6 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for TIM6 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM6::ptr() }
    }
}
#[doc = "Basic timer"]
pub mod tim6;
#[doc = "TIM7"]
pub struct TIM7 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIM7 {}
impl TIM7 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const tim6::RegisterBlock {
        1073746944 as *const _
    }
}
impl Deref for TIM7 {
    type Target = tim6::RegisterBlock;
    fn deref(&self) -> &tim6::RegisterBlock {
        unsafe { &*TIM7::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073763328 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter integrated circuit"]
pub mod i2c1;
#[doc = "I2C2"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const i2c1::RegisterBlock {
        1073764352 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c1::RegisterBlock;
    fn deref(&self) -> &i2c1::RegisterBlock {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Serial peripheral interface"]
pub mod spi1;
#[doc = "SPI2"]
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI2 {}
impl SPI2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073756160 as *const _
    }
}
impl Deref for SPI2 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI2::ptr() }
    }
}
#[doc = "SPI3"]
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI3 {}
impl SPI3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi1::RegisterBlock {
        1073757184 as *const _
    }
}
impl Deref for SPI3 {
    type Target = spi1::RegisterBlock;
    fn deref(&self) -> &spi1::RegisterBlock {
        unsafe { &*SPI3::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073821696 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
pub mod usart1;
#[doc = "USART2"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073759232 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USART3"]
pub struct USART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART3 {}
impl USART3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usart1::RegisterBlock {
        1073760256 as *const _
    }
}
impl Deref for USART3 {
    type Target = usart1::RegisterBlock;
    fn deref(&self) -> &usart1::RegisterBlock {
        unsafe { &*USART3::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub struct ADC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC1 {}
impl ADC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc1::RegisterBlock {
        1073816576 as *const _
    }
}
impl Deref for ADC1 {
    type Target = adc1::RegisterBlock;
    fn deref(&self) -> &adc1::RegisterBlock {
        unsafe { &*ADC1::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub mod adc1;
#[doc = "Analog to digital converter"]
pub struct ADC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC2 {}
impl ADC2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc2::RegisterBlock {
        1073817600 as *const _
    }
}
impl Deref for ADC2 {
    type Target = adc2::RegisterBlock;
    fn deref(&self) -> &adc2::RegisterBlock {
        unsafe { &*ADC2::ptr() }
    }
}
#[doc = "Analog to digital converter"]
pub mod adc2;
#[doc = "ADC3"]
pub struct ADC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC3 {}
impl ADC3 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc2::RegisterBlock {
        1073822720 as *const _
    }
}
impl Deref for ADC3 {
    type Target = adc2::RegisterBlock;
    fn deref(&self) -> &adc2::RegisterBlock {
        unsafe { &*ADC3::ptr() }
    }
}
#[doc = "Controller area network"]
pub struct CAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN {}
impl CAN {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const can::RegisterBlock {
        1073767424 as *const _
    }
}
impl Deref for CAN {
    type Target = can::RegisterBlock;
    fn deref(&self) -> &can::RegisterBlock {
        unsafe { &*CAN::ptr() }
    }
}
#[doc = "Controller area network"]
pub mod can;
#[doc = "Digital to analog converter"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dac::RegisterBlock {
        1073771520 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    fn deref(&self) -> &dac::RegisterBlock {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital to analog converter"]
pub mod dac;
#[doc = "Debug support"]
pub struct DBG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DBG {}
impl DBG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const dbg::RegisterBlock {
        3758366720 as *const _
    }
}
impl Deref for DBG {
    type Target = dbg::RegisterBlock;
    fn deref(&self) -> &dbg::RegisterBlock {
        unsafe { &*DBG::ptr() }
    }
}
#[doc = "Debug support"]
pub mod dbg;
#[doc = "Universal asynchronous receiver transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart4::RegisterBlock {
        1073761280 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart4::RegisterBlock;
    fn deref(&self) -> &uart4::RegisterBlock {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal asynchronous receiver transmitter"]
pub mod uart4;
#[doc = "Universal asynchronous receiver transmitter"]
pub struct UART5 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART5 {}
impl UART5 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart5::RegisterBlock {
        1073762304 as *const _
    }
}
impl Deref for UART5 {
    type Target = uart5::RegisterBlock;
    fn deref(&self) -> &uart5::RegisterBlock {
        unsafe { &*UART5::ptr() }
    }
}
#[doc = "Universal asynchronous receiver transmitter"]
pub mod uart5;
#[doc = "CRC calculation unit"]
pub struct CRC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CRC {}
impl CRC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const crc::RegisterBlock {
        1073885184 as *const _
    }
}
impl Deref for CRC {
    type Target = crc::RegisterBlock;
    fn deref(&self) -> &crc::RegisterBlock {
        unsafe { &*CRC::ptr() }
    }
}
#[doc = "CRC calculation unit"]
pub mod crc;
#[doc = "FLASH"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const flash::RegisterBlock {
        1073881088 as *const _
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    fn deref(&self) -> &flash::RegisterBlock {
        unsafe { &*FLASH::ptr() }
    }
}
#[doc = "FLASH"]
pub mod flash;
#[doc = "Universal serial bus full-speed device interface"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const usb::RegisterBlock {
        1073765376 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    fn deref(&self) -> &usb::RegisterBlock {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "Universal serial bus full-speed device interface"]
pub mod usb;
#[allow(renamed_and_removed_lints)]
#[allow(private_no_mangle_statics)]
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FSMC"]
    pub FSMC: FSMC,
    #[doc = "PWR"]
    pub PWR: PWR,
    #[doc = "RCC"]
    pub RCC: RCC,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "GPIOC"]
    pub GPIOC: GPIOC,
    #[doc = "GPIOD"]
    pub GPIOD: GPIOD,
    #[doc = "GPIOE"]
    pub GPIOE: GPIOE,
    #[doc = "GPIOF"]
    pub GPIOF: GPIOF,
    #[doc = "GPIOG"]
    pub GPIOG: GPIOG,
    #[doc = "AFIO"]
    pub AFIO: AFIO,
    #[doc = "EXTI"]
    pub EXTI: EXTI,
    #[doc = "DMA1"]
    pub DMA1: DMA1,
    #[doc = "DMA2"]
    pub DMA2: DMA2,
    #[doc = "SDIO"]
    pub SDIO: SDIO,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "BKP"]
    pub BKP: BKP,
    #[doc = "IWDG"]
    pub IWDG: IWDG,
    #[doc = "WWDG"]
    pub WWDG: WWDG,
    #[doc = "TIM1"]
    pub TIM1: TIM1,
    #[doc = "TIM8"]
    pub TIM8: TIM8,
    #[doc = "TIM2"]
    pub TIM2: TIM2,
    #[doc = "TIM3"]
    pub TIM3: TIM3,
    #[doc = "TIM4"]
    pub TIM4: TIM4,
    #[doc = "TIM5"]
    pub TIM5: TIM5,
    #[doc = "TIM9"]
    pub TIM9: TIM9,
    #[doc = "TIM12"]
    pub TIM12: TIM12,
    #[doc = "TIM10"]
    pub TIM10: TIM10,
    #[doc = "TIM11"]
    pub TIM11: TIM11,
    #[doc = "TIM13"]
    pub TIM13: TIM13,
    #[doc = "TIM14"]
    pub TIM14: TIM14,
    #[doc = "TIM6"]
    pub TIM6: TIM6,
    #[doc = "TIM7"]
    pub TIM7: TIM7,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "SPI2"]
    pub SPI2: SPI2,
    #[doc = "SPI3"]
    pub SPI3: SPI3,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USART3"]
    pub USART3: USART3,
    #[doc = "ADC1"]
    pub ADC1: ADC1,
    #[doc = "ADC2"]
    pub ADC2: ADC2,
    #[doc = "ADC3"]
    pub ADC3: ADC3,
    #[doc = "CAN"]
    pub CAN: CAN,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "DBG"]
    pub DBG: DBG,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "UART5"]
    pub UART5: UART5,
    #[doc = "CRC"]
    pub CRC: CRC,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "USB"]
    pub USB: USB,
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
            FSMC: FSMC {
                _marker: PhantomData,
            },
            PWR: PWR {
                _marker: PhantomData,
            },
            RCC: RCC {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            GPIOC: GPIOC {
                _marker: PhantomData,
            },
            GPIOD: GPIOD {
                _marker: PhantomData,
            },
            GPIOE: GPIOE {
                _marker: PhantomData,
            },
            GPIOF: GPIOF {
                _marker: PhantomData,
            },
            GPIOG: GPIOG {
                _marker: PhantomData,
            },
            AFIO: AFIO {
                _marker: PhantomData,
            },
            EXTI: EXTI {
                _marker: PhantomData,
            },
            DMA1: DMA1 {
                _marker: PhantomData,
            },
            DMA2: DMA2 {
                _marker: PhantomData,
            },
            SDIO: SDIO {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            BKP: BKP {
                _marker: PhantomData,
            },
            IWDG: IWDG {
                _marker: PhantomData,
            },
            WWDG: WWDG {
                _marker: PhantomData,
            },
            TIM1: TIM1 {
                _marker: PhantomData,
            },
            TIM8: TIM8 {
                _marker: PhantomData,
            },
            TIM2: TIM2 {
                _marker: PhantomData,
            },
            TIM3: TIM3 {
                _marker: PhantomData,
            },
            TIM4: TIM4 {
                _marker: PhantomData,
            },
            TIM5: TIM5 {
                _marker: PhantomData,
            },
            TIM9: TIM9 {
                _marker: PhantomData,
            },
            TIM12: TIM12 {
                _marker: PhantomData,
            },
            TIM10: TIM10 {
                _marker: PhantomData,
            },
            TIM11: TIM11 {
                _marker: PhantomData,
            },
            TIM13: TIM13 {
                _marker: PhantomData,
            },
            TIM14: TIM14 {
                _marker: PhantomData,
            },
            TIM6: TIM6 {
                _marker: PhantomData,
            },
            TIM7: TIM7 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            SPI2: SPI2 {
                _marker: PhantomData,
            },
            SPI3: SPI3 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USART3: USART3 {
                _marker: PhantomData,
            },
            ADC1: ADC1 {
                _marker: PhantomData,
            },
            ADC2: ADC2 {
                _marker: PhantomData,
            },
            ADC3: ADC3 {
                _marker: PhantomData,
            },
            CAN: CAN {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            DBG: DBG {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            UART5: UART5 {
                _marker: PhantomData,
            },
            CRC: CRC {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
        }
    }
}
