#[doc = "Register `OAR2` reader"]
pub type R = crate::R<OAR2rs>;
#[doc = "Register `OAR2` writer"]
pub type W = crate::W<OAR2rs>;
#[doc = "Field `ENDUAL` reader - Dual addressing mode enable"]
pub type ENDUAL_R = crate::BitReader;
#[doc = "Field `ENDUAL` writer - Dual addressing mode enable"]
pub type ENDUAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD2` reader - Interface address"]
pub type ADD2_R = crate::FieldReader;
#[doc = "Field `ADD2` writer - Interface address"]
pub type ADD2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&self) -> ENDUAL_R {
        ENDUAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&self) -> ADD2_R {
        ADD2_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR2")
            .field("add2", &self.add2())
            .field("endual", &self.endual())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Dual addressing mode enable"]
    #[inline(always)]
    pub fn endual(&mut self) -> ENDUAL_W<OAR2rs> {
        ENDUAL_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add2(&mut self) -> ADD2_W<OAR2rs> {
        ADD2_W::new(self, 1)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`oar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR2rs;
impl crate::RegisterSpec for OAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar2::R`](R) reader structure"]
impl crate::Readable for OAR2rs {}
#[doc = "`write(|w| ..)` method takes [`oar2::W`](W) writer structure"]
impl crate::Writable for OAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR2 to value 0"]
impl crate::Resettable for OAR2rs {
    const RESET_VALUE: u32 = 0;
}
