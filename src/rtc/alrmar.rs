#[doc = "Register `ALRMAR` reader"]
pub type R = crate::R<ALRMARrs>;
#[doc = "Register `ALRMAR` writer"]
pub type W = crate::W<ALRMARrs>;
#[doc = "Field `SU` reader - Second units in BCD format"]
pub type SU_R = crate::FieldReader;
#[doc = "Field `SU` writer - Second units in BCD format"]
pub type SU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ST` reader - Second tens in BCD format"]
pub type ST_R = crate::FieldReader;
#[doc = "Field `ST` writer - Second tens in BCD format"]
pub type ST_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK1` reader - Alarm A seconds mask"]
pub type MSK1_R = crate::BitReader;
#[doc = "Field `MSK1` writer - Alarm A seconds mask"]
pub type MSK1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNU` reader - Minute units in BCD format"]
pub type MNU_R = crate::FieldReader;
#[doc = "Field `MNU` writer - Minute units in BCD format"]
pub type MNU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MNT` reader - Minute tens in BCD format"]
pub type MNT_R = crate::FieldReader;
#[doc = "Field `MNT` writer - Minute tens in BCD format"]
pub type MNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSK2` reader - Alarm A minutes mask"]
pub type MSK2_R = crate::BitReader;
#[doc = "Field `MSK2` writer - Alarm A minutes mask"]
pub type MSK2_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `MSK3` reader - Alarm A hours mask"]
pub type MSK3_R = crate::BitReader;
#[doc = "Field `MSK3` writer - Alarm A hours mask"]
pub type MSK3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Date units or day in BCD format"]
pub type DU_R = crate::FieldReader;
#[doc = "Field `DU` writer - Date units or day in BCD format"]
pub type DU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DT_R = crate::FieldReader;
#[doc = "Field `DT` writer - Date tens in BCD format"]
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WDSEL` reader - Week day selection"]
pub type WDSEL_R = crate::BitReader;
#[doc = "Field `WDSEL` writer - Week day selection"]
pub type WDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSK4` reader - Alarm A date mask"]
pub type MSK4_R = crate::BitReader;
#[doc = "Field `MSK4` writer - Alarm A date mask"]
pub type MSK4_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    pub fn msk1(&self) -> MSK1_R {
        MSK1_R::new(((self.bits >> 7) & 1) != 0)
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
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    pub fn msk2(&self) -> MSK2_R {
        MSK2_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    pub fn msk3(&self) -> MSK3_R {
        MSK3_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&self) -> WDSEL_R {
        WDSEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    pub fn msk4(&self) -> MSK4_R {
        MSK4_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ALRMAR")
            .field("msk4", &self.msk4())
            .field("wdsel", &self.wdsel())
            .field("dt", &self.dt())
            .field("du", &self.du())
            .field("msk3", &self.msk3())
            .field("pm", &self.pm())
            .field("ht", &self.ht())
            .field("hu", &self.hu())
            .field("msk2", &self.msk2())
            .field("mnt", &self.mnt())
            .field("mnu", &self.mnu())
            .field("msk1", &self.msk1())
            .field("st", &self.st())
            .field("su", &self.su())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Second units in BCD format"]
    #[inline(always)]
    pub fn su(&mut self) -> SU_W<ALRMARrs> {
        SU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format"]
    #[inline(always)]
    pub fn st(&mut self) -> ST_W<ALRMARrs> {
        ST_W::new(self, 4)
    }
    #[doc = "Bit 7 - Alarm A seconds mask"]
    #[inline(always)]
    pub fn msk1(&mut self) -> MSK1_W<ALRMARrs> {
        MSK1_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format"]
    #[inline(always)]
    pub fn mnu(&mut self) -> MNU_W<ALRMARrs> {
        MNU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format"]
    #[inline(always)]
    pub fn mnt(&mut self) -> MNT_W<ALRMARrs> {
        MNT_W::new(self, 12)
    }
    #[doc = "Bit 15 - Alarm A minutes mask"]
    #[inline(always)]
    pub fn msk2(&mut self) -> MSK2_W<ALRMARrs> {
        MSK2_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format"]
    #[inline(always)]
    pub fn hu(&mut self) -> HU_W<ALRMARrs> {
        HU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W<ALRMARrs> {
        HT_W::new(self, 20)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&mut self) -> PM_W<ALRMARrs> {
        PM_W::new(self, 22)
    }
    #[doc = "Bit 23 - Alarm A hours mask"]
    #[inline(always)]
    pub fn msk3(&mut self) -> MSK3_W<ALRMARrs> {
        MSK3_W::new(self, 23)
    }
    #[doc = "Bits 24:27 - Date units or day in BCD format"]
    #[inline(always)]
    pub fn du(&mut self) -> DU_W<ALRMARrs> {
        DU_W::new(self, 24)
    }
    #[doc = "Bits 28:29 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<ALRMARrs> {
        DT_W::new(self, 28)
    }
    #[doc = "Bit 30 - Week day selection"]
    #[inline(always)]
    pub fn wdsel(&mut self) -> WDSEL_W<ALRMARrs> {
        WDSEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - Alarm A date mask"]
    #[inline(always)]
    pub fn msk4(&mut self) -> MSK4_W<ALRMARrs> {
        MSK4_W::new(self, 31)
    }
}
#[doc = "alarm A register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMARrs;
impl crate::RegisterSpec for ALRMARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmar::R`](R) reader structure"]
impl crate::Readable for ALRMARrs {}
#[doc = "`write(|w| ..)` method takes [`alrmar::W`](W) writer structure"]
impl crate::Writable for ALRMARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRMAR to value 0"]
impl crate::Resettable for ALRMARrs {
    const RESET_VALUE: u32 = 0;
}
