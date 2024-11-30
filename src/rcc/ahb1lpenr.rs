#[doc = "Register `AHB1LPENR` reader"]
pub type R = crate::R<Ahb1lpenrSpec>;
#[doc = "Register `AHB1LPENR` writer"]
pub type W = crate::W<Ahb1lpenrSpec>;
#[doc = "Field `GPIOALPEN` reader - IO port A clock enable during sleep mode"]
pub type GpioalpenR = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - IO port A clock enable during sleep mode"]
pub type GpioalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLPEN` reader - IO port B clock enable during Sleep mode"]
pub type GpioblpenR = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - IO port B clock enable during Sleep mode"]
pub type GpioblpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLPEN` reader - IO port C clock enable during Sleep mode"]
pub type GpioclpenR = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - IO port C clock enable during Sleep mode"]
pub type GpioclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLPEN` reader - IO port D clock enable during Sleep mode"]
pub type GpiodlpenR = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - IO port D clock enable during Sleep mode"]
pub type GpiodlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELPEN` reader - IO port E clock enable during Sleep mode"]
pub type GpioelpenR = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - IO port E clock enable during Sleep mode"]
pub type GpioelpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLPEN` reader - IO port H clock enable during Sleep mode"]
pub type GpiohlpenR = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - IO port H clock enable during Sleep mode"]
pub type GpiohlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLPEN` reader - CRC clock enable during Sleep mode"]
pub type CrclpenR = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC clock enable during Sleep mode"]
pub type CrclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFLPEN` reader - Flash interface clock enable during Sleep mode"]
pub type FlitflpenR = crate::BitReader;
#[doc = "Field `FLITFLPEN` writer - Flash interface clock enable during Sleep mode"]
pub type FlitflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1LPEN` reader - SRAM 1interface clock enable during Sleep mode"]
pub type Sram1lpenR = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM 1interface clock enable during Sleep mode"]
pub type Sram1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA1LPEN` reader - DMA1 clock enable during Sleep mode"]
pub type Dma1lpenR = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 clock enable during Sleep mode"]
pub type Dma1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LPEN` reader - DMA2 clock enable during Sleep mode"]
pub type Dma2lpenR = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 clock enable during Sleep mode"]
pub type Dma2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GpioalpenR {
        GpioalpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GpioblpenR {
        GpioblpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GpioclpenR {
        GpioclpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GpiodlpenR {
        GpiodlpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GpioelpenR {
        GpioelpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GpiohlpenR {
        GpiohlpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CrclpenR {
        CrclpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FlitflpenR {
        FlitflpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> Sram1lpenR {
        Sram1lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> Dma1lpenR {
        Dma1lpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> Dma2lpenR {
        Dma2lpenR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable during sleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GpioalpenW<Ahb1lpenrSpec> {
        GpioalpenW::new(self, 0)
    }
    #[doc = "Bit 1 - IO port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GpioblpenW<Ahb1lpenrSpec> {
        GpioblpenW::new(self, 1)
    }
    #[doc = "Bit 2 - IO port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GpioclpenW<Ahb1lpenrSpec> {
        GpioclpenW::new(self, 2)
    }
    #[doc = "Bit 3 - IO port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GpiodlpenW<Ahb1lpenrSpec> {
        GpiodlpenW::new(self, 3)
    }
    #[doc = "Bit 4 - IO port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GpioelpenW<Ahb1lpenrSpec> {
        GpioelpenW::new(self, 4)
    }
    #[doc = "Bit 7 - IO port H clock enable during Sleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GpiohlpenW<Ahb1lpenrSpec> {
        GpiohlpenW::new(self, 7)
    }
    #[doc = "Bit 12 - CRC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> CrclpenW<Ahb1lpenrSpec> {
        CrclpenW::new(self, 12)
    }
    #[doc = "Bit 15 - Flash interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FlitflpenW<Ahb1lpenrSpec> {
        FlitflpenW::new(self, 15)
    }
    #[doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> Sram1lpenW<Ahb1lpenrSpec> {
        Sram1lpenW::new(self, 16)
    }
    #[doc = "Bit 21 - DMA1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> Dma1lpenW<Ahb1lpenrSpec> {
        Dma1lpenW::new(self, 21)
    }
    #[doc = "Bit 22 - DMA2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> Dma2lpenW<Ahb1lpenrSpec> {
        Dma2lpenW::new(self, 22)
    }
}
#[doc = "AHB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1lpenrSpec;
impl crate::RegisterSpec for Ahb1lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for Ahb1lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for Ahb1lpenrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0x7e67_91ff"]
impl crate::Resettable for Ahb1lpenrSpec {
    const RESET_VALUE: u32 = 0x7e67_91ff;
}
