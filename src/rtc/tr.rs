#[doc = "Register `TR` reader"]
pub type R = crate::R<TRrs>;
#[doc = "Register `TR` writer"]
pub type W = crate::W<TRrs>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HU` reader - Hour units in BCD format"]
pub type HU_R = crate::FieldReader;
#[doc = "Field `HU` writer - Hour units in BCD format"]
pub type HU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HT` reader - Hour tens in BCD format"]
pub type HT_R = crate::FieldReader;
#[doc = "Field `HT` writer - Hour tens in BCD format"]
pub type HT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PM_R = crate::BitReader;
#[doc = "Field `PM` writer - AM/PM notation"]
pub type PM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TR")
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<TRrs> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<TRrs> {
        ST_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<TRrs> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<TRrs> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<TRrs> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<TRrs> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<TRrs> {
        PM_W::new(self, 22)
    }
}
#[doc = "time register\n\nYou can [`read`](crate::Reg::read) this register and get [`tr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRrs;
impl crate::RegisterSpec for TRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr::R`](R) reader structure"]
impl crate::Readable for TRrs {}
#[doc = "`write(|w| ..)` method takes [`tr::W`](W) writer structure"]
impl crate::Writable for TRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TRrs {
    const RESET_VALUE: u32 = 0;
}
