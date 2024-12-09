#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `CCRCFAILC` reader - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_R = crate::BitReader;
#[doc = "Field `CCRCFAILC` writer - CCRCFAIL flag clear bit"]
pub type CCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAIL flag clear bit"]
pub type DCRCFAILC_R = crate::BitReader;
#[doc = "Field `DCRCFAILC` writer - DCRCFAIL flag clear bit"]
pub type DCRCFAILC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit"]
pub type CTIMEOUTC_R = crate::BitReader;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit"]
pub type CTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit"]
pub type DTIMEOUTC_R = crate::BitReader;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit"]
pub type DTIMEOUTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERR flag clear bit"]
pub type TXUNDERRC_R = crate::BitReader;
#[doc = "Field `TXUNDERRC` writer - TXUNDERR flag clear bit"]
pub type TXUNDERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRC` reader - RXOVERR flag clear bit"]
pub type RXOVERRC_R = crate::BitReader;
#[doc = "Field `RXOVERRC` writer - RXOVERR flag clear bit"]
pub type RXOVERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDC` reader - CMDREND flag clear bit"]
pub type CMDRENDC_R = crate::BitReader;
#[doc = "Field `CMDRENDC` writer - CMDREND flag clear bit"]
pub type CMDRENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTC` reader - CMDSENT flag clear bit"]
pub type CMDSENTC_R = crate::BitReader;
#[doc = "Field `CMDSENTC` writer - CMDSENT flag clear bit"]
pub type CMDSENTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDC` reader - DATAEND flag clear bit"]
pub type DATAENDC_R = crate::BitReader;
#[doc = "Field `DATAENDC` writer - DATAEND flag clear bit"]
pub type DATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBITERRC` reader - STBITERR flag clear bit"]
pub type STBITERRC_R = crate::BitReader;
#[doc = "Field `STBITERRC` writer - STBITERR flag clear bit"]
pub type STBITERRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKEND flag clear bit"]
pub type DBCKENDC_R = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKEND flag clear bit"]
pub type DBCKENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOIT flag clear bit"]
pub type SDIOITC_R = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOIT flag clear bit"]
pub type SDIOITC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATAENDC` reader - CEATAEND flag clear bit"]
pub type CEATAENDC_R = crate::BitReader;
#[doc = "Field `CEATAENDC` writer - CEATAEND flag clear bit"]
pub type CEATAENDC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CCRCFAILC_R {
        CCRCFAILC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DCRCFAILC_R {
        DCRCFAILC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CTIMEOUTC_R {
        CTIMEOUTC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DTIMEOUTC_R {
        DTIMEOUTC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    pub fn txunderrc(&self) -> TXUNDERRC_R {
        TXUNDERRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RXOVERRC_R {
        RXOVERRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CMDRENDC_R {
        CMDRENDC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CMDSENTC_R {
        CMDSENTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    pub fn dataendc(&self) -> DATAENDC_R {
        DATAENDC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERR flag clear bit"]
    #[inline(always)]
    pub fn stbiterrc(&self) -> STBITERRC_R {
        STBITERRC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    pub fn dbckendc(&self) -> DBCKENDC_R {
        DBCKENDC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    pub fn sdioitc(&self) -> SDIOITC_R {
        SDIOITC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CEATAEND flag clear bit"]
    #[inline(always)]
    pub fn ceataendc(&self) -> CEATAENDC_R {
        CEATAENDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICR")
            .field("ceataendc", &self.ceataendc())
            .field("sdioitc", &self.sdioitc())
            .field("dbckendc", &self.dbckendc())
            .field("stbiterrc", &self.stbiterrc())
            .field("dataendc", &self.dataendc())
            .field("cmdsentc", &self.cmdsentc())
            .field("cmdrendc", &self.cmdrendc())
            .field("rxoverrc", &self.rxoverrc())
            .field("txunderrc", &self.txunderrc())
            .field("dtimeoutc", &self.dtimeoutc())
            .field("ctimeoutc", &self.ctimeoutc())
            .field("dcrcfailc", &self.dcrcfailc())
            .field("ccrcfailc", &self.ccrcfailc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CCRCFAILC_W<ICRrs> {
        CCRCFAILC_W::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit"]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DCRCFAILC_W<ICRrs> {
        DCRCFAILC_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CTIMEOUTC_W<ICRrs> {
        CTIMEOUTC_W::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit"]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DTIMEOUTC_W<ICRrs> {
        DTIMEOUTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit"]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TXUNDERRC_W<ICRrs> {
        TXUNDERRC_W::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit"]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RXOVERRC_W<ICRrs> {
        RXOVERRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit"]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CMDRENDC_W<ICRrs> {
        CMDRENDC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit"]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CMDSENTC_W<ICRrs> {
        CMDSENTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit"]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DATAENDC_W<ICRrs> {
        DATAENDC_W::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERR flag clear bit"]
    #[inline(always)]
    pub fn stbiterrc(&mut self) -> STBITERRC_W<ICRrs> {
        STBITERRC_W::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit"]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DBCKENDC_W<ICRrs> {
        DBCKENDC_W::new(self, 10)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit"]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SDIOITC_W<ICRrs> {
        SDIOITC_W::new(self, 22)
    }
    #[doc = "Bit 23 - CEATAEND flag clear bit"]
    #[inline(always)]
    pub fn ceataendc(&mut self) -> CEATAENDC_W<ICRrs> {
        CEATAENDC_W::new(self, 23)
    }
}
#[doc = "interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
