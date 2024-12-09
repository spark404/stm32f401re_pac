#[doc = "Register `DTIMER` reader"]
pub type R = crate::R<DTIMERrs>;
#[doc = "Register `DTIMER` writer"]
pub type W = crate::W<DTIMERrs>;
#[doc = "Field `DATATIME` reader - Data timeout period"]
pub type DATATIME_R = crate::FieldReader<u32>;
#[doc = "Field `DATATIME` writer - Data timeout period"]
pub type DATATIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datatime(&self) -> DATATIME_R {
        DATATIME_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTIMER")
            .field("datatime", &self.datatime())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datatime(&mut self) -> DATATIME_W<DTIMERrs> {
        DATATIME_W::new(self, 0)
    }
}
#[doc = "data timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTIMERrs;
impl crate::RegisterSpec for DTIMERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtimer::R`](R) reader structure"]
impl crate::Readable for DTIMERrs {}
#[doc = "`write(|w| ..)` method takes [`dtimer::W`](W) writer structure"]
impl crate::Writable for DTIMERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTIMER to value 0"]
impl crate::Resettable for DTIMERrs {
    const RESET_VALUE: u32 = 0;
}
