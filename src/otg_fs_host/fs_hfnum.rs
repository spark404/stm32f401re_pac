#[doc = "Register `FS_HFNUM` reader"]
pub type R = crate::R<FS_HFNUMrs>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FRNUM_R = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FTREM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FTREM_R {
        FTREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HFNUM")
            .field("frnum", &self.frnum())
            .field("ftrem", &self.ftrem())
            .finish()
    }
}
#[doc = "OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HFNUMrs;
impl crate::RegisterSpec for FS_HFNUMrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hfnum::R`](R) reader structure"]
impl crate::Readable for FS_HFNUMrs {}
#[doc = "`reset()` method sets FS_HFNUM to value 0x3fff"]
impl crate::Resettable for FS_HFNUMrs {
    const RESET_VALUE: u32 = 0x3fff;
}
