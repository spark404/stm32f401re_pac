#[doc = "Register `S7NDTR` reader"]
pub type R = crate::R<S7NDTRrs>;
#[doc = "Register `S7NDTR` writer"]
pub type W = crate::W<S7NDTRrs>;
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NDT_R = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NDT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S7NDTR").field("ndt", &self.ndt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W<S7NDTRrs> {
        NDT_W::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s7ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s7ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7NDTRrs;
impl crate::RegisterSpec for S7NDTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7ndtr::R`](R) reader structure"]
impl crate::Readable for S7NDTRrs {}
#[doc = "`write(|w| ..)` method takes [`s7ndtr::W`](W) writer structure"]
impl crate::Writable for S7NDTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7NDTR to value 0"]
impl crate::Resettable for S7NDTRrs {
    const RESET_VALUE: u32 = 0;
}
