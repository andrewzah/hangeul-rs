use crate::constants::*;
use crate::is_syllable;

pub trait Syllable {
    fn composable_u32(&self) -> u32;
    fn to_u32(&self) -> u32;
    fn to_index(&self) -> u32;
}

#[derive(Debug, PartialEq)]
pub enum Choseong {
    Giyeok,
    SsangGiyeok,
    Nieun,
    Digeut,
    SsangDigeut,
    Rieul,
    Mieum,
    Bieup,
    SsangBieup,
    Siot,
    SsangSiot,
    Ieung,
    Jieut,
    SsangJieut,
    Chieut,
    Kiyeok,
    Tieut,
    Pieup,
    Hieuh,
}

/// `초성` -> Initial Sound, aka Initial or Lead Character
impl Choseong {
    pub fn from_char(c: &char) -> Option<Choseong> {
        Choseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Choseong> {
        match is_syllable(code) {
            true => Choseong::from_syllable(code),
            false => Choseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Choseong> {
        let value = (code - HANGEUL_OFFSET) / CHOSEONG_COUNT;

        Self::from_jamo(value + CHOSEONG_START)
    }

    pub fn from_jamo(code: u32) -> Option<Choseong> {
        match code {
            0x1100 | 0x3131 => Some(Choseong::Giyeok),
            0x1101 | 0x3132 => Some(Choseong::SsangGiyeok),
            0x1102 | 0x3134 => Some(Choseong::Nieun),
            0x1103 | 0x3137 => Some(Choseong::Digeut),
            0x1104 | 0x3138 => Some(Choseong::SsangDigeut),
            0x1105 | 0x3139 => Some(Choseong::Rieul),
            0x1106 | 0x3141 => Some(Choseong::Mieum),
            0x1107 | 0x3142 => Some(Choseong::Bieup),
            0x1108 | 0x3143 => Some(Choseong::SsangBieup),
            0x1109 | 0x3145 => Some(Choseong::Siot),
            0x110A | 0x3146 => Some(Choseong::SsangSiot),
            0x110B | 0x3147 => Some(Choseong::Ieung),
            0x110C | 0x3148 => Some(Choseong::Jieut),
            0x110D | 0x3149 => Some(Choseong::SsangJieut),
            0x110E | 0x314A => Some(Choseong::Chieut),
            0x110F | 0x314B => Some(Choseong::Kiyeok),
            0x1110 | 0x314C => Some(Choseong::Tieut),
            0x1111 | 0x314D => Some(Choseong::Pieup),
            0x1112 | 0x314E => Some(Choseong::Hieuh),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Choseong::Giyeok => 'ㄱ',
            Choseong::SsangGiyeok => 'ㄲ',
            Choseong::Nieun => 'ㄴ',
            Choseong::Digeut => 'ㄷ',
            Choseong::SsangDigeut => 'ㄸ',
            Choseong::Rieul => 'ㄹ',
            Choseong::Mieum => 'ㅁ',
            Choseong::Bieup => 'ㅂ',
            Choseong::SsangBieup => 'ㅃ',
            Choseong::Siot => 'ㅅ',
            Choseong::SsangSiot => 'ㅆ',
            Choseong::Ieung => 'ㅇ',
            Choseong::Jieut => 'ㅈ',
            Choseong::SsangJieut => 'ㅉ',
            Choseong::Chieut => 'ㅊ',
            Choseong::Kiyeok => 'ㅋ',
            Choseong::Tieut => 'ㅌ',
            Choseong::Pieup => 'ㅍ',
            Choseong::Hieuh => 'ㅎ',
        }
    }
}

impl Syllable for Choseong {
    fn to_u32(&self) -> u32 {
        Self::to_index(self) + CHOSEONG_START
    }

    fn composable_u32(&self) -> u32 {
        Self::to_index(self) * CHOSEONG_COUNT
    }

    fn to_index(&self) -> u32 {
        match self {
            Choseong::Giyeok => 0,
            Choseong::SsangGiyeok => 1,
            Choseong::Nieun => 2,
            Choseong::Digeut => 3,
            Choseong::SsangDigeut => 4,
            Choseong::Rieul => 5,
            Choseong::Mieum => 6,
            Choseong::Bieup => 7,
            Choseong::SsangBieup => 8,
            Choseong::Siot => 9,
            Choseong::SsangSiot => 10,
            Choseong::Ieung => 11,
            Choseong::Jieut => 12,
            Choseong::SsangJieut => 13,
            Choseong::Chieut => 14,
            Choseong::Kiyeok => 15,
            Choseong::Tieut => 16,
            Choseong::Pieup => 17,
            Choseong::Hieuh => 18,
        }
    }
}

/// `중성` -> Middle Sound (always a vowel)
#[derive(Debug, PartialEq)]
pub enum Jungseong {
    A,   // ㅏ
    AE,  // ㅐ
    YA,  // ㅑ
    YAE, // ㅒ
    EO,  // ㅓ
    E,   // ㅔ
    YEO, // ㅕ
    YE,  // ㅖ
    O,   // ㅗ
    WA,  // ㅘ
    WAE, // ㅙ
    OE,  // ㅚ
    YO,  // ㅛ
    U,   // ㅜ
    WEO, // ㅝ
    WE,  // ㅞ
    WI,  // ㅟ
    YU,  // ㅠ
    EU,  // ㅡ
    YI,  // ㅢ
    I,   // ㅣ
}

impl Jungseong {
    pub fn from_char(c: &char) -> Option<Jungseong> {
        Jungseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Jungseong> {
        match is_syllable(code) {
            true => Jungseong::from_syllable(code),
            false => Jungseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Jungseong> {
        let jongseong_code = (code - HANGEUL_OFFSET) % JUNGSEONG_COUNT;
        let value = ((code - HANGEUL_OFFSET - jongseong_code) % CHOSEONG_COUNT) / JUNGSEONG_COUNT;

        Self::from_jamo(value + JUNGSEONG_START)
    }

    pub fn from_jamo(code: u32) -> Option<Jungseong> {
        match code {
            0x1161 | 0x314F => Some(Jungseong::A),
            0x1162 | 0x3150 => Some(Jungseong::AE),
            0x1163 | 0x3151 => Some(Jungseong::YA),
            0x1164 | 0x3152 => Some(Jungseong::YAE),
            0x1165 | 0x3153 => Some(Jungseong::EO),
            0x1166 | 0x3154 => Some(Jungseong::E),
            0x1167 | 0x3155 => Some(Jungseong::YEO),
            0x1168 | 0x3156 => Some(Jungseong::YE),
            0x1169 | 0x3157 => Some(Jungseong::O),
            0x116A | 0x3158 => Some(Jungseong::WA),
            0x116B | 0x3159 => Some(Jungseong::WAE),
            0x116C | 0x315A => Some(Jungseong::OE),
            0x116D | 0x315B => Some(Jungseong::YO),
            0x116E | 0x315C => Some(Jungseong::U),
            0x116F | 0x315D => Some(Jungseong::WEO),
            0x1170 | 0x315E => Some(Jungseong::WE),
            0x1171 | 0x315F => Some(Jungseong::WI),
            0x1172 | 0x3160 => Some(Jungseong::YU),
            0x1173 | 0x3161 => Some(Jungseong::EU),
            0x1174 | 0x3162 => Some(Jungseong::YI),
            0x1175 | 0x3163 => Some(Jungseong::I),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Jungseong::A => 'ㅏ',
            Jungseong::AE => 'ㅐ',
            Jungseong::YA => 'ㅑ',
            Jungseong::YAE => 'ㅒ',
            Jungseong::EO => 'ㅓ',
            Jungseong::E => 'ㅔ',
            Jungseong::YEO => 'ㅕ',
            Jungseong::YE => 'ㅖ',
            Jungseong::O => 'ㅗ',
            Jungseong::WA => 'ㅘ',
            Jungseong::WAE => 'ㅙ',
            Jungseong::OE => 'ㅚ',
            Jungseong::YO => 'ㅛ',
            Jungseong::U => 'ㅜ',
            Jungseong::WEO => 'ㅝ',
            Jungseong::WE => 'ㅞ',
            Jungseong::WI => 'ㅟ',
            Jungseong::YU => 'ㅠ',
            Jungseong::EU => 'ㅡ',
            Jungseong::YI => 'ㅢ',
            Jungseong::I => 'ㅣ',
        }
    }
}

impl Syllable for Jungseong {
    fn to_u32(&self) -> u32 {
        Self::to_index(self) + JUNGSEONG_START
    }

    fn composable_u32(&self) -> u32 {
        Self::to_index(self) * JUNGSEONG_COUNT
    }

    fn to_index(&self) -> u32 {
        match self {
            Jungseong::A => 0,
            Jungseong::AE => 1,
            Jungseong::YA => 2,
            Jungseong::YAE => 3,
            Jungseong::EO => 4,
            Jungseong::E => 5,
            Jungseong::YEO => 6,
            Jungseong::YE => 7,
            Jungseong::O => 8,
            Jungseong::WA => 9,
            Jungseong::WAE => 10,
            Jungseong::OE => 11,
            Jungseong::YO => 12,
            Jungseong::U => 13,
            Jungseong::WEO => 14,
            Jungseong::WE => 15,
            Jungseong::WI => 16,
            Jungseong::YU => 17,
            Jungseong::EU => 18,
            Jungseong::YI => 19,
            Jungseong::I => 20,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Jongseong {
    Giyeok,
    SsangGiyeok,
    GiyeokSiot,
    Nieun,
    NieunJieut,
    NieunHieuh,
    Digeut,
    Rieul,
    RieulGiyeok,
    RieulMieum,
    RieulBieup,
    RieulSiot,
    RieulTieut,
    RieulPieup,
    RieulHieuh,
    Mieum,
    Bieup,
    BieupSiot,
    Siot,
    SsangSiot,
    Ieung,
    Jieut,
    Chieut,
    Kieuk,
    Tieut,
    Pieup,
    Hieuh,
}

impl Jongseong {
    pub fn from_char(c: &char) -> Option<Jongseong> {
        Jongseong::from_u32(*c as u32)
    }

    pub fn from_u32(code: u32) -> Option<Jongseong> {
        match is_syllable(code) {
            true => Jongseong::from_syllable(code),
            false => Jongseong::from_jamo(code),
        }
    }

    pub fn from_syllable(code: u32) -> Option<Jongseong> {
        let value = (code - HANGEUL_OFFSET) % JUNGSEONG_COUNT;

        Self::from_jamo(value + JONGSEONG_START - 1)
    }

    pub fn from_jamo(code: u32) -> Option<Jongseong> {
        match code {
            0x11A8 | 0x3131 => Some(Jongseong::Giyeok),
            0x11A9 | 0x3132 => Some(Jongseong::SsangGiyeok),
            0x11AA | 0x3133 => Some(Jongseong::GiyeokSiot),
            0x11AB | 0x3134 => Some(Jongseong::Nieun),
            0x11AC | 0x3135 => Some(Jongseong::NieunJieut),
            0x11AD | 0x3136 => Some(Jongseong::NieunHieuh),
            0x11AE | 0x3137 => Some(Jongseong::Digeut),
            0x11AF | 0x3139 => Some(Jongseong::Rieul),
            0x11B0 | 0x313A => Some(Jongseong::RieulGiyeok),
            0x11B1 | 0x313B => Some(Jongseong::RieulMieum),
            0x11B2 | 0x313C => Some(Jongseong::RieulBieup),
            0x11B3 | 0x313D => Some(Jongseong::RieulSiot),
            0x11B4 | 0x313E => Some(Jongseong::RieulTieut),
            0x11B5 | 0x313F => Some(Jongseong::RieulPieup),
            0x11B6 | 0x3140 => Some(Jongseong::RieulHieuh),
            0x11B7 | 0x3141 => Some(Jongseong::Mieum),
            0x11B8 | 0x3142 => Some(Jongseong::Bieup),
            0x11B9 | 0x3144 => Some(Jongseong::BieupSiot),
            0x11BA | 0x3145 => Some(Jongseong::Siot),
            0x11BB | 0x3146 => Some(Jongseong::SsangSiot),
            0x11BC | 0x3147 => Some(Jongseong::Ieung),
            0x11BD | 0x3148 => Some(Jongseong::Jieut),
            0x11BE | 0x314A => Some(Jongseong::Chieut),
            0x11BF | 0x314B => Some(Jongseong::Kieuk),
            0x11C0 | 0x314C => Some(Jongseong::Tieut),
            0x11C1 | 0x314D => Some(Jongseong::Pieup),
            0x11C2 | 0x314E => Some(Jongseong::Hieuh),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Jongseong::Giyeok => 'ㄱ',
            Jongseong::SsangGiyeok => 'ㄲ',
            Jongseong::GiyeokSiot => 'ㄳ',
            Jongseong::Nieun => 'ㄴ',
            Jongseong::NieunJieut => 'ㄵ',
            Jongseong::NieunHieuh => 'ㄶ',
            Jongseong::Digeut => 'ㄷ',
            Jongseong::Rieul => 'ㄹ',
            Jongseong::RieulGiyeok => 'ㄺ',
            Jongseong::RieulMieum => 'ㄻ',
            Jongseong::RieulBieup => 'ㄼ',
            Jongseong::RieulSiot => 'ㄽ',
            Jongseong::RieulTieut => 'ㄾ',
            Jongseong::RieulPieup => 'ㄿ',
            Jongseong::RieulHieuh => 'ㅀ',
            Jongseong::Mieum => 'ㅁ',
            Jongseong::Bieup => 'ㅂ',
            Jongseong::BieupSiot => 'ㅄ',
            Jongseong::Siot => 'ㅅ',
            Jongseong::SsangSiot => 'ㅆ',
            Jongseong::Ieung => 'ㅇ',
            Jongseong::Jieut => 'ㅈ',
            Jongseong::Chieut => 'ㅊ',
            Jongseong::Kieuk => 'ㅋ',
            Jongseong::Tieut => 'ㅌ',
            Jongseong::Pieup => 'ㅍ',
            Jongseong::Hieuh => 'ㅎ',
        }
    }
}

impl Syllable for Jongseong {
    fn to_u32(&self) -> u32 {
        Self::to_index(self) + JONGSEONG_START
    }

    fn composable_u32(&self) -> u32 {
        Self::to_index(self)
    }

    fn to_index(&self) -> u32 {
        match self {
            Jongseong::Giyeok => 1,
            Jongseong::SsangGiyeok => 2,
            Jongseong::GiyeokSiot => 3,
            Jongseong::Nieun => 4,
            Jongseong::NieunJieut => 5,
            Jongseong::NieunHieuh => 6,
            Jongseong::Digeut => 7,
            Jongseong::Rieul => 8,
            Jongseong::RieulGiyeok => 9,
            Jongseong::RieulMieum => 10,
            Jongseong::RieulBieup => 11,
            Jongseong::RieulSiot => 12,
            Jongseong::RieulTieut => 13,
            Jongseong::RieulPieup => 14,
            Jongseong::RieulHieuh => 15,
            Jongseong::Mieum => 16,
            Jongseong::Bieup => 17,
            Jongseong::BieupSiot => 18,
            Jongseong::Siot => 19,
            Jongseong::SsangSiot => 20,
            Jongseong::Ieung => 21,
            Jongseong::Jieut => 22,
            Jongseong::Chieut => 23,
            Jongseong::Kieuk => 24,
            Jongseong::Tieut => 25,
            Jongseong::Pieup => 26,
            Jongseong::Hieuh => 27,
        }
    }
}
