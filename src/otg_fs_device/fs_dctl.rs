#[doc = "Register `FS_DCTL` reader"]
pub type R = crate::R<FS_DCTLrs>;
#[doc = "Register `FS_DCTL` writer"]
pub type W = crate::W<FS_DCTLrs>;
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RWUSIG_R = crate::BitReader;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RWUSIG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SDIS_R = crate::BitReader;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GINSTS_R = crate::BitReader;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GONSTS_R = crate::BitReader;
#[doc = "Field `TCTL` reader - Test control"]
pub type TCTL_R = crate::FieldReader;
#[doc = "Field `TCTL` writer - Test control"]
pub type TCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` reader - Set global IN NAK"]
pub type SGINAK_R = crate::BitReader;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` reader - Clear global IN NAK"]
pub type CGINAK_R = crate::BitReader;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CGINAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` reader - Set global OUT NAK"]
pub type SGONAK_R = crate::BitReader;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` reader - Clear global OUT NAK"]
pub type CGONAK_R = crate::BitReader;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CGONAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type POPRGDNE_R = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type POPRGDNE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RWUSIG_R {
        RWUSIG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GINSTS_R {
        GINSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GONSTS_R {
        GONSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TCTL_R {
        TCTL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&self) -> SGINAK_R {
        SGINAK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&self) -> CGINAK_R {
        CGINAK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&self) -> SGONAK_R {
        SGONAK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&self) -> CGONAK_R {
        CGONAK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> POPRGDNE_R {
        POPRGDNE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DCTL")
            .field("rwusig", &self.rwusig())
            .field("sdis", &self.sdis())
            .field("ginsts", &self.ginsts())
            .field("gonsts", &self.gonsts())
            .field("tctl", &self.tctl())
            .field("sginak", &self.sginak())
            .field("cginak", &self.cginak())
            .field("sgonak", &self.sgonak())
            .field("cgonak", &self.cgonak())
            .field("poprgdne", &self.poprgdne())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RWUSIG_W<FS_DCTLrs> {
        RWUSIG_W::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SDIS_W<FS_DCTLrs> {
        SDIS_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TCTL_W<FS_DCTLrs> {
        TCTL_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SGINAK_W<FS_DCTLrs> {
        SGINAK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CGINAK_W<FS_DCTLrs> {
        CGINAK_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SGONAK_W<FS_DCTLrs> {
        SGONAK_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CGONAK_W<FS_DCTLrs> {
        CGONAK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> POPRGDNE_W<FS_DCTLrs> {
        POPRGDNE_W::new(self, 11)
    }
}
#[doc = "OTG_FS device control register (OTG_FS_DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_DCTLrs;
impl crate::RegisterSpec for FS_DCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dctl::R`](R) reader structure"]
impl crate::Readable for FS_DCTLrs {}
#[doc = "`write(|w| ..)` method takes [`fs_dctl::W`](W) writer structure"]
impl crate::Writable for FS_DCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_DCTL to value 0"]
impl crate::Resettable for FS_DCTLrs {
    const RESET_VALUE: u32 = 0;
}
