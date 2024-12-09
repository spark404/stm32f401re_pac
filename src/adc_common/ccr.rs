#[doc = "Register `CCR` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DDS` reader - DMA disable selection for multi-ADC mode"]
pub type DDS_R = crate::BitReader;
#[doc = "Field `DDS` writer - DMA disable selection for multi-ADC mode"]
pub type DDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA` reader - Direct memory access mode for multi ADC mode"]
pub type DMA_R = crate::FieldReader;
#[doc = "Field `DMA` writer - Direct memory access mode for multi ADC mode"]
pub type DMA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCPRE` reader - ADC prescaler"]
pub type ADCPRE_R = crate::FieldReader;
#[doc = "Field `ADCPRE` writer - ADC prescaler"]
pub type ADCPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VBATE` reader - VBAT enable"]
pub type VBATE_R = crate::BitReader;
#[doc = "Field `VBATE` writer - VBAT enable"]
pub type VBATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVREFE` reader - Temperature sensor and VREFINT enable"]
pub type TSVREFE_R = crate::BitReader;
#[doc = "Field `TSVREFE` writer - Temperature sensor and VREFINT enable"]
pub type TSVREFE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&self) -> VBATE_R {
        VBATE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&self) -> TSVREFE_R {
        TSVREFE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("tsvrefe", &self.tsvrefe())
            .field("vbate", &self.vbate())
            .field("adcpre", &self.adcpre())
            .field("dma", &self.dma())
            .field("dds", &self.dds())
            .field("delay", &self.delay())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&mut self) -> DELAY_W<CCRrs> {
        DELAY_W::new(self, 8)
    }
    #[doc = "Bit 13 - DMA disable selection for multi-ADC mode"]
    #[inline(always)]
    pub fn dds(&mut self) -> DDS_W<CCRrs> {
        DDS_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W<CCRrs> {
        DMA_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - ADC prescaler"]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W<CCRrs> {
        ADCPRE_W::new(self, 16)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbate(&mut self) -> VBATE_W<CCRrs> {
        VBATE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Temperature sensor and VREFINT enable"]
    #[inline(always)]
    pub fn tsvrefe(&mut self) -> TSVREFE_W<CCRrs> {
        TSVREFE_W::new(self, 23)
    }
}
#[doc = "ADC common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
