#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBUS` reader - SMBus mode"]
pub type SMBUS_R = crate::BitReader;
#[doc = "Field `SMBUS` writer - SMBus mode"]
pub type SMBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBTYPE` reader - SMBus type"]
pub type SMBTYPE_R = crate::BitReader;
#[doc = "Field `SMBTYPE` writer - SMBus type"]
pub type SMBTYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENARP` reader - ARP enable"]
pub type ENARP_R = crate::BitReader;
#[doc = "Field `ENARP` writer - ARP enable"]
pub type ENARP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENPEC` reader - PEC enable"]
pub type ENPEC_R = crate::BitReader;
#[doc = "Field `ENPEC` writer - PEC enable"]
pub type ENPEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENGC` reader - General call enable"]
pub type ENGC_R = crate::BitReader;
#[doc = "Field `ENGC` writer - General call enable"]
pub type ENGC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_R = crate::BitReader;
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - Stop generation"]
pub type STOP_R = crate::BitReader;
#[doc = "Field `STOP` writer - Stop generation"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - Acknowledge enable"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - Acknowledge enable"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS` reader - Acknowledge/PEC Position (for data reception)"]
pub type POS_R = crate::BitReader;
#[doc = "Field `POS` writer - Acknowledge/PEC Position (for data reception)"]
pub type POS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PEC_R = crate::BitReader;
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type ALERT_R = crate::BitReader;
#[doc = "Field `ALERT` writer - SMBus alert"]
pub type ALERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader;
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("swrst", &self.swrst())
            .field("alert", &self.alert())
            .field("pec", &self.pec())
            .field("pos", &self.pos())
            .field("ack", &self.ack())
            .field("stop", &self.stop())
            .field("start", &self.start())
            .field("nostretch", &self.nostretch())
            .field("engc", &self.engc())
            .field("enpec", &self.enpec())
            .field("enarp", &self.enarp())
            .field("smbtype", &self.smbtype())
            .field("smbus", &self.smbus())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<CR1rs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&mut self) -> SMBUS_W<CR1rs> {
        SMBUS_W::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&mut self) -> SMBTYPE_W<CR1rs> {
        SMBTYPE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&mut self) -> ENARP_W<CR1rs> {
        ENARP_W::new(self, 4)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&mut self) -> ENPEC_W<CR1rs> {
        ENPEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&mut self) -> ENGC_W<CR1rs> {
        ENGC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<CR1rs> {
        NOSTRETCH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W<CR1rs> {
        START_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<CR1rs> {
        STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<CR1rs> {
        ACK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&mut self) -> POS_W<CR1rs> {
        POS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<CR1rs> {
        PEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W<CR1rs> {
        ALERT_W::new(self, 13)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<CR1rs> {
        SWRST_W::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
