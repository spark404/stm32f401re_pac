#[doc = "Register `FS_GAHBCFG` reader"]
pub type R = crate::R<FS_GAHBCFGrs>;
#[doc = "Register `FS_GAHBCFG` writer"]
pub type W = crate::W<FS_GAHBCFGrs>;
#[doc = "Field `GINT` reader - Global interrupt mask"]
pub type GINT_R = crate::BitReader;
#[doc = "Field `GINT` writer - Global interrupt mask"]
pub type GINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFELVL` reader - TxFIFO empty level"]
pub type TXFELVL_R = crate::BitReader;
#[doc = "Field `TXFELVL` writer - TxFIFO empty level"]
pub type TXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFELVL` reader - Periodic TxFIFO empty level"]
pub type PTXFELVL_R = crate::BitReader;
#[doc = "Field `PTXFELVL` writer - Periodic TxFIFO empty level"]
pub type PTXFELVL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GAHBCFG")
            .field("gint", &self.gint())
            .field("txfelvl", &self.txfelvl())
            .field("ptxfelvl", &self.ptxfelvl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Global interrupt mask"]
    #[inline(always)]
    pub fn gint(&mut self) -> GINT_W<FS_GAHBCFGrs> {
        GINT_W::new(self, 0)
    }
    #[doc = "Bit 7 - TxFIFO empty level"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W<FS_GAHBCFGrs> {
        TXFELVL_W::new(self, 7)
    }
    #[doc = "Bit 8 - Periodic TxFIFO empty level"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W<FS_GAHBCFGrs> {
        PTXFELVL_W::new(self, 8)
    }
}
#[doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gahbcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gahbcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GAHBCFGrs;
impl crate::RegisterSpec for FS_GAHBCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gahbcfg::R`](R) reader structure"]
impl crate::Readable for FS_GAHBCFGrs {}
#[doc = "`write(|w| ..)` method takes [`fs_gahbcfg::W`](W) writer structure"]
impl crate::Writable for FS_GAHBCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GAHBCFG to value 0"]
impl crate::Resettable for FS_GAHBCFGrs {
    const RESET_VALUE: u32 = 0;
}
