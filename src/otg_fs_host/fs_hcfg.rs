#[doc = "Register `FS_HCFG` reader"]
pub type R = crate::R<FS_HCFGrs>;
#[doc = "Register `FS_HCFG` writer"]
pub type W = crate::W<FS_HCFGrs>;
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FSLSPCS_R = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FSLSPCS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FSLSS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HCFG")
            .field("fslspcs", &self.fslspcs())
            .field("fslss", &self.fslss())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W<FS_HCFGrs> {
        FSLSPCS_W::new(self, 0)
    }
}
#[doc = "OTG_FS host configuration register (OTG_FS_HCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HCFGrs;
impl crate::RegisterSpec for FS_HCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcfg::R`](R) reader structure"]
impl crate::Readable for FS_HCFGrs {}
#[doc = "`write(|w| ..)` method takes [`fs_hcfg::W`](W) writer structure"]
impl crate::Writable for FS_HCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_HCFG to value 0"]
impl crate::Resettable for FS_HCFGrs {
    const RESET_VALUE: u32 = 0;
}
