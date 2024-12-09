#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRLrs>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRLrs>;
#[doc = "Field `AFRL0` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL0_R = crate::FieldReader;
#[doc = "Field `AFRL0` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL1` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL1_R = crate::FieldReader;
#[doc = "Field `AFRL1` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL2` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL2_R = crate::FieldReader;
#[doc = "Field `AFRL2` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL3` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_R = crate::FieldReader;
#[doc = "Field `AFRL3` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL4` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL4_R = crate::FieldReader;
#[doc = "Field `AFRL4` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL5` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL5_R = crate::FieldReader;
#[doc = "Field `AFRL5` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL6` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL6_R = crate::FieldReader;
#[doc = "Field `AFRL6` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRL7` reader - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL7_R = crate::FieldReader;
#[doc = "Field `AFRL7` writer - Alternate function selection for port x bit y (y = 0..7)"]
pub type AFRL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&self) -> AFRL0_R {
        AFRL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&self) -> AFRL1_R {
        AFRL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&self) -> AFRL2_R {
        AFRL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&self) -> AFRL3_R {
        AFRL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&self) -> AFRL4_R {
        AFRL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&self) -> AFRL5_R {
        AFRL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&self) -> AFRL6_R {
        AFRL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&self) -> AFRL7_R {
        AFRL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRL")
            .field("afrl7", &self.afrl7())
            .field("afrl6", &self.afrl6())
            .field("afrl5", &self.afrl5())
            .field("afrl4", &self.afrl4())
            .field("afrl3", &self.afrl3())
            .field("afrl2", &self.afrl2())
            .field("afrl1", &self.afrl1())
            .field("afrl0", &self.afrl0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl0(&mut self) -> AFRL0_W<AFRLrs> {
        AFRL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl1(&mut self) -> AFRL1_W<AFRLrs> {
        AFRL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl2(&mut self) -> AFRL2_W<AFRLrs> {
        AFRL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl3(&mut self) -> AFRL3_W<AFRLrs> {
        AFRL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl4(&mut self) -> AFRL4_W<AFRLrs> {
        AFRL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl5(&mut self) -> AFRL5_W<AFRLrs> {
        AFRL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl6(&mut self) -> AFRL6_W<AFRLrs> {
        AFRL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)"]
    #[inline(always)]
    pub fn afrl7(&mut self) -> AFRL7_W<AFRLrs> {
        AFRL7_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRLrs {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRLrs {
    const RESET_VALUE: u32 = 0;
}
