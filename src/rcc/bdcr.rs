#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BDCRrs>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BDCRrs>;
#[doc = "Field `LSEON` reader - External low-speed oscillator enable"]
pub type LSEON_R = crate::BitReader;
#[doc = "Field `LSEON` writer - External low-speed oscillator enable"]
pub type LSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - External low-speed oscillator ready"]
pub type LSERDY_R = crate::BitReader;
#[doc = "Field `LSEBYP` reader - External low-speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader;
#[doc = "Field `LSEBYP` writer - External low-speed oscillator bypass"]
pub type LSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL0` reader - RTC clock source selection"]
pub type RTCSEL0_R = crate::BitReader;
#[doc = "Field `RTCSEL0` writer - RTC clock source selection"]
pub type RTCSEL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSEL1` reader - RTC clock source selection"]
pub type RTCSEL1_R = crate::BitReader;
#[doc = "Field `RTCSEL1` writer - RTC clock source selection"]
pub type RTCSEL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader;
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External low-speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel0(&self) -> RTCSEL0_R {
        RTCSEL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel1(&self) -> RTCSEL1_R {
        RTCSEL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BDCR")
            .field("bdrst", &self.bdrst())
            .field("rtcen", &self.rtcen())
            .field("rtcsel1", &self.rtcsel1())
            .field("rtcsel0", &self.rtcsel0())
            .field("lsebyp", &self.lsebyp())
            .field("lserdy", &self.lserdy())
            .field("lseon", &self.lseon())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - External low-speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LSEON_W<BDCRrs> {
        LSEON_W::new(self, 0)
    }
    #[doc = "Bit 2 - External low-speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LSEBYP_W<BDCRrs> {
        LSEBYP_W::new(self, 2)
    }
    #[doc = "Bit 8 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel0(&mut self) -> RTCSEL0_W<BDCRrs> {
        RTCSEL0_W::new(self, 8)
    }
    #[doc = "Bit 9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel1(&mut self) -> RTCSEL1_W<BDCRrs> {
        RTCSEL1_W::new(self, 9)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RTCEN_W<BDCRrs> {
        RTCEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&mut self) -> BDRST_W<BDCRrs> {
        BDRST_W::new(self, 16)
    }
}
#[doc = "Backup domain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDCRrs;
impl crate::RegisterSpec for BDCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BDCRrs {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BDCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCRrs {
    const RESET_VALUE: u32 = 0;
}
