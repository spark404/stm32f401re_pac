#[doc = "Register `DIEPTSIZ3` reader"]
pub type R = crate::R<DIEPTSIZ3rs>;
#[doc = "Register `DIEPTSIZ3` writer"]
pub type W = crate::W<DIEPTSIZ3rs>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MCNT` reader - Multi count"]
pub type MCNT_R = crate::FieldReader;
#[doc = "Field `MCNT` writer - Multi count"]
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ3")
            .field("mcnt", &self.mcnt())
            .field("pktcnt", &self.pktcnt())
            .field("xfrsiz", &self.xfrsiz())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ3rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ3rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W<DIEPTSIZ3rs> {
        MCNT_W::new(self, 29)
    }
}
#[doc = "device endpoint-3 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ3rs;
impl crate::RegisterSpec for DIEPTSIZ3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz3::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ3rs {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz3::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ3 to value 0"]
impl crate::Resettable for DIEPTSIZ3rs {
    const RESET_VALUE: u32 = 0;
}
