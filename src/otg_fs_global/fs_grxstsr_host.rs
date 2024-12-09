#[doc = "Register `FS_GRXSTSR_Host` reader"]
pub type R = crate::R<FS_GRXSTSR_HOSTrs>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EPNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PKTSTS_R = crate::FieldReader;
#[doc = "Field `FRMNUM` reader - Frame number"]
pub type FRMNUM_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EPNUM_R {
        EPNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame number"]
    #[inline(always)]
    pub fn frmnum(&self) -> FRMNUM_R {
        FRMNUM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GRXSTSR_Host")
            .field("epnum", &self.epnum())
            .field("bcnt", &self.bcnt())
            .field("dpid", &self.dpid())
            .field("pktsts", &self.pktsts())
            .field("frmnum", &self.frmnum())
            .finish()
    }
}
#[doc = "OTG_FS Receive status debug read(Host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_grxstsr_host::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GRXSTSR_HOSTrs;
impl crate::RegisterSpec for FS_GRXSTSR_HOSTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_grxstsr_host::R`](R) reader structure"]
impl crate::Readable for FS_GRXSTSR_HOSTrs {}
#[doc = "`reset()` method sets FS_GRXSTSR_Host to value 0"]
impl crate::Resettable for FS_GRXSTSR_HOSTrs {
    const RESET_VALUE: u32 = 0;
}
