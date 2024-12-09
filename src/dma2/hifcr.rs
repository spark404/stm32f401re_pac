#[doc = "Register `HIFCR` writer"]
pub type W = crate::W<HIFCRrs>;
#[doc = "Field `CFEIF4` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF4` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF5` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF5` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF6` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF6` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF7` writer - Stream x clear FIFO error interrupt flag (x = 7..4)"]
pub type CFEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF7` writer - Stream x clear direct mode error interrupt flag (x = 7..4)"]
pub type CDMEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - Stream x clear transfer error interrupt flag (x = 7..4)"]
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - Stream x clear half transfer interrupt flag (x = 7..4)"]
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - Stream x clear transfer complete interrupt flag (x = 7..4)"]
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<HIFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif4(&mut self) -> CFEIF4_W<HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    #[doc = "Bit 2 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    #[doc = "Bit 3 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> CTEIF4_W<HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif4(&mut self) -> CHTIF4_W<HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif4(&mut self) -> CTCIF4_W<HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    #[doc = "Bit 6 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif5(&mut self) -> CFEIF5_W<HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    #[doc = "Bit 8 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> CTEIF5_W<HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    #[doc = "Bit 10 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif5(&mut self) -> CHTIF5_W<HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    #[doc = "Bit 11 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif5(&mut self) -> CTCIF5_W<HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    #[doc = "Bit 16 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif6(&mut self) -> CFEIF6_W<HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    #[doc = "Bit 18 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    #[doc = "Bit 19 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> CTEIF6_W<HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    #[doc = "Bit 20 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif6(&mut self) -> CHTIF6_W<HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    #[doc = "Bit 21 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif6(&mut self) -> CTCIF6_W<HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    #[doc = "Bit 22 - Stream x clear FIFO error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cfeif7(&mut self) -> CFEIF7_W<HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    #[doc = "Bit 24 - Stream x clear direct mode error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    #[doc = "Bit 25 - Stream x clear transfer error interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> CTEIF7_W<HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    #[doc = "Bit 26 - Stream x clear half transfer interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn chtif7(&mut self) -> CHTIF7_W<HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    #[doc = "Bit 27 - Stream x clear transfer complete interrupt flag (x = 7..4)"]
    #[inline(always)]
    pub fn ctcif7(&mut self) -> CTCIF7_W<HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
#[doc = "high interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIFCRrs;
impl crate::RegisterSpec for HIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hifcr::W`](W) writer structure"]
impl crate::Writable for HIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HIFCR to value 0"]
impl crate::Resettable for HIFCRrs {
    const RESET_VALUE: u32 = 0;
}
