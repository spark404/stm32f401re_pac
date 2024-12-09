#[doc = "Register `FS_DIEPMSK` reader"]
pub type R = crate::R<FS_DIEPMSKrs>;
#[doc = "Register `FS_DIEPMSK` writer"]
pub type W = crate::W<FS_DIEPMSKrs>;
#[doc = "Field `XFRCM` reader - Transfer completed interrupt mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed interrupt mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDM` reader - Endpoint disabled interrupt mask"]
pub type EPDM_R = crate::BitReader;
#[doc = "Field `EPDM` writer - Endpoint disabled interrupt mask"]
pub type EPDM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOM` reader - Timeout condition mask (Non-isochronous endpoints)"]
pub type TOM_R = crate::BitReader;
#[doc = "Field `TOM` writer - Timeout condition mask (Non-isochronous endpoints)"]
pub type TOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFEMSK` reader - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_R = crate::BitReader;
#[doc = "Field `ITTXFEMSK` writer - IN token received when TxFIFO empty mask"]
pub type ITTXFEMSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNMM` reader - IN token received with EP mismatch mask"]
pub type INEPNMM_R = crate::BitReader;
#[doc = "Field `INEPNMM` writer - IN token received with EP mismatch mask"]
pub type INEPNMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNEM` reader - IN endpoint NAK effective mask"]
pub type INEPNEM_R = crate::BitReader;
#[doc = "Field `INEPNEM` writer - IN endpoint NAK effective mask"]
pub type INEPNEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&self) -> TOM_R {
        TOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&self) -> ITTXFEMSK_R {
        ITTXFEMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&self) -> INEPNMM_R {
        INEPNMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&self) -> INEPNEM_R {
        INEPNEM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DIEPMSK")
            .field("xfrcm", &self.xfrcm())
            .field("epdm", &self.epdm())
            .field("tom", &self.tom())
            .field("ittxfemsk", &self.ittxfemsk())
            .field("inepnmm", &self.inepnmm())
            .field("inepnem", &self.inepnem())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<FS_DIEPMSKrs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt mask"]
    #[inline(always)]
    pub fn epdm(&mut self) -> EPDM_W<FS_DIEPMSKrs> {
        EPDM_W::new(self, 1)
    }
    #[doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)"]
    #[inline(always)]
    pub fn tom(&mut self) -> TOM_W<FS_DIEPMSKrs> {
        TOM_W::new(self, 3)
    }
    #[doc = "Bit 4 - IN token received when TxFIFO empty mask"]
    #[inline(always)]
    pub fn ittxfemsk(&mut self) -> ITTXFEMSK_W<FS_DIEPMSKrs> {
        ITTXFEMSK_W::new(self, 4)
    }
    #[doc = "Bit 5 - IN token received with EP mismatch mask"]
    #[inline(always)]
    pub fn inepnmm(&mut self) -> INEPNMM_W<FS_DIEPMSKrs> {
        INEPNMM_W::new(self, 5)
    }
    #[doc = "Bit 6 - IN endpoint NAK effective mask"]
    #[inline(always)]
    pub fn inepnem(&mut self) -> INEPNEM_W<FS_DIEPMSKrs> {
        INEPNEM_W::new(self, 6)
    }
}
#[doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_diepmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_diepmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_DIEPMSKrs;
impl crate::RegisterSpec for FS_DIEPMSKrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_diepmsk::R`](R) reader structure"]
impl crate::Readable for FS_DIEPMSKrs {}
#[doc = "`write(|w| ..)` method takes [`fs_diepmsk::W`](W) writer structure"]
impl crate::Writable for FS_DIEPMSKrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_DIEPMSK to value 0"]
impl crate::Resettable for FS_DIEPMSKrs {
    const RESET_VALUE: u32 = 0;
}
