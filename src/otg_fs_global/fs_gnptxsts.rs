#[doc = "Register `FS_GNPTXSTS` reader"]
pub type R = crate::R<FS_GNPTXSTSrs>;
#[doc = "Field `NPTXFSAV` reader - Non-periodic TxFIFO space available"]
pub type NPTXFSAV_R = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Non-periodic transmit request queue space available"]
pub type NPTQXSAV_R = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the non-periodic transmit request queue"]
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GNPTXSTS")
            .field("nptxfsav", &self.nptxfsav())
            .field("nptqxsav", &self.nptqxsav())
            .field("nptxqtop", &self.nptxqtop())
            .finish()
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GNPTXSTSrs;
impl crate::RegisterSpec for FS_GNPTXSTSrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gnptxsts::R`](R) reader structure"]
impl crate::Readable for FS_GNPTXSTSrs {}
#[doc = "`reset()` method sets FS_GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for FS_GNPTXSTSrs {
    const RESET_VALUE: u32 = 0x0008_0200;
}
