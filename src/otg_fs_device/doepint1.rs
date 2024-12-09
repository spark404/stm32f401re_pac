#[doc = "Register `DOEPINT1` reader"]
pub type R = crate::R<DOEPINT1rs>;
#[doc = "Register `DOEPINT1` writer"]
pub type W = crate::W<DOEPINT1rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - STUP"]
pub type STUP_R = crate::BitReader;
#[doc = "Field `STUP` writer - STUP"]
pub type STUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OTEPDIS"]
pub type OTEPDIS_R = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OTEPDIS"]
pub type OTEPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - B2BSTUP"]
pub type B2BSTUP_R = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - B2BSTUP"]
pub type B2BSTUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&self) -> STUP_R {
        STUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&self) -> OTEPDIS_R {
        OTEPDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2BSTUP_R {
        B2BSTUP_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPINT1")
            .field("b2bstup", &self.b2bstup())
            .field("otepdis", &self.otepdis())
            .field("stup", &self.stup())
            .field("epdisd", &self.epdisd())
            .field("xfrc", &self.xfrc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DOEPINT1rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DOEPINT1rs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - STUP"]
    #[inline(always)]
    pub fn stup(&mut self) -> STUP_W<DOEPINT1rs> {
        STUP_W::new(self, 3)
    }
    #[doc = "Bit 4 - OTEPDIS"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OTEPDIS_W<DOEPINT1rs> {
        OTEPDIS_W::new(self, 4)
    }
    #[doc = "Bit 6 - B2BSTUP"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2BSTUP_W<DOEPINT1rs> {
        B2BSTUP_W::new(self, 6)
    }
}
#[doc = "device endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPINT1rs;
impl crate::RegisterSpec for DOEPINT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepint1::R`](R) reader structure"]
impl crate::Readable for DOEPINT1rs {}
#[doc = "`write(|w| ..)` method takes [`doepint1::W`](W) writer structure"]
impl crate::Writable for DOEPINT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPINT1 to value 0x80"]
impl crate::Resettable for DOEPINT1rs {
    const RESET_VALUE: u32 = 0x80;
}
