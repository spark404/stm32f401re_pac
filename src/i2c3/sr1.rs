#[doc = "Register `SR1` reader"]
pub type R = crate::R<SR1rs>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<SR1rs>;
#[doc = "Field `SB` reader - Start bit (Master mode)"]
pub type SB_R = crate::BitReader;
#[doc = "Field `ADDR` reader - Address sent (master mode)/matched (slave mode)"]
pub type ADDR_R = crate::BitReader;
#[doc = "Field `BTF` reader - Byte transfer finished"]
pub type BTF_R = crate::BitReader;
#[doc = "Field `ADD10` reader - 10-bit header sent (Master mode)"]
pub type ADD10_R = crate::BitReader;
#[doc = "Field `STOPF` reader - Stop detection (slave mode)"]
pub type STOPF_R = crate::BitReader;
#[doc = "Field `RxNE` reader - Data register not empty (receivers)"]
pub type RX_NE_R = crate::BitReader;
#[doc = "Field `TxE` reader - Data register empty (transmitters)"]
pub type TX_E_R = crate::BitReader;
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader;
#[doc = "Field `BERR` writer - Bus error"]
pub type BERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLO` reader - Arbitration lost (master mode)"]
pub type ARLO_R = crate::BitReader;
#[doc = "Field `ARLO` writer - Arbitration lost (master mode)"]
pub type ARLO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AF` reader - Acknowledge failure"]
pub type AF_R = crate::BitReader;
#[doc = "Field `AF` writer - Acknowledge failure"]
pub type AF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - Overrun/Underrun"]
pub type OVR_R = crate::BitReader;
#[doc = "Field `OVR` writer - Overrun/Underrun"]
pub type OVR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader;
#[doc = "Field `PECERR` writer - PEC Error in reception"]
pub type PECERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - Timeout or Tlow error"]
pub type TIMEOUT_R = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - Timeout or Tlow error"]
pub type TIMEOUT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALERT` reader - SMBus alert"]
pub type SMBALERT_R = crate::BitReader;
#[doc = "Field `SMBALERT` writer - SMBus alert"]
pub type SMBALERT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start bit (Master mode)"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address sent (master mode)/matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Byte transfer finished"]
    #[inline(always)]
    pub fn btf(&self) -> BTF_R {
        BTF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 10-bit header sent (Master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stop detection (slave mode)"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Data register not empty (receivers)"]
    #[inline(always)]
    pub fn rx_ne(&self) -> RX_NE_R {
        RX_NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data register empty (transmitters)"]
    #[inline(always)]
    pub fn tx_e(&self) -> TX_E_R {
        TX_E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&self) -> AF_R {
        AF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR1")
            .field("smbalert", &self.smbalert())
            .field("timeout", &self.timeout())
            .field("pecerr", &self.pecerr())
            .field("ovr", &self.ovr())
            .field("af", &self.af())
            .field("arlo", &self.arlo())
            .field("berr", &self.berr())
            .field("tx_e", &self.tx_e())
            .field("rx_ne", &self.rx_ne())
            .field("stopf", &self.stopf())
            .field("add10", &self.add10())
            .field("btf", &self.btf())
            .field("addr", &self.addr())
            .field("sb", &self.sb())
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W<SR1rs> {
        BERR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration lost (master mode)"]
    #[inline(always)]
    pub fn arlo(&mut self) -> ARLO_W<SR1rs> {
        ARLO_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge failure"]
    #[inline(always)]
    pub fn af(&mut self) -> AF_W<SR1rs> {
        AF_W::new(self, 10)
    }
    #[doc = "Bit 11 - Overrun/Underrun"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<SR1rs> {
        OVR_W::new(self, 11)
    }
    #[doc = "Bit 12 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&mut self) -> PECERR_W<SR1rs> {
        PECERR_W::new(self, 12)
    }
    #[doc = "Bit 14 - Timeout or Tlow error"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<SR1rs> {
        TIMEOUT_W::new(self, 14)
    }
    #[doc = "Bit 15 - SMBus alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W<SR1rs> {
        SMBALERT_W::new(self, 15)
    }
}
#[doc = "Status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR1rs;
impl crate::RegisterSpec for SR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for SR1rs {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for SR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for SR1rs {
    const RESET_VALUE: u32 = 0;
}
