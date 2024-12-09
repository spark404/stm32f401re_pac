#[doc = "Register `FS_HPRT` reader"]
pub type R = crate::R<FS_HPRTrs>;
#[doc = "Register `FS_HPRT` writer"]
pub type W = crate::W<FS_HPRTrs>;
#[doc = "Field `PCSTS` reader - Port connect status"]
pub type PCSTS_R = crate::BitReader;
#[doc = "Field `PCDET` reader - Port connect detected"]
pub type PCDET_R = crate::BitReader;
#[doc = "Field `PCDET` writer - Port connect detected"]
pub type PCDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENA` reader - Port enable"]
pub type PENA_R = crate::BitReader;
#[doc = "Field `PENA` writer - Port enable"]
pub type PENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PENCHNG` reader - Port enable/disable change"]
pub type PENCHNG_R = crate::BitReader;
#[doc = "Field `PENCHNG` writer - Port enable/disable change"]
pub type PENCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POCA` reader - Port overcurrent active"]
pub type POCA_R = crate::BitReader;
#[doc = "Field `POCCHNG` reader - Port overcurrent change"]
pub type POCCHNG_R = crate::BitReader;
#[doc = "Field `POCCHNG` writer - Port overcurrent change"]
pub type POCCHNG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRES` reader - Port resume"]
pub type PRES_R = crate::BitReader;
#[doc = "Field `PRES` writer - Port resume"]
pub type PRES_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSUSP` reader - Port suspend"]
pub type PSUSP_R = crate::BitReader;
#[doc = "Field `PSUSP` writer - Port suspend"]
pub type PSUSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRST` reader - Port reset"]
pub type PRST_R = crate::BitReader;
#[doc = "Field `PRST` writer - Port reset"]
pub type PRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSTS` reader - Port line status"]
pub type PLSTS_R = crate::FieldReader;
#[doc = "Field `PPWR` reader - Port power"]
pub type PPWR_R = crate::BitReader;
#[doc = "Field `PPWR` writer - Port power"]
pub type PPWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTCTL` reader - Port test control"]
pub type PTCTL_R = crate::FieldReader;
#[doc = "Field `PTCTL` writer - Port test control"]
pub type PTCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PSPD` reader - Port speed"]
pub type PSPD_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn pcsts(&self) -> PCSTS_R {
        PCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcdet(&self) -> PCDET_R {
        PCDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pena(&self) -> PENA_R {
        PENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(&self) -> PENCHNG_R {
        PENCHNG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port overcurrent active"]
    #[inline(always)]
    pub fn poca(&self) -> POCA_R {
        POCA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(&self) -> POCCHNG_R {
        POCCHNG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psusp(&self) -> PSUSP_R {
        PSUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&self) -> PRST_R {
        PRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn plsts(&self) -> PLSTS_R {
        PLSTS_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn ppwr(&self) -> PPWR_R {
        PPWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn ptctl(&self) -> PTCTL_R {
        PTCTL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn pspd(&self) -> PSPD_R {
        PSPD_R::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FS_HPRT")
            .field("pcsts", &self.pcsts())
            .field("pcdet", &self.pcdet())
            .field("pena", &self.pena())
            .field("penchng", &self.penchng())
            .field("poca", &self.poca())
            .field("pocchng", &self.pocchng())
            .field("pres", &self.pres())
            .field("psusp", &self.psusp())
            .field("prst", &self.prst())
            .field("plsts", &self.plsts())
            .field("ppwr", &self.ppwr())
            .field("ptctl", &self.ptctl())
            .field("pspd", &self.pspd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn pcdet(&mut self) -> PCDET_W<FS_HPRTrs> {
        PCDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn pena(&mut self) -> PENA_W<FS_HPRTrs> {
        PENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn penchng(&mut self) -> PENCHNG_W<FS_HPRTrs> {
        PENCHNG_W::new(self, 3)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn pocchng(&mut self) -> POCCHNG_W<FS_HPRTrs> {
        POCCHNG_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<FS_HPRTrs> {
        PRES_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn psusp(&mut self) -> PSUSP_W<FS_HPRTrs> {
        PSUSP_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prst(&mut self) -> PRST_W<FS_HPRTrs> {
        PRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn ppwr(&mut self) -> PPWR_W<FS_HPRTrs> {
        PPWR_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn ptctl(&mut self) -> PTCTL_W<FS_HPRTrs> {
        PTCTL_W::new(self, 13)
    }
}
#[doc = "OTG_FS host port control and status register (OTG_FS_HPRT)\n\nYou can [`read`](crate::Reg::read) this register and get [`fs_hprt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_hprt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FS_HPRTrs;
impl crate::RegisterSpec for FS_HPRTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_hprt::R`](R) reader structure"]
impl crate::Readable for FS_HPRTrs {}
#[doc = "`write(|w| ..)` method takes [`fs_hprt::W`](W) writer structure"]
impl crate::Writable for FS_HPRTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FS_HPRT to value 0"]
impl crate::Resettable for FS_HPRTrs {
    const RESET_VALUE: u32 = 0;
}
