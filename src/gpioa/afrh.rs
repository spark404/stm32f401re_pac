#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AFRHrs>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AFRHrs>;
#[doc = "Field `AFRH8` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_R = crate::FieldReader;
#[doc = "Field `AFRH8` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH9` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH9_R = crate::FieldReader;
#[doc = "Field `AFRH9` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH10` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH10_R = crate::FieldReader;
#[doc = "Field `AFRH10` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH11` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH11_R = crate::FieldReader;
#[doc = "Field `AFRH11` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH12` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH12_R = crate::FieldReader;
#[doc = "Field `AFRH12` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH13` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH13_R = crate::FieldReader;
#[doc = "Field `AFRH13` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH14` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH14_R = crate::FieldReader;
#[doc = "Field `AFRH14` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFRH15` reader - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH15_R = crate::FieldReader;
#[doc = "Field `AFRH15` writer - Alternate function selection for port x bit y (y = 8..15)"]
pub type AFRH15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh8(&self) -> AFRH8_R {
        AFRH8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh9(&self) -> AFRH9_R {
        AFRH9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh10(&self) -> AFRH10_R {
        AFRH10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh11(&self) -> AFRH11_R {
        AFRH11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh12(&self) -> AFRH12_R {
        AFRH12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh13(&self) -> AFRH13_R {
        AFRH13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh14(&self) -> AFRH14_R {
        AFRH14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh15(&self) -> AFRH15_R {
        AFRH15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("afrh15", &self.afrh15())
            .field("afrh14", &self.afrh14())
            .field("afrh13", &self.afrh13())
            .field("afrh12", &self.afrh12())
            .field("afrh11", &self.afrh11())
            .field("afrh10", &self.afrh10())
            .field("afrh9", &self.afrh9())
            .field("afrh8", &self.afrh8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh8(&mut self) -> AFRH8_W<AFRHrs> {
        AFRH8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh9(&mut self) -> AFRH9_W<AFRHrs> {
        AFRH9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh10(&mut self) -> AFRH10_W<AFRHrs> {
        AFRH10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh11(&mut self) -> AFRH11_W<AFRHrs> {
        AFRH11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh12(&mut self) -> AFRH12_W<AFRHrs> {
        AFRH12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh13(&mut self) -> AFRH13_W<AFRHrs> {
        AFRH13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh14(&mut self) -> AFRH14_W<AFRHrs> {
        AFRH14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)"]
    #[inline(always)]
    pub fn afrh15(&mut self) -> AFRH15_W<AFRHrs> {
        AFRH15_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRHrs;
impl crate::RegisterSpec for AFRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AFRHrs {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AFRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AFRHrs {
    const RESET_VALUE: u32 = 0;
}
