#[doc = "Register `FS_GNPTXSTS` reader"]
pub type R = crate::R<FsGnptxstsSpec>;
#[doc = "Field `NPTXFSAV` reader - Non-periodic TxFIFO space available"]
pub type NptxfsavR = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Non-periodic transmit request queue space available"]
pub type NptqxsavR = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the non-periodic transmit request queue"]
pub type NptxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Non-periodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NptxfsavR {
        NptxfsavR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Non-periodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NptqxsavR {
        NptqxsavR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the non-periodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NptxqtopR {
        NptxqtopR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsGnptxstsSpec;
impl crate::RegisterSpec for FsGnptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gnptxsts::R`](R) reader structure"]
impl crate::Readable for FsGnptxstsSpec {}
#[doc = "`reset()` method sets FS_GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for FsGnptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0200;
}
