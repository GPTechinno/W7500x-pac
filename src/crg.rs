#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Oscillator power down register"]
    pub osc_pdr: crate::Reg<osc_pdr::OSC_PDR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - PLL power down register"]
    pub pll_pdr: crate::Reg<pll_pdr::PLL_PDR_SPEC>,
    #[doc = "0x14 - PLL frequency calculating register"]
    pub pll_fcr: crate::Reg<pll_fcr::PLL_FCR_SPEC>,
    #[doc = "0x18 - PLL output enable register"]
    pub pll_oer: crate::Reg<pll_oer::PLL_OER_SPEC>,
    #[doc = "0x1c - PLL bypass register"]
    pub pll_bpr: crate::Reg<pll_bpr::PLL_BPR_SPEC>,
    #[doc = "0x20 - PLL input frequency select register"]
    pub pll_ifsr: crate::Reg<pll_ifsr::PLL_IFSR_SPEC>,
    _reserved6: [u8; 0x0c],
    #[doc = "0x30 - FCLK source select register"]
    pub fclk_ssr: crate::Reg<fclk_ssr::FCLK_SSR_SPEC>,
    #[doc = "0x34 - FCLK prescale value select register"]
    pub fclk_pvsr: crate::Reg<fclk_pvsr::FCLK_PVSR_SPEC>,
    _reserved8: [u8; 0x08],
    #[doc = "0x40 - SSPCLK source select register"]
    pub sspclk_ssr: crate::Reg<sspclk_ssr::SSPCLK_SSR_SPEC>,
    #[doc = "0x44 - SSPCLK prescale value select register"]
    pub sspclk_pvsr: crate::Reg<sspclk_pvsr::SSPCLK_PVSR_SPEC>,
    _reserved10: [u8; 0x18],
    #[doc = "0x60 - ADCCLK source select register"]
    pub adcclk_ssr: crate::Reg<adcclk_ssr::ADCCLK_SSR_SPEC>,
    #[doc = "0x64 - ADCCLK prescale value select register"]
    pub adcclk_pvsr: crate::Reg<adcclk_pvsr::ADCCLK_PVSR_SPEC>,
    _reserved12: [u8; 0x08],
    #[doc = "0x70 - TIMER0CLK source select register"]
    pub timer0clk_ssr: crate::Reg<timer0clk_ssr::TIMER0CLK_SSR_SPEC>,
    #[doc = "0x74 - TIMER0CLK prescale value select register"]
    pub timer0clk_pvsr: crate::Reg<timer0clk_pvsr::TIMER0CLK_PVSR_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x80 - TIMER1CLK source select register"]
    pub timer1clk_ssr: crate::Reg<timer1clk_ssr::TIMER1CLK_SSR_SPEC>,
    #[doc = "0x84 - TIMER1CLK prescale value select register"]
    pub timer1clk_pvsr: crate::Reg<timer1clk_pvsr::TIMER1CLK_PVSR_SPEC>,
    _reserved16: [u8; 0x28],
    #[doc = "0xb0 - PWM0CLK source select register"]
    pub pwm0clk_ssr: crate::Reg<pwm0clk_ssr::PWM0CLK_SSR_SPEC>,
    #[doc = "0xb4 - PWM0CLK prescale value select register"]
    pub pwm0clk_pvsr: crate::Reg<pwm0clk_pvsr::PWM0CLK_PVSR_SPEC>,
    _reserved18: [u8; 0x08],
    #[doc = "0xc0 - PWM1CLK source select register"]
    pub pwm1clk_ssr: crate::Reg<pwm1clk_ssr::PWM1CLK_SSR_SPEC>,
    #[doc = "0xc4 - PWM1CLK prescale value select register"]
    pub pwm1clk_pvsr: crate::Reg<pwm1clk_pvsr::PWM1CLK_PVSR_SPEC>,
    _reserved20: [u8; 0x08],
    #[doc = "0xd0 - PWM2CLK source select register"]
    pub pwm2clk_ssr: crate::Reg<pwm2clk_ssr::PWM2CLK_SSR_SPEC>,
    #[doc = "0xd4 - PWM2CLK prescale value select register"]
    pub pwm2clk_pvsr: crate::Reg<pwm2clk_pvsr::PWM2CLK_PVSR_SPEC>,
    _reserved22: [u8; 0x08],
    #[doc = "0xe0 - PWM3CLK source select register"]
    pub pwm3clk_ssr: crate::Reg<pwm3clk_ssr::PWM3CLK_SSR_SPEC>,
    #[doc = "0xe4 - PWM3CLK prescale value select register"]
    pub pwm3clk_pvsr: crate::Reg<pwm3clk_pvsr::PWM3CLK_PVSR_SPEC>,
    _reserved24: [u8; 0x08],
    #[doc = "0xf0 - PWM4CLK source select register"]
    pub pwm4clk_ssr: crate::Reg<pwm4clk_ssr::PWM4CLK_SSR_SPEC>,
    #[doc = "0xf4 - PWM4CLK prescale value select register"]
    pub pwm4clk_pvsr: crate::Reg<pwm4clk_pvsr::PWM4CLK_PVSR_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0x100 - PWM5CLK source select register"]
    pub pwm5clk_ssr: crate::Reg<pwm5clk_ssr::PWM5CLK_SSR_SPEC>,
    #[doc = "0x104 - PWM5CLK prescale value select register"]
    pub pwm5clk_pvsr: crate::Reg<pwm5clk_pvsr::PWM5CLK_PVSR_SPEC>,
    _reserved28: [u8; 0x08],
    #[doc = "0x110 - PWM6CLK source select register"]
    pub pwm6clk_ssr: crate::Reg<pwm6clk_ssr::PWM6CLK_SSR_SPEC>,
    #[doc = "0x114 - PWM6CLK prescale value select register"]
    pub pwm6clk_pvsr: crate::Reg<pwm6clk_pvsr::PWM6CLK_PVSR_SPEC>,
    _reserved30: [u8; 0x08],
    #[doc = "0x120 - PWM7CLK source select register"]
    pub pwm7clk_ssr: crate::Reg<pwm7clk_ssr::PWM7CLK_SSR_SPEC>,
    #[doc = "0x124 - PWM7CLK prescale value select register"]
    pub pwm7clk_pvsr: crate::Reg<pwm7clk_pvsr::PWM7CLK_PVSR_SPEC>,
    _reserved32: [u8; 0x08],
    #[doc = "0x130 - RTCCLK source select register"]
    pub rtc_hs_ssr: crate::Reg<rtc_hs_ssr::RTC_HS_SSR_SPEC>,
    #[doc = "0x134 - RTCCLK prescale value select register"]
    pub rtc_hs_pvsr: crate::Reg<rtc_hs_pvsr::RTC_HS_PVSR_SPEC>,
    _reserved34: [u8; 0x04],
    #[doc = "0x13c - RTCCLK 32K select register"]
    pub rtc_ssr: crate::Reg<rtc_ssr::RTC_SSR_SPEC>,
    #[doc = "0x140 - WDOGCLK High Speed source select register"]
    pub wdogclk_hs_ssr: crate::Reg<wdogclk_hs_ssr::WDOGCLK_HS_SSR_SPEC>,
    #[doc = "0x144 - WDOGCLK High Speed prescale value select register"]
    pub wdogclk_hs_pvsr: crate::Reg<wdogclk_hs_pvsr::WDOGCLK_HS_PVSR_SPEC>,
    _reserved37: [u8; 0x08],
    #[doc = "0x150 - UARTCLK source select register"]
    pub uartclk_ssr: crate::Reg<uartclk_ssr::UARTCLK_SSR_SPEC>,
    #[doc = "0x154 - UARTCLK prescale value select register"]
    pub uartclk_pvsr: crate::Reg<uartclk_pvsr::UARTCLK_PVSR_SPEC>,
    _reserved39: [u8; 0x08],
    #[doc = "0x160 - MII clock enable control register"]
    pub miiclk_ecr: crate::Reg<miiclk_ecr::MIICLK_ECR_SPEC>,
    _reserved40: [u8; 0x0c],
    #[doc = "0x170 - Select clock source for monitoring (monitoring pin : PA_02)"]
    pub monclk_ssr: crate::Reg<monclk_ssr::MONCLK_SSR_SPEC>,
}
#[doc = "OSC_PDR register accessor: an alias for `Reg<OSC_PDR_SPEC>`"]
pub type OSC_PDR = crate::Reg<osc_pdr::OSC_PDR_SPEC>;
#[doc = "Oscillator power down register"]
pub mod osc_pdr;
#[doc = "PLL_PDR register accessor: an alias for `Reg<PLL_PDR_SPEC>`"]
pub type PLL_PDR = crate::Reg<pll_pdr::PLL_PDR_SPEC>;
#[doc = "PLL power down register"]
pub mod pll_pdr;
#[doc = "PLL_FCR register accessor: an alias for `Reg<PLL_FCR_SPEC>`"]
pub type PLL_FCR = crate::Reg<pll_fcr::PLL_FCR_SPEC>;
#[doc = "PLL frequency calculating register"]
pub mod pll_fcr;
#[doc = "PLL_OER register accessor: an alias for `Reg<PLL_OER_SPEC>`"]
pub type PLL_OER = crate::Reg<pll_oer::PLL_OER_SPEC>;
#[doc = "PLL output enable register"]
pub mod pll_oer;
#[doc = "PLL_BPR register accessor: an alias for `Reg<PLL_BPR_SPEC>`"]
pub type PLL_BPR = crate::Reg<pll_bpr::PLL_BPR_SPEC>;
#[doc = "PLL bypass register"]
pub mod pll_bpr;
#[doc = "PLL_IFSR register accessor: an alias for `Reg<PLL_IFSR_SPEC>`"]
pub type PLL_IFSR = crate::Reg<pll_ifsr::PLL_IFSR_SPEC>;
#[doc = "PLL input frequency select register"]
pub mod pll_ifsr;
#[doc = "FCLK_SSR register accessor: an alias for `Reg<FCLK_SSR_SPEC>`"]
pub type FCLK_SSR = crate::Reg<fclk_ssr::FCLK_SSR_SPEC>;
#[doc = "FCLK source select register"]
pub mod fclk_ssr;
#[doc = "FCLK_PVSR register accessor: an alias for `Reg<FCLK_PVSR_SPEC>`"]
pub type FCLK_PVSR = crate::Reg<fclk_pvsr::FCLK_PVSR_SPEC>;
#[doc = "FCLK prescale value select register"]
pub mod fclk_pvsr;
#[doc = "SSPCLK_SSR register accessor: an alias for `Reg<SSPCLK_SSR_SPEC>`"]
pub type SSPCLK_SSR = crate::Reg<sspclk_ssr::SSPCLK_SSR_SPEC>;
#[doc = "SSPCLK source select register"]
pub mod sspclk_ssr;
#[doc = "SSPCLK_PVSR register accessor: an alias for `Reg<SSPCLK_PVSR_SPEC>`"]
pub type SSPCLK_PVSR = crate::Reg<sspclk_pvsr::SSPCLK_PVSR_SPEC>;
#[doc = "SSPCLK prescale value select register"]
pub mod sspclk_pvsr;
#[doc = "ADCCLK_SSR register accessor: an alias for `Reg<ADCCLK_SSR_SPEC>`"]
pub type ADCCLK_SSR = crate::Reg<adcclk_ssr::ADCCLK_SSR_SPEC>;
#[doc = "ADCCLK source select register"]
pub mod adcclk_ssr;
#[doc = "ADCCLK_PVSR register accessor: an alias for `Reg<ADCCLK_PVSR_SPEC>`"]
pub type ADCCLK_PVSR = crate::Reg<adcclk_pvsr::ADCCLK_PVSR_SPEC>;
#[doc = "ADCCLK prescale value select register"]
pub mod adcclk_pvsr;
#[doc = "TIMER0CLK_SSR register accessor: an alias for `Reg<TIMER0CLK_SSR_SPEC>`"]
pub type TIMER0CLK_SSR = crate::Reg<timer0clk_ssr::TIMER0CLK_SSR_SPEC>;
#[doc = "TIMER0CLK source select register"]
pub mod timer0clk_ssr;
#[doc = "TIMER0CLK_PVSR register accessor: an alias for `Reg<TIMER0CLK_PVSR_SPEC>`"]
pub type TIMER0CLK_PVSR = crate::Reg<timer0clk_pvsr::TIMER0CLK_PVSR_SPEC>;
#[doc = "TIMER0CLK prescale value select register"]
pub mod timer0clk_pvsr;
#[doc = "TIMER1CLK_SSR register accessor: an alias for `Reg<TIMER1CLK_SSR_SPEC>`"]
pub type TIMER1CLK_SSR = crate::Reg<timer1clk_ssr::TIMER1CLK_SSR_SPEC>;
#[doc = "TIMER1CLK source select register"]
pub mod timer1clk_ssr;
#[doc = "TIMER1CLK_PVSR register accessor: an alias for `Reg<TIMER1CLK_PVSR_SPEC>`"]
pub type TIMER1CLK_PVSR = crate::Reg<timer1clk_pvsr::TIMER1CLK_PVSR_SPEC>;
#[doc = "TIMER1CLK prescale value select register"]
pub mod timer1clk_pvsr;
#[doc = "PWM0CLK_SSR register accessor: an alias for `Reg<PWM0CLK_SSR_SPEC>`"]
pub type PWM0CLK_SSR = crate::Reg<pwm0clk_ssr::PWM0CLK_SSR_SPEC>;
#[doc = "PWM0CLK source select register"]
pub mod pwm0clk_ssr;
#[doc = "PWM0CLK_PVSR register accessor: an alias for `Reg<PWM0CLK_PVSR_SPEC>`"]
pub type PWM0CLK_PVSR = crate::Reg<pwm0clk_pvsr::PWM0CLK_PVSR_SPEC>;
#[doc = "PWM0CLK prescale value select register"]
pub mod pwm0clk_pvsr;
#[doc = "PWM1CLK_SSR register accessor: an alias for `Reg<PWM1CLK_SSR_SPEC>`"]
pub type PWM1CLK_SSR = crate::Reg<pwm1clk_ssr::PWM1CLK_SSR_SPEC>;
#[doc = "PWM1CLK source select register"]
pub mod pwm1clk_ssr;
#[doc = "PWM1CLK_PVSR register accessor: an alias for `Reg<PWM1CLK_PVSR_SPEC>`"]
pub type PWM1CLK_PVSR = crate::Reg<pwm1clk_pvsr::PWM1CLK_PVSR_SPEC>;
#[doc = "PWM1CLK prescale value select register"]
pub mod pwm1clk_pvsr;
#[doc = "PWM2CLK_SSR register accessor: an alias for `Reg<PWM2CLK_SSR_SPEC>`"]
pub type PWM2CLK_SSR = crate::Reg<pwm2clk_ssr::PWM2CLK_SSR_SPEC>;
#[doc = "PWM2CLK source select register"]
pub mod pwm2clk_ssr;
#[doc = "PWM2CLK_PVSR register accessor: an alias for `Reg<PWM2CLK_PVSR_SPEC>`"]
pub type PWM2CLK_PVSR = crate::Reg<pwm2clk_pvsr::PWM2CLK_PVSR_SPEC>;
#[doc = "PWM2CLK prescale value select register"]
pub mod pwm2clk_pvsr;
#[doc = "PWM3CLK_SSR register accessor: an alias for `Reg<PWM3CLK_SSR_SPEC>`"]
pub type PWM3CLK_SSR = crate::Reg<pwm3clk_ssr::PWM3CLK_SSR_SPEC>;
#[doc = "PWM3CLK source select register"]
pub mod pwm3clk_ssr;
#[doc = "PWM3CLK_PVSR register accessor: an alias for `Reg<PWM3CLK_PVSR_SPEC>`"]
pub type PWM3CLK_PVSR = crate::Reg<pwm3clk_pvsr::PWM3CLK_PVSR_SPEC>;
#[doc = "PWM3CLK prescale value select register"]
pub mod pwm3clk_pvsr;
#[doc = "PWM4CLK_SSR register accessor: an alias for `Reg<PWM4CLK_SSR_SPEC>`"]
pub type PWM4CLK_SSR = crate::Reg<pwm4clk_ssr::PWM4CLK_SSR_SPEC>;
#[doc = "PWM4CLK source select register"]
pub mod pwm4clk_ssr;
#[doc = "PWM4CLK_PVSR register accessor: an alias for `Reg<PWM4CLK_PVSR_SPEC>`"]
pub type PWM4CLK_PVSR = crate::Reg<pwm4clk_pvsr::PWM4CLK_PVSR_SPEC>;
#[doc = "PWM4CLK prescale value select register"]
pub mod pwm4clk_pvsr;
#[doc = "PWM5CLK_SSR register accessor: an alias for `Reg<PWM5CLK_SSR_SPEC>`"]
pub type PWM5CLK_SSR = crate::Reg<pwm5clk_ssr::PWM5CLK_SSR_SPEC>;
#[doc = "PWM5CLK source select register"]
pub mod pwm5clk_ssr;
#[doc = "PWM5CLK_PVSR register accessor: an alias for `Reg<PWM5CLK_PVSR_SPEC>`"]
pub type PWM5CLK_PVSR = crate::Reg<pwm5clk_pvsr::PWM5CLK_PVSR_SPEC>;
#[doc = "PWM5CLK prescale value select register"]
pub mod pwm5clk_pvsr;
#[doc = "PWM6CLK_SSR register accessor: an alias for `Reg<PWM6CLK_SSR_SPEC>`"]
pub type PWM6CLK_SSR = crate::Reg<pwm6clk_ssr::PWM6CLK_SSR_SPEC>;
#[doc = "PWM6CLK source select register"]
pub mod pwm6clk_ssr;
#[doc = "PWM6CLK_PVSR register accessor: an alias for `Reg<PWM6CLK_PVSR_SPEC>`"]
pub type PWM6CLK_PVSR = crate::Reg<pwm6clk_pvsr::PWM6CLK_PVSR_SPEC>;
#[doc = "PWM6CLK prescale value select register"]
pub mod pwm6clk_pvsr;
#[doc = "PWM7CLK_SSR register accessor: an alias for `Reg<PWM7CLK_SSR_SPEC>`"]
pub type PWM7CLK_SSR = crate::Reg<pwm7clk_ssr::PWM7CLK_SSR_SPEC>;
#[doc = "PWM7CLK source select register"]
pub mod pwm7clk_ssr;
#[doc = "PWM7CLK_PVSR register accessor: an alias for `Reg<PWM7CLK_PVSR_SPEC>`"]
pub type PWM7CLK_PVSR = crate::Reg<pwm7clk_pvsr::PWM7CLK_PVSR_SPEC>;
#[doc = "PWM7CLK prescale value select register"]
pub mod pwm7clk_pvsr;
#[doc = "RTC_HS_SSR register accessor: an alias for `Reg<RTC_HS_SSR_SPEC>`"]
pub type RTC_HS_SSR = crate::Reg<rtc_hs_ssr::RTC_HS_SSR_SPEC>;
#[doc = "RTCCLK source select register"]
pub mod rtc_hs_ssr;
#[doc = "RTC_HS_PVSR register accessor: an alias for `Reg<RTC_HS_PVSR_SPEC>`"]
pub type RTC_HS_PVSR = crate::Reg<rtc_hs_pvsr::RTC_HS_PVSR_SPEC>;
#[doc = "RTCCLK prescale value select register"]
pub mod rtc_hs_pvsr;
#[doc = "RTC_SSR register accessor: an alias for `Reg<RTC_SSR_SPEC>`"]
pub type RTC_SSR = crate::Reg<rtc_ssr::RTC_SSR_SPEC>;
#[doc = "RTCCLK 32K select register"]
pub mod rtc_ssr;
#[doc = "WDOGCLK_HS_SSR register accessor: an alias for `Reg<WDOGCLK_HS_SSR_SPEC>`"]
pub type WDOGCLK_HS_SSR = crate::Reg<wdogclk_hs_ssr::WDOGCLK_HS_SSR_SPEC>;
#[doc = "WDOGCLK High Speed source select register"]
pub mod wdogclk_hs_ssr;
#[doc = "WDOGCLK_HS_PVSR register accessor: an alias for `Reg<WDOGCLK_HS_PVSR_SPEC>`"]
pub type WDOGCLK_HS_PVSR = crate::Reg<wdogclk_hs_pvsr::WDOGCLK_HS_PVSR_SPEC>;
#[doc = "WDOGCLK High Speed prescale value select register"]
pub mod wdogclk_hs_pvsr;
#[doc = "UARTCLK_SSR register accessor: an alias for `Reg<UARTCLK_SSR_SPEC>`"]
pub type UARTCLK_SSR = crate::Reg<uartclk_ssr::UARTCLK_SSR_SPEC>;
#[doc = "UARTCLK source select register"]
pub mod uartclk_ssr;
#[doc = "UARTCLK_PVSR register accessor: an alias for `Reg<UARTCLK_PVSR_SPEC>`"]
pub type UARTCLK_PVSR = crate::Reg<uartclk_pvsr::UARTCLK_PVSR_SPEC>;
#[doc = "UARTCLK prescale value select register"]
pub mod uartclk_pvsr;
#[doc = "MIICLK_ECR register accessor: an alias for `Reg<MIICLK_ECR_SPEC>`"]
pub type MIICLK_ECR = crate::Reg<miiclk_ecr::MIICLK_ECR_SPEC>;
#[doc = "MII clock enable control register"]
pub mod miiclk_ecr;
#[doc = "MONCLK_SSR register accessor: an alias for `Reg<MONCLK_SSR_SPEC>`"]
pub type MONCLK_SSR = crate::Reg<monclk_ssr::MONCLK_SSR_SPEC>;
#[doc = "Select clock source for monitoring (monitoring pin : PA_02)"]
pub mod monclk_ssr;
