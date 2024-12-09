#[doc = "Register `APB1LPENR` reader"]
pub type R = crate::R<APB1LPENRrs>;
#[doc = "Register `APB1LPENR` writer"]
pub type W = crate::W<APB1LPENRrs>;
#[doc = "Field `TIM2LPEN` reader - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_R = crate::BitReader;
#[doc = "Field `TIM2LPEN` writer - TIM2 clock enable during Sleep mode"]
pub type TIM2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3LPEN` reader - TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_R = crate::BitReader;
#[doc = "Field `TIM3LPEN` writer - TIM3 clock enable during Sleep mode"]
pub type TIM3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4LPEN` reader - TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_R = crate::BitReader;
#[doc = "Field `TIM4LPEN` writer - TIM4 clock enable during Sleep mode"]
pub type TIM4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5LPEN` reader - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_R = crate::BitReader;
#[doc = "Field `TIM5LPEN` writer - TIM5 clock enable during Sleep mode"]
pub type TIM5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGLPEN` reader - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_R = crate::BitReader;
#[doc = "Field `WWDGLPEN` writer - Window watchdog clock enable during Sleep mode"]
pub type WWDGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2LPEN` reader - SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_R = crate::BitReader;
#[doc = "Field `SPI2LPEN` writer - SPI2 clock enable during Sleep mode"]
pub type SPI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3LPEN` reader - SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_R = crate::BitReader;
#[doc = "Field `SPI3LPEN` writer - SPI3 clock enable during Sleep mode"]
pub type SPI3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2LPEN` reader - USART2 clock enable during Sleep mode"]
pub type USART2LPEN_R = crate::BitReader;
#[doc = "Field `USART2LPEN` writer - USART2 clock enable during Sleep mode"]
pub type USART2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1LPEN` reader - I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_R = crate::BitReader;
#[doc = "Field `I2C1LPEN` writer - I2C1 clock enable during Sleep mode"]
pub type I2C1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2LPEN` reader - I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_R = crate::BitReader;
#[doc = "Field `I2C2LPEN` writer - I2C2 clock enable during Sleep mode"]
pub type I2C2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3LPEN` reader - I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_R = crate::BitReader;
#[doc = "Field `I2C3LPEN` writer - I2C3 clock enable during Sleep mode"]
pub type I2C3LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRLPEN` reader - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_R = crate::BitReader;
#[doc = "Field `PWRLPEN` writer - Power interface clock enable during Sleep mode"]
pub type PWRLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&self) -> WWDGLPEN_R {
        WWDGLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1LPENR")
            .field("pwrlpen", &self.pwrlpen())
            .field("i2c3lpen", &self.i2c3lpen())
            .field("i2c2lpen", &self.i2c2lpen())
            .field("i2c1lpen", &self.i2c1lpen())
            .field("usart2lpen", &self.usart2lpen())
            .field("spi3lpen", &self.spi3lpen())
            .field("spi2lpen", &self.spi2lpen())
            .field("wwdglpen", &self.wwdglpen())
            .field("tim5lpen", &self.tim5lpen())
            .field("tim4lpen", &self.tim4lpen())
            .field("tim3lpen", &self.tim3lpen())
            .field("tim2lpen", &self.tim2lpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<APB1LPENRrs> {
        TIM2LPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<APB1LPENRrs> {
        TIM3LPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<APB1LPENRrs> {
        TIM4LPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<APB1LPENRrs> {
        TIM5LPEN_W::new(self, 3)
    }
    #[doc = "Bit 11 - Window watchdog clock enable during Sleep mode"]
    #[inline(always)]
    pub fn wwdglpen(&mut self) -> WWDGLPEN_W<APB1LPENRrs> {
        WWDGLPEN_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<APB1LPENRrs> {
        SPI2LPEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<APB1LPENRrs> {
        SPI3LPEN_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<APB1LPENRrs> {
        USART2LPEN_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<APB1LPENRrs> {
        I2C1LPEN_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<APB1LPENRrs> {
        I2C2LPEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<APB1LPENRrs> {
        I2C3LPEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - Power interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<APB1LPENRrs> {
        PWRLPEN_W::new(self, 28)
    }
}
#[doc = "APB1 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1LPENRrs;
impl crate::RegisterSpec for APB1LPENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lpenr::R`](R) reader structure"]
impl crate::Readable for APB1LPENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1lpenr::W`](W) writer structure"]
impl crate::Writable for APB1LPENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1LPENR to value 0x36fe_c9ff"]
impl crate::Resettable for APB1LPENRrs {
    const RESET_VALUE: u32 = 0x36fe_c9ff;
}
