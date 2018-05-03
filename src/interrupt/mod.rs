use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm ! ( "\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    " ) ;
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm ! ( "\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    " ) ;
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak WWDG\nWWDG = DH_TRAMPOLINE\n.weak PVD\nPVD = DH_TRAMPOLINE\n.weak TAMPER\nTAMPER = DH_TRAMPOLINE\n.weak RTC\nRTC = DH_TRAMPOLINE\n.weak FLASH\nFLASH = DH_TRAMPOLINE\n.weak RCC\nRCC = DH_TRAMPOLINE\n.weak EXTI0\nEXTI0 = DH_TRAMPOLINE\n.weak EXTI1\nEXTI1 = DH_TRAMPOLINE\n.weak EXTI2\nEXTI2 = DH_TRAMPOLINE\n.weak EXTI3\nEXTI3 = DH_TRAMPOLINE\n.weak EXTI4\nEXTI4 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL1\nDMA1_CHANNEL1 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL2\nDMA1_CHANNEL2 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL3\nDMA1_CHANNEL3 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL4\nDMA1_CHANNEL4 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL5\nDMA1_CHANNEL5 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL6\nDMA1_CHANNEL6 = DH_TRAMPOLINE\n.weak DMA1_CHANNEL7\nDMA1_CHANNEL7 = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak CAN1_TX\nCAN1_TX = DH_TRAMPOLINE\n.weak CAN1_RX0\nCAN1_RX0 = DH_TRAMPOLINE\n.weak CAN1_RX1\nCAN1_RX1 = DH_TRAMPOLINE\n.weak CAN1_SCE\nCAN1_SCE = DH_TRAMPOLINE\n.weak EXTI9_5\nEXTI9_5 = DH_TRAMPOLINE\n.weak TIM1_BRK_TIM9\nTIM1_BRK_TIM9 = DH_TRAMPOLINE\n.weak TIM1_UP_TIM10\nTIM1_UP_TIM10 = DH_TRAMPOLINE\n.weak TIM1_TRG_COM_TIM11\nTIM1_TRG_COM_TIM11 = DH_TRAMPOLINE\n.weak TIM1_CC\nTIM1_CC = DH_TRAMPOLINE\n.weak TIM2\nTIM2 = DH_TRAMPOLINE\n.weak TIM3\nTIM3 = DH_TRAMPOLINE\n.weak TIM4\nTIM4 = DH_TRAMPOLINE\n.weak I2C1_EV\nI2C1_EV = DH_TRAMPOLINE\n.weak I2C1_ER\nI2C1_ER = DH_TRAMPOLINE\n.weak I2C2_EV\nI2C2_EV = DH_TRAMPOLINE\n.weak I2C2_ER\nI2C2_ER = DH_TRAMPOLINE\n.weak SPI1\nSPI1 = DH_TRAMPOLINE\n.weak SPI2\nSPI2 = DH_TRAMPOLINE\n.weak USART1\nUSART1 = DH_TRAMPOLINE\n.weak USART2\nUSART2 = DH_TRAMPOLINE\n.weak USART3\nUSART3 = DH_TRAMPOLINE\n.weak EXTI15_10\nEXTI15_10 = DH_TRAMPOLINE\n.weak RTCALARM\nRTCALARM = DH_TRAMPOLINE\n.weak USB_FS_WKUP\nUSB_FS_WKUP = DH_TRAMPOLINE\n.weak TIM8_BRK_TIM12\nTIM8_BRK_TIM12 = DH_TRAMPOLINE\n.weak TIM8_UP_TIM13\nTIM8_UP_TIM13 = DH_TRAMPOLINE\n.weak TIM8_TRG_COM_TIM14\nTIM8_TRG_COM_TIM14 = DH_TRAMPOLINE\n.weak TIM8_CC\nTIM8_CC = DH_TRAMPOLINE\n.weak ADC3\nADC3 = DH_TRAMPOLINE\n.weak FSMC\nFSMC = DH_TRAMPOLINE\n.weak SDIO\nSDIO = DH_TRAMPOLINE\n.weak TIM5\nTIM5 = DH_TRAMPOLINE\n.weak SPI3\nSPI3 = DH_TRAMPOLINE\n.weak UART4\nUART4 = DH_TRAMPOLINE\n.weak UART5\nUART5 = DH_TRAMPOLINE\n.weak TIM6\nTIM6 = DH_TRAMPOLINE\n.weak TIM7\nTIM7 = DH_TRAMPOLINE\n.weak DMA2_CHANNEL1\nDMA2_CHANNEL1 = DH_TRAMPOLINE\n.weak DMA2_CHANNEL2\nDMA2_CHANNEL2 = DH_TRAMPOLINE\n.weak DMA2_CHANNEL3\nDMA2_CHANNEL3 = DH_TRAMPOLINE\n.weak DMA2_CHANNEL4_5\nDMA2_CHANNEL4_5 = DH_TRAMPOLINE" ) ;
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
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 60] = [
    Some(WWDG),
    Some(PVD),
    Some(TAMPER),
    Some(RTC),
    Some(FLASH),
    Some(RCC),
    Some(EXTI0),
    Some(EXTI1),
    Some(EXTI2),
    Some(EXTI3),
    Some(EXTI4),
    Some(DMA1_CHANNEL1),
    Some(DMA1_CHANNEL2),
    Some(DMA1_CHANNEL3),
    Some(DMA1_CHANNEL4),
    Some(DMA1_CHANNEL5),
    Some(DMA1_CHANNEL6),
    Some(DMA1_CHANNEL7),
    Some(ADC),
    Some(CAN1_TX),
    Some(CAN1_RX0),
    Some(CAN1_RX1),
    Some(CAN1_SCE),
    Some(EXTI9_5),
    Some(TIM1_BRK_TIM9),
    Some(TIM1_UP_TIM10),
    Some(TIM1_TRG_COM_TIM11),
    Some(TIM1_CC),
    Some(TIM2),
    Some(TIM3),
    Some(TIM4),
    Some(I2C1_EV),
    Some(I2C1_ER),
    Some(I2C2_EV),
    Some(I2C2_ER),
    Some(SPI1),
    Some(SPI2),
    Some(USART1),
    Some(USART2),
    Some(USART3),
    Some(EXTI15_10),
    Some(RTCALARM),
    Some(USB_FS_WKUP),
    Some(TIM8_BRK_TIM12),
    Some(TIM8_UP_TIM13),
    Some(TIM8_TRG_COM_TIM14),
    Some(TIM8_CC),
    Some(ADC3),
    Some(FSMC),
    Some(SDIO),
    Some(TIM5),
    Some(SPI3),
    Some(UART4),
    Some(UART5),
    Some(TIM6),
    Some(TIM7),
    Some(DMA2_CHANNEL1),
    Some(DMA2_CHANNEL2),
    Some(DMA2_CHANNEL3),
    Some(DMA2_CHANNEL4_5),
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
    #[doc = "23 - EXTI Line[9:5] interrupts"]
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
    #[doc = "40 - EXTI Line[15:10] interrupts"]
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
unsafe impl Nr for Interrupt {
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
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::WWDG),
            1 => Ok(Interrupt::PVD),
            2 => Ok(Interrupt::TAMPER),
            3 => Ok(Interrupt::RTC),
            4 => Ok(Interrupt::FLASH),
            5 => Ok(Interrupt::RCC),
            6 => Ok(Interrupt::EXTI0),
            7 => Ok(Interrupt::EXTI1),
            8 => Ok(Interrupt::EXTI2),
            9 => Ok(Interrupt::EXTI3),
            10 => Ok(Interrupt::EXTI4),
            11 => Ok(Interrupt::DMA1_CHANNEL1),
            12 => Ok(Interrupt::DMA1_CHANNEL2),
            13 => Ok(Interrupt::DMA1_CHANNEL3),
            14 => Ok(Interrupt::DMA1_CHANNEL4),
            15 => Ok(Interrupt::DMA1_CHANNEL5),
            16 => Ok(Interrupt::DMA1_CHANNEL6),
            17 => Ok(Interrupt::DMA1_CHANNEL7),
            18 => Ok(Interrupt::ADC),
            19 => Ok(Interrupt::CAN1_TX),
            20 => Ok(Interrupt::CAN1_RX0),
            21 => Ok(Interrupt::CAN1_RX1),
            22 => Ok(Interrupt::CAN1_SCE),
            23 => Ok(Interrupt::EXTI9_5),
            24 => Ok(Interrupt::TIM1_BRK_TIM9),
            25 => Ok(Interrupt::TIM1_UP_TIM10),
            26 => Ok(Interrupt::TIM1_TRG_COM_TIM11),
            27 => Ok(Interrupt::TIM1_CC),
            28 => Ok(Interrupt::TIM2),
            29 => Ok(Interrupt::TIM3),
            30 => Ok(Interrupt::TIM4),
            31 => Ok(Interrupt::I2C1_EV),
            32 => Ok(Interrupt::I2C1_ER),
            33 => Ok(Interrupt::I2C2_EV),
            34 => Ok(Interrupt::I2C2_ER),
            35 => Ok(Interrupt::SPI1),
            36 => Ok(Interrupt::SPI2),
            37 => Ok(Interrupt::USART1),
            38 => Ok(Interrupt::USART2),
            39 => Ok(Interrupt::USART3),
            40 => Ok(Interrupt::EXTI15_10),
            41 => Ok(Interrupt::RTCALARM),
            42 => Ok(Interrupt::USB_FS_WKUP),
            43 => Ok(Interrupt::TIM8_BRK_TIM12),
            44 => Ok(Interrupt::TIM8_UP_TIM13),
            45 => Ok(Interrupt::TIM8_TRG_COM_TIM14),
            46 => Ok(Interrupt::TIM8_CC),
            47 => Ok(Interrupt::ADC3),
            48 => Ok(Interrupt::FSMC),
            49 => Ok(Interrupt::SDIO),
            50 => Ok(Interrupt::TIM5),
            51 => Ok(Interrupt::SPI3),
            52 => Ok(Interrupt::UART4),
            53 => Ok(Interrupt::UART5),
            54 => Ok(Interrupt::TIM6),
            55 => Ok(Interrupt::TIM7),
            56 => Ok(Interrupt::DMA2_CHANNEL1),
            57 => Ok(Interrupt::DMA2_CHANNEL2),
            58 => Ok(Interrupt::DMA2_CHANNEL3),
            59 => Ok(Interrupt::DMA2_CHANNEL4_5),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
