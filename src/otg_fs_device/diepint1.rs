#[doc = "Register `DIEPINT1` reader"]
pub type R = crate::R<DIEPINT1rs>;
#[doc = "Register `DIEPINT1` writer"]
pub type W = crate::W<DIEPINT1rs>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XFRC_R = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XFRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EPDISD_R = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EPDISD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - TOC"]
pub type TOC_R = crate::BitReader;
#[doc = "Field `TOC` writer - TOC"]
pub type TOC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type ITTXFE_R = crate::BitReader;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type ITTXFE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type INEPNE_R = crate::BitReader;
#[doc = "Field `INEPNE` writer - INEPNE"]
pub type INEPNE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TXFE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XFRC_R {
        XFRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EPDISD_R {
        EPDISD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> ITTXFE_R {
        ITTXFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> INEPNE_R {
        INEPNE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TXFE_R {
        TXFE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPINT1")
            .field("txfe", &self.txfe())
            .field("inepne", &self.inepne())
            .field("ittxfe", &self.ittxfe())
            .field("toc", &self.toc())
            .field("epdisd", &self.epdisd())
            .field("xfrc", &self.xfrc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XFRC_W<DIEPINT1rs> {
        XFRC_W::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EPDISD_W<DIEPINT1rs> {
        EPDISD_W::new(self, 1)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<DIEPINT1rs> {
        TOC_W::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&mut self) -> ITTXFE_W<DIEPINT1rs> {
        ITTXFE_W::new(self, 4)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&mut self) -> INEPNE_W<DIEPINT1rs> {
        INEPNE_W::new(self, 6)
    }
}
#[doc = "device endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPINT1rs;
impl crate::RegisterSpec for DIEPINT1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint1::R`](R) reader structure"]
impl crate::Readable for DIEPINT1rs {}
#[doc = "`write(|w| ..)` method takes [`diepint1::W`](W) writer structure"]
impl crate::Writable for DIEPINT1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT1 to value 0x80"]
impl crate::Resettable for DIEPINT1rs {
    const RESET_VALUE: u32 = 0x80;
}
