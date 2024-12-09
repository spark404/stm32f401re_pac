#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "Field `AWD` reader - Analog watchdog flag"]
pub type AWD_R = crate::BitReader;
#[doc = "Field `AWD` writer - Analog watchdog flag"]
pub type AWD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - Regular channel end of conversion"]
pub type EOC_R = crate::BitReader;
#[doc = "Field `EOC` writer - Regular channel end of conversion"]
pub type EOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - Injected channel end of conversion"]
pub type JEOC_R = crate::BitReader;
#[doc = "Field `JEOC` writer - Injected channel end of conversion"]
pub type JEOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSTRT` reader - Injected channel start flag"]
pub type JSTRT_R = crate::BitReader;
#[doc = "Field `JSTRT` writer - Injected channel start flag"]
pub type JSTRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRT` reader - Regular channel start flag"]
pub type STRT_R = crate::BitReader;
#[doc = "Field `STRT` writer - Regular channel start flag"]
pub type STRT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - Overrun"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - Overrun"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("ovr", &self.ovr())
            .field("strt", &self.strt())
            .field("jstrt", &self.jstrt())
            .field("jeoc", &self.jeoc())
            .field("eoc", &self.eoc())
            .field("awd", &self.awd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Analog watchdog flag"]
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<SRrs> {
        AWD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Regular channel end of conversion"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<SRrs> {
        EOC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Injected channel end of conversion"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<SRrs> {
        JEOC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Injected channel start flag"]
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W<SRrs> {
        JSTRT_W::new(self, 3)
    }
    #[doc = "Bit 4 - Regular channel start flag"]
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<SRrs> {
        STRT_W::new(self, 4)
    }
    #[doc = "Bit 5 - Overrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<SRrs> {
        OVR_W::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
