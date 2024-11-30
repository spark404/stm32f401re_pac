#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IrenR = crate::BitReader;
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IrlpR = crate::BitReader;
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IrlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type ScenR = crate::BitReader;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DmarR = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DmarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DmatR = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RtseR = crate::BitReader;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CtsieR = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CtsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type OnebitR = crate::BitReader;
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IrenR {
        IrenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IrlpR {
        IrlpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DmarR {
        DmarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DmatR {
        DmatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RtseR {
        RtseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CtsieR {
        CtsieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<Cr3Spec> {
        EieW::new(self, 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&mut self) -> IrenW<Cr3Spec> {
        IrenW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&mut self) -> IrlpW<Cr3Spec> {
        IrlpW::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<Cr3Spec> {
        NackW::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&mut self) -> ScenW<Cr3Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DmarW<Cr3Spec> {
        DmarW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DmatW<Cr3Spec> {
        DmatW::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RtseW<Cr3Spec> {
        RtseW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<Cr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CtsieW<Cr3Spec> {
        CtsieW::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&mut self) -> OnebitW<Cr3Spec> {
        OnebitW::new(self, 11)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0;
}