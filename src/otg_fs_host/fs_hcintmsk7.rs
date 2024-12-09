#[doc = "Register `FS_HCINTMSK7` reader"]
pub type R = crate::R<FS_HCINTMSK7rs>;
#[doc = "Register `FS_HCINTMSK7` writer"]
pub type W = crate::W<FS_HCINTMSK7rs>;
#[doc = "Field `XFRCM` reader - Transfer completed mask"]
pub type XFRCM_R = crate::BitReader;
#[doc = "Field `XFRCM` writer - Transfer completed mask"]
pub type XFRCM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - Channel halted mask"]
pub type CHHM_R = crate::BitReader;
#[doc = "Field `CHHM` writer - Channel halted mask"]
pub type CHHM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - STALL response received interrupt mask"]
pub type STALLM_R = crate::BitReader;
#[doc = "Field `STALLM` writer - STALL response received interrupt mask"]
pub type STALLM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - NAK response received interrupt mask"]
pub type NAKM_R = crate::BitReader;
#[doc = "Field `NAKM` writer - NAK response received interrupt mask"]
pub type NAKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - ACK response received/transmitted interrupt mask"]
pub type ACKM_R = crate::BitReader;
#[doc = "Field `ACKM` writer - ACK response received/transmitted interrupt mask"]
pub type ACKM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - response received interrupt mask"]
pub type NYET_R = crate::BitReader;
#[doc = "Field `NYET` writer - response received interrupt mask"]
pub type NYET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - Transaction error mask"]
pub type TXERRM_R = crate::BitReader;
#[doc = "Field `TXERRM` writer - Transaction error mask"]
pub type TXERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - Babble error mask"]
pub type BBERRM_R = crate::BitReader;
#[doc = "Field `BBERRM` writer - Babble error mask"]
pub type BBERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - Frame overrun mask"]
pub type FRMORM_R = crate::BitReader;
#[doc = "Field `FRMORM` writer - Frame overrun mask"]
pub type FRMORM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - Data toggle error mask"]
pub type DTERRM_R = crate::BitReader;
#[doc = "Field `DTERRM` writer - Data toggle error mask"]
pub type DTERRM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&self) -> CHHM_R {
        CHHM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&self) -> STALLM_R {
        STALLM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&self) -> NAKM_R {
        NAKM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&self) -> ACKM_R {
        ACKM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&self) -> NYET_R {
        NYET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&self) -> TXERRM_R {
        TXERRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&self) -> BBERRM_R {
        BBERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&self) -> FRMORM_R {
        FRMORM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&self) -> DTERRM_R {
        DTERRM_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HCINTMSK7")
            .field("xfrcm", &self.xfrcm())
            .field("chhm", &self.chhm())
            .field("stallm", &self.stallm())
            .field("nakm", &self.nakm())
            .field("ackm", &self.ackm())
            .field("nyet", &self.nyet())
            .field("txerrm", &self.txerrm())
            .field("bberrm", &self.bberrm())
            .field("frmorm", &self.frmorm())
            .field("dterrm", &self.dterrm())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XFRCM_W<FS_HCINTMSK7rs> {
        XFRCM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhm(&mut self) -> CHHM_W<FS_HCINTMSK7rs> {
        CHHM_W::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallm(&mut self) -> STALLM_W<FS_HCINTMSK7rs> {
        STALLM_W::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NAKM_W<FS_HCINTMSK7rs> {
        NAKM_W::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackm(&mut self) -> ACKM_W<FS_HCINTMSK7rs> {
        ACKM_W::new(self, 5)
    }
    #[doc = "Bit 6 - response received interrupt mask"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NYET_W<FS_HCINTMSK7rs> {
        NYET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TXERRM_W<FS_HCINTMSK7rs> {
        TXERRM_W::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BBERRM_W<FS_HCINTMSK7rs> {
        BBERRM_W::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FRMORM_W<FS_HCINTMSK7rs> {
        FRMORM_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DTERRM_W<FS_HCINTMSK7rs> {
        DTERRM_W::new(self, 10)
    }
}
#[doc = "OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hcintmsk7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcintmsk7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HCINTMSK7rs;
impl crate::RegisterSpec for FS_HCINTMSK7rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcintmsk7::R`](R) reader structure"]
impl crate::Readable for FS_HCINTMSK7rs {}
#[doc = "`write(|w| ..)` method takes [`fs_hcintmsk7::W`](W) writer structure"]
impl crate::Writable for FS_HCINTMSK7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_HCINTMSK7 to value 0"]
impl crate::Resettable for FS_HCINTMSK7rs {
    const RESET_VALUE: u32 = 0;
}
