#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `SW0` reader - System clock switch"]
pub type SW0_R = crate::BitReader;
#[doc = "Field `SW0` writer - System clock switch"]
pub type SW0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW1` reader - System clock switch"]
pub type SW1_R = crate::BitReader;
#[doc = "Field `SW1` writer - System clock switch"]
pub type SW1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWS0` reader - System clock switch status"]
pub type SWS0_R = crate::BitReader;
#[doc = "Field `SWS1` reader - System clock switch status"]
pub type SWS1_R = crate::BitReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader;
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE2` reader - APB high-speed prescaler (APB2)"]
pub type PPRE2_R = crate::FieldReader;
#[doc = "Field `PPRE2` writer - APB high-speed prescaler (APB2)"]
pub type PPRE2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub type RTCPRE_R = crate::FieldReader;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub type RTCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MCO1` reader - Microcontroller clock output 1"]
pub type MCO1_R = crate::FieldReader;
#[doc = "Field `MCO1` writer - Microcontroller clock output 1"]
pub type MCO1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2SSRC` reader - I2S clock selection"]
pub type I2SSRC_R = crate::BitReader;
#[doc = "Field `I2SSRC` writer - I2S clock selection"]
pub type I2SSRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub type MCO1PRE_R = crate::FieldReader;
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub type MCO1PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub type MCO2PRE_R = crate::FieldReader;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO2` reader - Microcontroller clock output 2"]
pub type MCO2_R = crate::FieldReader;
#[doc = "Field `MCO2` writer - Microcontroller clock output 2"]
pub type MCO2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    pub fn sw0(&self) -> SW0_R {
        SW0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    pub fn sw1(&self) -> SW1_R {
        SW1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System clock switch status"]
    #[inline(always)]
    pub fn sws0(&self) -> SWS0_R {
        SWS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System clock switch status"]
    #[inline(always)]
    pub fn sws1(&self) -> SWS1_R {
        SWS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("mco2", &self.mco2())
            .field("mco2pre", &self.mco2pre())
            .field("mco1pre", &self.mco1pre())
            .field("i2ssrc", &self.i2ssrc())
            .field("mco1", &self.mco1())
            .field("rtcpre", &self.rtcpre())
            .field("ppre2", &self.ppre2())
            .field("ppre1", &self.ppre1())
            .field("hpre", &self.hpre())
            .field("sws1", &self.sws1())
            .field("sws0", &self.sws0())
            .field("sw1", &self.sw1())
            .field("sw0", &self.sw0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - System clock switch"]
    #[inline(always)]
    pub fn sw0(&mut self) -> SW0_W<CFGRrs> {
        SW0_W::new(self, 0)
    }
    #[doc = "Bit 1 - System clock switch"]
    #[inline(always)]
    pub fn sw1(&mut self) -> SW1_W<CFGRrs> {
        SW1_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W<CFGRrs> {
        HPRE_W::new(self, 4)
    }
    #[doc = "Bits 10:12 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&mut self) -> PPRE1_W<CFGRrs> {
        PPRE1_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - APB high-speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&mut self) -> PPRE2_W<CFGRrs> {
        PPRE2_W::new(self, 13)
    }
    #[doc = "Bits 16:20 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W<CFGRrs> {
        RTCPRE_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - Microcontroller clock output 1"]
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W<CFGRrs> {
        MCO1_W::new(self, 21)
    }
    #[doc = "Bit 23 - I2S clock selection"]
    #[inline(always)]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<CFGRrs> {
        I2SSRC_W::new(self, 23)
    }
    #[doc = "Bits 24:26 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W<CFGRrs> {
        MCO1PRE_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W<CFGRrs> {
        MCO2PRE_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - Microcontroller clock output 2"]
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W<CFGRrs> {
        MCO2_W::new(self, 30)
    }
}
#[doc = "clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
