#[doc = "Register `DBGMCU_IDCODE` reader"]
pub type R = crate::R<DBGMCU_IDCODErs>;
#[doc = "Field `DEV_ID` reader - DEV_ID"]
pub type DEV_ID_R = crate::FieldReader<u16>;
#[doc = "Field `REV_ID` reader - REV_ID"]
pub type REV_ID_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - DEV_ID"]
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:31 - REV_ID"]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_IDCODE")
            .field("dev_id", &self.dev_id())
            .field("rev_id", &self.rev_id())
            .finish()
    }
}
#[doc = "IDCODE\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_idcode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_IDCODErs;
impl crate::RegisterSpec for DBGMCU_IDCODErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_idcode::R`](R) reader structure"]
impl crate::Readable for DBGMCU_IDCODErs {}
#[doc = "`reset()` method sets DBGMCU_IDCODE to value 0x1000_6411"]
impl crate::Resettable for DBGMCU_IDCODErs {
    const RESET_VALUE: u32 = 0x1000_6411;
}
