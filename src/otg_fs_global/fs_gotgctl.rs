#[doc = "Register `FS_GOTGCTL` reader"]
pub type R = crate::R<FS_GOTGCTLrs>;
#[doc = "Register `FS_GOTGCTL` writer"]
pub type W = crate::W<FS_GOTGCTLrs>;
#[doc = "Field `SRQSCS` reader - Session request success"]
pub type SRQSCS_R = crate::BitReader;
#[doc = "Field `SRQ` reader - Session request"]
pub type SRQ_R = crate::BitReader;
#[doc = "Field `SRQ` writer - Session request"]
pub type SRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub type HNGSCS_R = crate::BitReader;
#[doc = "Field `HNPRQ` reader - HNP request"]
pub type HNPRQ_R = crate::BitReader;
#[doc = "Field `HNPRQ` writer - HNP request"]
pub type HNPRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub type HSHNPEN_R = crate::BitReader;
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub type HSHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DHNPEN_R = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DHNPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub type CIDSTS_R = crate::BitReader;
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub type DBCT_R = crate::BitReader;
#[doc = "Field `ASVLD` reader - A-session valid"]
pub type ASVLD_R = crate::BitReader;
#[doc = "Field `BSVLD` reader - B-session valid"]
pub type BSVLD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SRQSCS_R {
        SRQSCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SRQ_R {
        SRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HNGSCS_R {
        HNGSCS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HNPRQ_R {
        HNPRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HSHNPEN_R {
        HSHNPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DHNPEN_R {
        DHNPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CIDSTS_R {
        CIDSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DBCT_R {
        DBCT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> ASVLD_R {
        ASVLD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BSVLD_R {
        BSVLD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_GOTGCTL")
            .field("srqscs", &self.srqscs())
            .field("srq", &self.srq())
            .field("hngscs", &self.hngscs())
            .field("hnprq", &self.hnprq())
            .field("hshnpen", &self.hshnpen())
            .field("dhnpen", &self.dhnpen())
            .field("cidsts", &self.cidsts())
            .field("dbct", &self.dbct())
            .field("asvld", &self.asvld())
            .field("bsvld", &self.bsvld())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SRQ_W<FS_GOTGCTLrs> {
        SRQ_W::new(self, 1)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HNPRQ_W<FS_GOTGCTLrs> {
        HNPRQ_W::new(self, 9)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HSHNPEN_W<FS_GOTGCTLrs> {
        HSHNPEN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DHNPEN_W<FS_GOTGCTLrs> {
        DHNPEN_W::new(self, 11)
    }
}
#[doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_GOTGCTLrs;
impl crate::RegisterSpec for FS_GOTGCTLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_gotgctl::R`](R) reader structure"]
impl crate::Readable for FS_GOTGCTLrs {}
#[doc = "`write(|w| ..)` method takes [`fs_gotgctl::W`](W) writer structure"]
impl crate::Writable for FS_GOTGCTLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_GOTGCTL to value 0x0800"]
impl crate::Resettable for FS_GOTGCTLrs {
    const RESET_VALUE: u32 = 0x0800;
}
