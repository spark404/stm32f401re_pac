#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Field `IT4_RMP` reader - Timer Input 4 remap"]
pub type IT4_RMP_R = crate::FieldReader;
#[doc = "Field `IT4_RMP` writer - Timer Input 4 remap"]
pub type IT4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&self) -> IT4_RMP_R {
        IT4_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("it4_rmp", &self.it4_rmp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 6:7 - Timer Input 4 remap"]
    #[inline(always)]
    pub fn it4_rmp(&mut self) -> IT4_RMP_W<ORrs> {
        IT4_RMP_W::new(self, 6)
    }
}
#[doc = "TIM5 option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
