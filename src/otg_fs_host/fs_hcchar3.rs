#[doc = "Register `FS_HCCHAR3` reader"]
pub type R = crate::R<FsHcchar3Spec>;
#[doc = "Register `FS_HCCHAR3` writer"]
pub type W = crate::W<FsHcchar3Spec>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MpsizR = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `EPNUM` reader - Endpoint number"]
pub type EpnumR = crate::FieldReader;
#[doc = "Field `EPNUM` writer - Endpoint number"]
pub type EpnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EPDIR` reader - Endpoint direction"]
pub type EpdirR = crate::BitReader;
#[doc = "Field `EPDIR` writer - Endpoint direction"]
pub type EpdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSDEV` reader - Low-speed device"]
pub type LsdevR = crate::BitReader;
#[doc = "Field `LSDEV` writer - Low-speed device"]
pub type LsdevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `EPTYP` writer - Endpoint type"]
pub type EptypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCNT` reader - Multicount"]
pub type McntR = crate::FieldReader;
#[doc = "Field `MCNT` writer - Multicount"]
pub type McntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAD` reader - Device address"]
pub type DadR = crate::FieldReader;
#[doc = "Field `DAD` writer - Device address"]
pub type DadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODDFRM` reader - Odd frame"]
pub type OddfrmR = crate::BitReader;
#[doc = "Field `ODDFRM` writer - Odd frame"]
pub type OddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHDIS` reader - Channel disable"]
pub type ChdisR = crate::BitReader;
#[doc = "Field `CHDIS` writer - Channel disable"]
pub type ChdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHENA` reader - Channel enable"]
pub type ChenaR = crate::BitReader;
#[doc = "Field `CHENA` writer - Channel enable"]
pub type ChenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&self) -> EpnumR {
        EpnumR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&self) -> EpdirR {
        EpdirR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&self) -> LsdevR {
        LsdevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mcnt(&self) -> McntR {
        McntR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&self) -> DadR {
        DadR::new(((self.bits >> 22) & 0x7f) as u8)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&self) -> OddfrmR {
        OddfrmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&self) -> ChdisR {
        ChdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&self) -> ChenaR {
        ChenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<FsHcchar3Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bits 11:14 - Endpoint number"]
    #[inline(always)]
    pub fn epnum(&mut self) -> EpnumW<FsHcchar3Spec> {
        EpnumW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint direction"]
    #[inline(always)]
    pub fn epdir(&mut self) -> EpdirW<FsHcchar3Spec> {
        EpdirW::new(self, 15)
    }
    #[doc = "Bit 17 - Low-speed device"]
    #[inline(always)]
    pub fn lsdev(&mut self) -> LsdevW<FsHcchar3Spec> {
        LsdevW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EptypW<FsHcchar3Spec> {
        EptypW::new(self, 18)
    }
    #[doc = "Bits 20:21 - Multicount"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> McntW<FsHcchar3Spec> {
        McntW::new(self, 20)
    }
    #[doc = "Bits 22:28 - Device address"]
    #[inline(always)]
    pub fn dad(&mut self) -> DadW<FsHcchar3Spec> {
        DadW::new(self, 22)
    }
    #[doc = "Bit 29 - Odd frame"]
    #[inline(always)]
    pub fn oddfrm(&mut self) -> OddfrmW<FsHcchar3Spec> {
        OddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Channel disable"]
    #[inline(always)]
    pub fn chdis(&mut self) -> ChdisW<FsHcchar3Spec> {
        ChdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Channel enable"]
    #[inline(always)]
    pub fn chena(&mut self) -> ChenaW<FsHcchar3Spec> {
        ChenaW::new(self, 31)
    }
}
#[doc = "OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hcchar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hcchar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsHcchar3Spec;
impl crate::RegisterSpec for FsHcchar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hcchar3::R`](R) reader structure"]
impl crate::Readable for FsHcchar3Spec {}
#[doc = "`write(|w| ..)` method takes [`fs_hcchar3::W`](W) writer structure"]
impl crate::Writable for FsHcchar3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_HCCHAR3 to value 0"]
impl crate::Resettable for FsHcchar3Spec {
    const RESET_VALUE: u32 = 0;
}