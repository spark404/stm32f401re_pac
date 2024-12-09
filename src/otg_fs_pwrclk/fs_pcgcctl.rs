#[doc = "Register `FS_PCGCCTL` reader"]
pub type R = crate::R<FS_PCGCCTLrs>;
#[doc = "Register `FS_PCGCCTL` writer"]
pub type W = crate::W<FS_PCGCCTLrs>;
#[doc = "Field `STPPCLK` reader - Stop PHY clock"]
pub type STPPCLK_R = crate::BitReader;
#[doc = "Field `STPPCLK` writer - Stop PHY clock"]
pub type STPPCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GATEHCLK` reader - Gate HCLK"]
pub type GATEHCLK_R = crate::BitReader;
#[doc = "Field `GATEHCLK` writer - Gate HCLK"]
pub type GATEHCLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHYSUSP` reader - PHY Suspended"]
pub type PHYSUSP_R = crate::BitReader;
#[doc = "Field `PHYSUSP` writer - PHY Suspended"]
pub type PHYSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&self) -> STPPCLK_R {
        STPPCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&self) -> GATEHCLK_R {
        GATEHCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&self) -> PHYSUSP_R {
        PHYSUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_PCGCCTL")
            .field("stppclk", &self.stppclk())
            .field("gatehclk", &self.gatehclk())
            .field("physusp", &self.physusp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stop PHY clock"]
    #[inline(always)]
    pub fn stppclk(&mut self) -> STPPCLK_W<FS_PCGCCTLrs> {
        STPPCLK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Gate HCLK"]
    #[inline(always)]
    pub fn gatehclk(&mut self) -> GATEHCLK_W<FS_PCGCCTLrs> {
        GATEHCLK_W::new(self, 1)
    }
    #[doc = "Bit 4 - PHY Suspended"]
    #[inline(always)]
    pub fn physusp(&mut self) -> PHYSUSP_W<FS_PCGCCTLrs> {
        PHYSUSP_W::new(self, 4)
    }
}
#[doc = "OTG_FS power and clock gating control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_pcgcctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_pcgcctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_PCGCCTLrs;
impl crate::RegisterSpec for FS_PCGCCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_pcgcctl::R`](R) reader structure"]
impl crate::Readable for FS_PCGCCTLrs {}
#[doc = "`write(|w| ..)` method takes [`fs_pcgcctl::W`](W) writer structure"]
impl crate::Writable for FS_PCGCCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_PCGCCTL to value 0"]
impl crate::Resettable for FS_PCGCCTLrs {
    const RESET_VALUE: u32 = 0;
}
