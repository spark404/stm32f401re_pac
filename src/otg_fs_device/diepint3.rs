#[doc = "Register `DIEPINT3` reader"]
pub type R = crate::R<Diepint3Spec>;
#[doc = "Register `DIEPINT3` writer"]
pub type W = crate::W<Diepint3Spec>;
#[doc = "Field `XFRC` reader - XFRC"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - XFRC"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - EPDISD"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - EPDISD"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOC` reader - TOC"]
pub type TocR = crate::BitReader;
#[doc = "Field `TOC` writer - TOC"]
pub type TocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITTXFE` reader - ITTXFE"]
pub type IttxfeR = crate::BitReader;
#[doc = "Field `ITTXFE` writer - ITTXFE"]
pub type IttxfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INEPNE` reader - INEPNE"]
pub type InepneR = crate::BitReader;
#[doc = "Field `INEPNE` writer - INEPNE"]
pub type InepneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFE` reader - TXFE"]
pub type TxfeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&self) -> TocR {
        TocR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&self) -> IttxfeR {
        IttxfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&self) -> InepneR {
        InepneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXFE"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<Diepint3Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - EPDISD"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EpdisdW<Diepint3Spec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - TOC"]
    #[inline(always)]
    pub fn toc(&mut self) -> TocW<Diepint3Spec> {
        TocW::new(self, 3)
    }
    #[doc = "Bit 4 - ITTXFE"]
    #[inline(always)]
    pub fn ittxfe(&mut self) -> IttxfeW<Diepint3Spec> {
        IttxfeW::new(self, 4)
    }
    #[doc = "Bit 6 - INEPNE"]
    #[inline(always)]
    pub fn inepne(&mut self) -> InepneW<Diepint3Spec> {
        InepneW::new(self, 6)
    }
}
#[doc = "device endpoint-3 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepint3Spec;
impl crate::RegisterSpec for Diepint3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepint3::R`](R) reader structure"]
impl crate::Readable for Diepint3Spec {}
#[doc = "`write(|w| ..)` method takes [`diepint3::W`](W) writer structure"]
impl crate::Writable for Diepint3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPINT3 to value 0x80"]
impl crate::Resettable for Diepint3Spec {
    const RESET_VALUE: u32 = 0x80;
}