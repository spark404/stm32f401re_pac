#[doc = "Register `OAR1` reader"]
pub type R = crate::R<OAR1rs>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<OAR1rs>;
#[doc = "Field `ADD0` reader - Interface address"]
pub type ADD0_R = crate::BitReader;
#[doc = "Field `ADD0` writer - Interface address"]
pub type ADD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD7` reader - Interface address"]
pub type ADD7_R = crate::FieldReader;
#[doc = "Field `ADD7` writer - Interface address"]
pub type ADD7_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADD10` reader - Interface address"]
pub type ADD10_R = crate::FieldReader;
#[doc = "Field `ADD10` writer - Interface address"]
pub type ADD10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADDMODE` reader - Addressing mode (slave mode)"]
pub type ADDMODE_R = crate::BitReader;
#[doc = "Field `ADDMODE` writer - Addressing mode (slave mode)"]
pub type ADDMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&self) -> ADD0_R {
        ADD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&self) -> ADD7_R {
        ADD7_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&self) -> ADDMODE_R {
        ADDMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OAR1")
            .field("addmode", &self.addmode())
            .field("add10", &self.add10())
            .field("add7", &self.add7())
            .field("add0", &self.add0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn add0(&mut self) -> ADD0_W<OAR1rs> {
        ADD0_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn add7(&mut self) -> ADD7_W<OAR1rs> {
        ADD7_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn add10(&mut self) -> ADD10_W<OAR1rs> {
        ADD10_W::new(self, 8)
    }
    #[doc = "Bit 15 - Addressing mode (slave mode)"]
    #[inline(always)]
    pub fn addmode(&mut self) -> ADDMODE_W<OAR1rs> {
        ADDMODE_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR1rs;
impl crate::RegisterSpec for OAR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for OAR1rs {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for OAR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1rs {
    const RESET_VALUE: u32 = 0;
}
