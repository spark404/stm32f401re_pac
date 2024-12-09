#[doc = "Register `DBGMCU_APB1_FZ` reader"]
pub type R = crate::R<DBGMCU_APB1_FZrs>;
#[doc = "Register `DBGMCU_APB1_FZ` writer"]
pub type W = crate::W<DBGMCU_APB1_FZrs>;
#[doc = "Field `DBG_TIM2_STOP` reader - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM2_STOP` writer - DBG_TIM2_STOP"]
pub type DBG_TIM2_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - DBG_TIM3 _STOP"]
pub type DBG_TIM3_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - DBG_TIM3 _STOP"]
pub type DBG_TIM3_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4_STOP` reader - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM4_STOP` writer - DBG_TIM4_STOP"]
pub type DBG_TIM4_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5_STOP` reader - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_R = crate::BitReader;
#[doc = "Field `DBG_TIM5_STOP` writer - DBG_TIM5_STOP"]
pub type DBG_TIM5_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC_Stop` reader - RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_R = crate::BitReader;
#[doc = "Field `DBG_RTC_Stop` writer - RTC stopped when Core is halted"]
pub type DBG_RTC_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG_STOP` reader - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_WWDG_STOP` writer - DBG_WWDG_STOP"]
pub type DBG_WWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDEG_STOP` reader - DBG_IWDEG_STOP"]
pub type DBG_IWDEG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_IWDEG_STOP` writer - DBG_IWDEG_STOP"]
pub type DBG_IWDEG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` reader - DBG_J2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C1_SMBUS_TIMEOUT` writer - DBG_J2C1_SMBUS_TIMEOUT"]
pub type DBG_I2C1_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` reader - DBG_J2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C2_SMBUS_TIMEOUT` writer - DBG_J2C2_SMBUS_TIMEOUT"]
pub type DBG_I2C2_SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3SMBUS_TIMEOUT` reader - DBG_J2C3SMBUS_TIMEOUT"]
pub type DBG_I2C3SMBUS_TIMEOUT_R = crate::BitReader;
#[doc = "Field `DBG_I2C3SMBUS_TIMEOUT` writer - DBG_J2C3SMBUS_TIMEOUT"]
pub type DBG_I2C3SMBUS_TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&self) -> DBG_TIM2_STOP_R {
        DBG_TIM2_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DBG_TIM3_STOP_R {
        DBG_TIM3_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&self) -> DBG_TIM4_STOP_R {
        DBG_TIM4_STOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&self) -> DBG_TIM5_STOP_R {
        DBG_TIM5_STOP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&self) -> DBG_WWDG_STOP_R {
        DBG_WWDG_STOP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdeg_stop(&self) -> DBG_IWDEG_STOP_R {
        DBG_IWDEG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&self) -> DBG_I2C1_SMBUS_TIMEOUT_R {
        DBG_I2C1_SMBUS_TIMEOUT_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&self) -> DBG_I2C2_SMBUS_TIMEOUT_R {
        DBG_I2C2_SMBUS_TIMEOUT_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c3smbus_timeout(&self) -> DBG_I2C3SMBUS_TIMEOUT_R {
        DBG_I2C3SMBUS_TIMEOUT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_APB1_FZ")
            .field("dbg_tim2_stop", &self.dbg_tim2_stop())
            .field("dbg_tim3_stop", &self.dbg_tim3_stop())
            .field("dbg_tim4_stop", &self.dbg_tim4_stop())
            .field("dbg_tim5_stop", &self.dbg_tim5_stop())
            .field("dbg_rtc_stop", &self.dbg_rtc_stop())
            .field("dbg_wwdg_stop", &self.dbg_wwdg_stop())
            .field("dbg_iwdeg_stop", &self.dbg_iwdeg_stop())
            .field("dbg_i2c1_smbus_timeout", &self.dbg_i2c1_smbus_timeout())
            .field("dbg_i2c2_smbus_timeout", &self.dbg_i2c2_smbus_timeout())
            .field("dbg_i2c3smbus_timeout", &self.dbg_i2c3smbus_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DBG_TIM2_STOP"]
    #[inline(always)]
    pub fn dbg_tim2_stop(&mut self) -> DBG_TIM2_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_TIM2_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_TIM3 _STOP"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DBG_TIM3_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_TIM3_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_TIM4_STOP"]
    #[inline(always)]
    pub fn dbg_tim4_stop(&mut self) -> DBG_TIM4_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_TIM4_STOP_W::new(self, 2)
    }
    #[doc = "Bit 3 - DBG_TIM5_STOP"]
    #[inline(always)]
    pub fn dbg_tim5_stop(&mut self) -> DBG_TIM5_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_TIM5_STOP_W::new(self, 3)
    }
    #[doc = "Bit 10 - RTC stopped when Core is halted"]
    #[inline(always)]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_RTC_STOP_W::new(self, 10)
    }
    #[doc = "Bit 11 - DBG_WWDG_STOP"]
    #[inline(always)]
    pub fn dbg_wwdg_stop(&mut self) -> DBG_WWDG_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_WWDG_STOP_W::new(self, 11)
    }
    #[doc = "Bit 12 - DBG_IWDEG_STOP"]
    #[inline(always)]
    pub fn dbg_iwdeg_stop(&mut self) -> DBG_IWDEG_STOP_W<DBGMCU_APB1_FZrs> {
        DBG_IWDEG_STOP_W::new(self, 12)
    }
    #[doc = "Bit 21 - DBG_J2C1_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c1_smbus_timeout(&mut self) -> DBG_I2C1_SMBUS_TIMEOUT_W<DBGMCU_APB1_FZrs> {
        DBG_I2C1_SMBUS_TIMEOUT_W::new(self, 21)
    }
    #[doc = "Bit 22 - DBG_J2C2_SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c2_smbus_timeout(&mut self) -> DBG_I2C2_SMBUS_TIMEOUT_W<DBGMCU_APB1_FZrs> {
        DBG_I2C2_SMBUS_TIMEOUT_W::new(self, 22)
    }
    #[doc = "Bit 23 - DBG_J2C3SMBUS_TIMEOUT"]
    #[inline(always)]
    pub fn dbg_i2c3smbus_timeout(&mut self) -> DBG_I2C3SMBUS_TIMEOUT_W<DBGMCU_APB1_FZrs> {
        DBG_I2C3SMBUS_TIMEOUT_W::new(self, 23)
    }
}
#[doc = "Debug MCU APB1 Freeze registe\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_apb1_fz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_apb1_fz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_APB1_FZrs;
impl crate::RegisterSpec for DBGMCU_APB1_FZrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_apb1_fz::R`](R) reader structure"]
impl crate::Readable for DBGMCU_APB1_FZrs {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_apb1_fz::W`](W) writer structure"]
impl crate::Writable for DBGMCU_APB1_FZrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_APB1_FZ to value 0"]
impl crate::Resettable for DBGMCU_APB1_FZrs {
    const RESET_VALUE: u32 = 0;
}
