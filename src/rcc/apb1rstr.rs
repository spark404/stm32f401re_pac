#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<APB1RSTRrs>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<APB1RSTRrs>;
#[doc = "Field `TIM2RST` reader - TIM2 reset"]
pub type TIM2RST_R = crate::BitReader;
#[doc = "Field `TIM2RST` writer - TIM2 reset"]
pub type TIM2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3RST` reader - TIM3 reset"]
pub type TIM3RST_R = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM3 reset"]
pub type TIM3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4RST` reader - TIM4 reset"]
pub type TIM4RST_R = crate::BitReader;
#[doc = "Field `TIM4RST` writer - TIM4 reset"]
pub type TIM4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5RST` reader - TIM5 reset"]
pub type TIM5RST_R = crate::BitReader;
#[doc = "Field `TIM5RST` writer - TIM5 reset"]
pub type TIM5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDGRST` reader - Window watchdog reset"]
pub type WWDGRST_R = crate::BitReader;
#[doc = "Field `WWDGRST` writer - Window watchdog reset"]
pub type WWDGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI 2 reset"]
pub type SPI2RST_R = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI 2 reset"]
pub type SPI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI 3 reset"]
pub type SPI3RST_R = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI 3 reset"]
pub type SPI3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2RST` reader - USART 2 reset"]
pub type UART2RST_R = crate::BitReader;
#[doc = "Field `UART2RST` writer - USART 2 reset"]
pub type UART2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C 1 reset"]
pub type I2C1RST_R = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C 1 reset"]
pub type I2C1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C 2 reset"]
pub type I2C2RST_R = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C 2 reset"]
pub type I2C2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3 reset"]
pub type I2C3RST_R = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3 reset"]
pub type I2C3RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRRST` reader - Power interface reset"]
pub type PWRRST_R = crate::BitReader;
#[doc = "Field `PWRRST` writer - Power interface reset"]
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> TIM2RST_R {
        TIM2RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> TIM3RST_R {
        TIM3RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> TIM4RST_R {
        TIM4RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> TIM5RST_R {
        TIM5RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&self) -> WWDGRST_R {
        WWDGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> SPI2RST_R {
        SPI2RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> SPI3RST_R {
        SPI3RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn uart2rst(&self) -> UART2RST_R {
        UART2RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2C1RST_R {
        I2C1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2C2RST_R {
        I2C2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2C3RST_R {
        I2C3RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1RSTR")
            .field("pwrrst", &self.pwrrst())
            .field("i2c3rst", &self.i2c3rst())
            .field("i2c2rst", &self.i2c2rst())
            .field("i2c1rst", &self.i2c1rst())
            .field("uart2rst", &self.uart2rst())
            .field("spi3rst", &self.spi3rst())
            .field("spi2rst", &self.spi2rst())
            .field("wwdgrst", &self.wwdgrst())
            .field("tim5rst", &self.tim5rst())
            .field("tim4rst", &self.tim4rst())
            .field("tim3rst", &self.tim3rst())
            .field("tim2rst", &self.tim2rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> TIM2RST_W<APB1RSTRrs> {
        TIM2RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> TIM3RST_W<APB1RSTRrs> {
        TIM3RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> TIM4RST_W<APB1RSTRrs> {
        TIM4RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> TIM5RST_W<APB1RSTRrs> {
        TIM5RST_W::new(self, 3)
    }
    #[doc = "Bit 11 - Window watchdog reset"]
    #[inline(always)]
    pub fn wwdgrst(&mut self) -> WWDGRST_W<APB1RSTRrs> {
        WWDGRST_W::new(self, 11)
    }
    #[doc = "Bit 14 - SPI 2 reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> SPI2RST_W<APB1RSTRrs> {
        SPI2RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - SPI 3 reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> SPI3RST_W<APB1RSTRrs> {
        SPI3RST_W::new(self, 15)
    }
    #[doc = "Bit 17 - USART 2 reset"]
    #[inline(always)]
    pub fn uart2rst(&mut self) -> UART2RST_W<APB1RSTRrs> {
        UART2RST_W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C 1 reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2C1RST_W<APB1RSTRrs> {
        I2C1RST_W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C 2 reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2C2RST_W<APB1RSTRrs> {
        I2C2RST_W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2C3RST_W<APB1RSTRrs> {
        I2C3RST_W::new(self, 23)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<APB1RSTRrs> {
        PWRRST_W::new(self, 28)
    }
}
#[doc = "APB1 peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTRrs;
impl crate::RegisterSpec for APB1RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for APB1RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for APB1RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for APB1RSTRrs {
    const RESET_VALUE: u32 = 0;
}
