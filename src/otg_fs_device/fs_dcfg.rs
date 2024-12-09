#[doc = "Register `FS_DCFG` reader"]
pub type R = crate::R<FS_DCFGrs>;
#[doc = "Register `FS_DCFG` writer"]
pub type W = crate::W<FS_DCFGrs>;
#[doc = "Field `DSPD` reader - Device speed"]
pub type DSPD_R = crate::FieldReader;
#[doc = "Field `DSPD` writer - Device speed"]
pub type DSPD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NZLSOHSK` reader - Non-zero-length status OUT handshake"]
pub type NZLSOHSK_R = crate::BitReader;
#[doc = "Field `NZLSOHSK` writer - Non-zero-length status OUT handshake"]
pub type NZLSOHSK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAD` reader - Device address"]
pub type DAD_R = crate::FieldReader;
#[doc = "Field `DAD` writer - Device address"]
pub type DAD_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PFIVL` reader - Periodic frame interval"]
pub type PFIVL_R = crate::FieldReader;
#[doc = "Field `PFIVL` writer - Periodic frame interval"]
pub type PFIVL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&self) -> DSPD_R {
        DSPD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&self) -> NZLSOHSK_R {
        NZLSOHSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DAD_R {
        DAD_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    pub fn pfivl(&self) -> PFIVL_R {
        PFIVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_DCFG")
            .field("dspd", &self.dspd())
            .field("nzlsohsk", &self.nzlsohsk())
            .field("dad", &self.dad())
            .field("pfivl", &self.pfivl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Device speed"]
    #[inline(always)]
    pub fn dspd(&mut self) -> DSPD_W<FS_DCFGrs> {
        DSPD_W::new(self, 0)
    }
    #[doc = "Bit 2 - Non-zero-length status OUT handshake"]
    #[inline(always)]
    pub fn nzlsohsk(&mut self) -> NZLSOHSK_W<FS_DCFGrs> {
        NZLSOHSK_W::new(self, 2)
    }
    #[doc = "Bits 4:10 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DAD_W<FS_DCFGrs> {
        DAD_W::new(self, 4)
    }
    #[doc = "Bits 11:12 - Periodic frame interval"]
    #[inline(always)]
    pub fn pfivl(&mut self) -> PFIVL_W<FS_DCFGrs> {
        PFIVL_W::new(self, 11)
    }
}
#[doc = "OTG_FS device configuration register (OTG_FS_DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_dcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_dcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_DCFGrs;
impl crate::RegisterSpec for FS_DCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_dcfg::R`](R) reader structure"]
impl crate::Readable for FS_DCFGrs {}
#[doc = "`write(|w| ..)` method takes [`fs_dcfg::W`](W) writer structure"]
impl crate::Writable for FS_DCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_DCFG to value 0x0220_0000"]
impl crate::Resettable for FS_DCFGrs {
    const RESET_VALUE: u32 = 0x0220_0000;
}
