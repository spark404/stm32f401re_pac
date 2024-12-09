#[doc = "Register `SSCGR` reader"]
pub type R = crate::R<SSCGRrs>;
#[doc = "Register `SSCGR` writer"]
pub type W = crate::W<SSCGRrs>;
#[doc = "Field `MODPER` reader - Modulation period"]
pub type MODPER_R = crate::FieldReader<u16>;
#[doc = "Field `MODPER` writer - Modulation period"]
pub type MODPER_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `INCSTEP` reader - Incrementation step"]
pub type INCSTEP_R = crate::FieldReader<u16>;
#[doc = "Field `INCSTEP` writer - Incrementation step"]
pub type INCSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SPREADSEL` reader - Spread Select"]
pub type SPREADSEL_R = crate::BitReader;
#[doc = "Field `SPREADSEL` writer - Spread Select"]
pub type SPREADSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSCGEN` reader - Spread spectrum modulation enable"]
pub type SSCGEN_R = crate::BitReader;
#[doc = "Field `SSCGEN` writer - Spread spectrum modulation enable"]
pub type SSCGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&self) -> MODPER_R {
        MODPER_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&self) -> INCSTEP_R {
        INCSTEP_R::new(((self.bits >> 13) & 0x7fff) as u16)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&self) -> SPREADSEL_R {
        SPREADSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&self) -> SSCGEN_R {
        SSCGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SSCGR")
            .field("sscgen", &self.sscgen())
            .field("spreadsel", &self.spreadsel())
            .field("incstep", &self.incstep())
            .field("modper", &self.modper())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12 - Modulation period"]
    #[inline(always)]
    pub fn modper(&mut self) -> MODPER_W<SSCGRrs> {
        MODPER_W::new(self, 0)
    }
    #[doc = "Bits 13:27 - Incrementation step"]
    #[inline(always)]
    pub fn incstep(&mut self) -> INCSTEP_W<SSCGRrs> {
        INCSTEP_W::new(self, 13)
    }
    #[doc = "Bit 30 - Spread Select"]
    #[inline(always)]
    pub fn spreadsel(&mut self) -> SPREADSEL_W<SSCGRrs> {
        SPREADSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Spread spectrum modulation enable"]
    #[inline(always)]
    pub fn sscgen(&mut self) -> SSCGEN_W<SSCGRrs> {
        SSCGEN_W::new(self, 31)
    }
}
#[doc = "spread spectrum clock generation register\n\nYou can [`read`](crate::Reg::read) this register and get [`sscgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSCGRrs;
impl crate::RegisterSpec for SSCGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sscgr::R`](R) reader structure"]
impl crate::Readable for SSCGRrs {}
#[doc = "`write(|w| ..)` method takes [`sscgr::W`](W) writer structure"]
impl crate::Writable for SSCGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSCGR to value 0"]
impl crate::Resettable for SSCGRrs {
    const RESET_VALUE: u32 = 0;
}
