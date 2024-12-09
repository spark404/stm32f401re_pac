#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSRrs>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSRrs>;
#[doc = "Field `LSION` reader - Internal low-speed oscillator enable"]
pub type LSION_R = crate::BitReader;
#[doc = "Field `LSION` writer - Internal low-speed oscillator enable"]
pub type LSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSIRDY` reader - Internal low-speed oscillator ready"]
pub type LSIRDY_R = crate::BitReader;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RMVF_R = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RMVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BORRSTF_R = crate::BitReader;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub type BORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADRSTF` reader - PIN reset flag"]
pub type PADRSTF_R = crate::BitReader;
#[doc = "Field `PADRSTF` writer - PIN reset flag"]
pub type PADRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PORRSTF_R = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PORRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - Software reset flag"]
pub type SFTRSTF_R = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - Software reset flag"]
pub type SFTRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGRSTF` reader - Independent watchdog reset flag"]
pub type WDGRSTF_R = crate::BitReader;
#[doc = "Field `WDGRSTF` writer - Independent watchdog reset flag"]
pub type WDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRSTF` reader - Window watchdog reset flag"]
pub type WWDGRSTF_R = crate::BitReader;
#[doc = "Field `WWDGRSTF` writer - Window watchdog reset flag"]
pub type WWDGRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWRRSTF` reader - Low-power reset flag"]
pub type LPWRRSTF_R = crate::BitReader;
#[doc = "Field `LPWRRSTF` writer - Low-power reset flag"]
pub type LPWRRSTF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal low-speed oscillator ready"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn padrstf(&self) -> PADRSTF_R {
        PADRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PORRSTF_R {
        PORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn wdgrstf(&self) -> WDGRSTF_R {
        WDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSR")
            .field("lpwrrstf", &self.lpwrrstf())
            .field("wwdgrstf", &self.wwdgrstf())
            .field("wdgrstf", &self.wdgrstf())
            .field("sftrstf", &self.sftrstf())
            .field("porrstf", &self.porrstf())
            .field("padrstf", &self.padrstf())
            .field("borrstf", &self.borrstf())
            .field("rmvf", &self.rmvf())
            .field("lsirdy", &self.lsirdy())
            .field("lsion", &self.lsion())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Internal low-speed oscillator enable"]
    #[inline(always)]
    pub fn lsion(&mut self) -> LSION_W<CSRrs> {
        LSION_W::new(self, 0)
    }
    #[doc = "Bit 24 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RMVF_W<CSRrs> {
        RMVF_W::new(self, 24)
    }
    #[doc = "Bit 25 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BORRSTF_W<CSRrs> {
        BORRSTF_W::new(self, 25)
    }
    #[doc = "Bit 26 - PIN reset flag"]
    #[inline(always)]
    pub fn padrstf(&mut self) -> PADRSTF_W<CSRrs> {
        PADRSTF_W::new(self, 26)
    }
    #[doc = "Bit 27 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PORRSTF_W<CSRrs> {
        PORRSTF_W::new(self, 27)
    }
    #[doc = "Bit 28 - Software reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SFTRSTF_W<CSRrs> {
        SFTRSTF_W::new(self, 28)
    }
    #[doc = "Bit 29 - Independent watchdog reset flag"]
    #[inline(always)]
    pub fn wdgrstf(&mut self) -> WDGRSTF_W<CSRrs> {
        WDGRSTF_W::new(self, 29)
    }
    #[doc = "Bit 30 - Window watchdog reset flag"]
    #[inline(always)]
    pub fn wwdgrstf(&mut self) -> WWDGRSTF_W<CSRrs> {
        WWDGRSTF_W::new(self, 30)
    }
    #[doc = "Bit 31 - Low-power reset flag"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LPWRRSTF_W<CSRrs> {
        LPWRRSTF_W::new(self, 31)
    }
}
#[doc = "clock control & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSRrs;
impl crate::RegisterSpec for CSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSRrs {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0x0e00_0000"]
impl crate::Resettable for CSRrs {
    const RESET_VALUE: u32 = 0x0e00_0000;
}
