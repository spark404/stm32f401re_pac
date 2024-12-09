#[doc = "Register `DBGMCU_CR` reader"]
pub type R = crate::R<DBGMCU_CRrs>;
#[doc = "Register `DBGMCU_CR` writer"]
pub type W = crate::W<DBGMCU_CRrs>;
#[doc = "Field `DBG_SLEEP` reader - DBG_SLEEP"]
pub type DBG_SLEEP_R = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - DBG_SLEEP"]
pub type DBG_SLEEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - DBG_STOP"]
pub type DBG_STOP_R = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - DBG_STOP"]
pub type DBG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STANDBY` reader - DBG_STANDBY"]
pub type DBG_STANDBY_R = crate::BitReader;
#[doc = "Field `DBG_STANDBY` writer - DBG_STANDBY"]
pub type DBG_STANDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_IOEN` reader - TRACE_IOEN"]
pub type TRACE_IOEN_R = crate::BitReader;
#[doc = "Field `TRACE_IOEN` writer - TRACE_IOEN"]
pub type TRACE_IOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACE_MODE` reader - TRACE_MODE"]
pub type TRACE_MODE_R = crate::FieldReader;
#[doc = "Field `TRACE_MODE` writer - TRACE_MODE"]
pub type TRACE_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DBG_SLEEP_R {
        DBG_SLEEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DBG_STOP_R {
        DBG_STOP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&self) -> DBG_STANDBY_R {
        DBG_STANDBY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&self) -> TRACE_IOEN_R {
        TRACE_IOEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&self) -> TRACE_MODE_R {
        TRACE_MODE_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGMCU_CR")
            .field("dbg_sleep", &self.dbg_sleep())
            .field("dbg_stop", &self.dbg_stop())
            .field("dbg_standby", &self.dbg_standby())
            .field("trace_ioen", &self.trace_ioen())
            .field("trace_mode", &self.trace_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DBG_SLEEP"]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DBG_SLEEP_W<DBGMCU_CRrs> {
        DBG_SLEEP_W::new(self, 0)
    }
    #[doc = "Bit 1 - DBG_STOP"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DBG_STOP_W<DBGMCU_CRrs> {
        DBG_STOP_W::new(self, 1)
    }
    #[doc = "Bit 2 - DBG_STANDBY"]
    #[inline(always)]
    pub fn dbg_standby(&mut self) -> DBG_STANDBY_W<DBGMCU_CRrs> {
        DBG_STANDBY_W::new(self, 2)
    }
    #[doc = "Bit 5 - TRACE_IOEN"]
    #[inline(always)]
    pub fn trace_ioen(&mut self) -> TRACE_IOEN_W<DBGMCU_CRrs> {
        TRACE_IOEN_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - TRACE_MODE"]
    #[inline(always)]
    pub fn trace_mode(&mut self) -> TRACE_MODE_W<DBGMCU_CRrs> {
        TRACE_MODE_W::new(self, 6)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgmcu_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgmcu_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGMCU_CRrs;
impl crate::RegisterSpec for DBGMCU_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbgmcu_cr::R`](R) reader structure"]
impl crate::Readable for DBGMCU_CRrs {}
#[doc = "`write(|w| ..)` method takes [`dbgmcu_cr::W`](W) writer structure"]
impl crate::Writable for DBGMCU_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGMCU_CR to value 0"]
impl crate::Resettable for DBGMCU_CRrs {
    const RESET_VALUE: u32 = 0;
}
