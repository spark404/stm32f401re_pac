#[doc = "Register `FS_GUSBCFG` reader"]
pub type R = crate::R<FS_GUSBCFGrs>;
#[doc = "Register `FS_GUSBCFG` writer"]
pub type W = crate::W<FS_GUSBCFGrs>;
#[doc = "Field `TOCAL` reader - FS timeout calibration"]
pub type TOCAL_R = crate::FieldReader;
#[doc = "Field `TOCAL` writer - FS timeout calibration"]
pub type TOCAL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PHYSEL` writer - Full Speed serial transceiver select"]
pub type PHYSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRPCAP` reader - SRP-capable"]
pub type SRPCAP_R = crate::BitReader;
#[doc = "Field `SRPCAP` writer - SRP-capable"]
pub type SRPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNPCAP` reader - HNP-capable"]
pub type HNPCAP_R = crate::BitReader;
#[doc = "Field `HNPCAP` writer - HNP-capable"]
pub type HNPCAP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRDT` reader - USB turnaround time"]
pub type TRDT_R = crate::FieldReader;
#[doc = "Field `TRDT` writer - USB turnaround time"]
pub type TRDT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FHMOD` reader - Force host mode"]
pub type FHMOD_R = crate::BitReader;
#[doc = "Field `FHMOD` writer - Force host mode"]
pub type FHMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDMOD` reader - Force device mode"]
pub type FDMOD_R = crate::BitReader;
#[doc = "Field `FDMOD` writer - Force device mode"]
pub type FDMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTXPKT` reader - Corrupt Tx packet"]
pub type CTXPKT_R = crate::BitReader;
#[doc = "Field `CTXPKT` writer - Corrupt Tx packet"]
pub type CTXPKT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&self) -> TOCAL_R {
        TOCAL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&self) -> SRPCAP_R {
        SRPCAP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&self) -> HNPCAP_R {
        HNPCAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&self) -> TRDT_R {
        TRDT_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhmod(&self) -> FHMOD_R {
        FHMOD_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdmod(&self) -> FDMOD_R {
        FDMOD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&self) -> CTXPKT_R {
        CTXPKT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GUSBCFG")
            .field("tocal", &self.tocal())
            .field("srpcap", &self.srpcap())
            .field("hnpcap", &self.hnpcap())
            .field("trdt", &self.trdt())
            .field("fhmod", &self.fhmod())
            .field("fdmod", &self.fdmod())
            .field("ctxpkt", &self.ctxpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn tocal(&mut self) -> TOCAL_W<FS_GUSBCFGrs> {
        TOCAL_W::new(self, 0)
    }
    #[doc = "Bit 6 - Full Speed serial transceiver select"]
    #[inline(always)]
    pub fn physel(&mut self) -> PHYSEL_W<FS_GUSBCFGrs> {
        PHYSEL_W::new(self, 6)
    }
    #[doc = "Bit 8 - SRP-capable"]
    #[inline(always)]
    pub fn srpcap(&mut self) -> SRPCAP_W<FS_GUSBCFGrs> {
        SRPCAP_W::new(self, 8)
    }
    #[doc = "Bit 9 - HNP-capable"]
    #[inline(always)]
    pub fn hnpcap(&mut self) -> HNPCAP_W<FS_GUSBCFGrs> {
        HNPCAP_W::new(self, 9)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn trdt(&mut self) -> TRDT_W<FS_GUSBCFGrs> {
        TRDT_W::new(self, 10)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhmod(&mut self) -> FHMOD_W<FS_GUSBCFGrs> {
        FHMOD_W::new(self, 29)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdmod(&mut self) -> FDMOD_W<FS_GUSBCFGrs> {
        FDMOD_W::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn ctxpkt(&mut self) -> CTXPKT_W<FS_GUSBCFGrs> {
        CTXPKT_W::new(self, 31)
    }
}
#[doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gusbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gusbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GUSBCFGrs;
impl crate::RegisterSpec for FS_GUSBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gusbcfg::R`](R) reader structure"]
impl crate::Readable for FS_GUSBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`fs_gusbcfg::W`](W) writer structure"]
impl crate::Writable for FS_GUSBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GUSBCFG to value 0x0a00"]
impl crate::Resettable for FS_GUSBCFGrs {
    const RESET_VALUE: u32 = 0x0a00;
}
