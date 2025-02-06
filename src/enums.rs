//! Enums exposed in the API.

/// The Matrix Coefficients of the video used to derive luma and chroma values
/// from red, green, and blue color primaries.
///
/// For clarity, the value and meanings for `MatrixCoefficients` are adopted from
/// Table 4 of ISO/IEC 23001-8:2016 or ITU-T H.273.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatrixCoefficients {
    /// Unknown,
    Unknown,
    /// Identity.
    Identity,
    /// ITU-R BT.709.
    Bt709,
    /// US FCC 73.682.
    Fcc73682,
    /// ITU-R BT.470BG.
    Bt470bg,
    /// SMPTE 170M.
    Smpte170,
    /// SMPTE 240M.
    Smpte240,
    /// YCoCg.
    YCoCg,
    /// BT2020 Non-constant Luminance.
    Bt2020Ncl,
    /// BT2020 Constant Luminance.
    Bt2020Cl,
    /// SMPTE ST 2085.
    SmpteSt2085,
    /// Chroma-derived Non-constant Luminance.
    ChromaDerivedNcl,
    /// Chroma-derived Constant Luminance.
    ChromaDerivedCl,
    /// ITU-R BT.2100-0.
    Bt2100,
}

impl From<u64> for MatrixCoefficients {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Identity,
            1 => Self::Bt709,
            4 => Self::Fcc73682,
            5 => Self::Bt470bg,
            6 => Self::Smpte170,
            7 => Self::Smpte240,
            8 => Self::YCoCg,
            9 => Self::Bt2020Ncl,
            10 => Self::Bt2020Cl,
            11 => Self::SmpteSt2085,
            12 => Self::ChromaDerivedNcl,
            13 => Self::ChromaDerivedCl,
            14 => Self::Bt2100,
            _ => Self::Unknown,
        }
    }
}

/// How `DisplayWidth` & `DisplayHeight` are interpreted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DisplayUnit {
    /// In pixels.
    Pixels,
    /// In centimeters.
    Centimeters,
    /// In inches.
    Inches,
    /// By using the aspect ratio.
    DisplayAspectRatio,
    /// Unknown.
    Unknown,
}

impl From<u64> for DisplayUnit {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Pixels,
            1 => Self::Centimeters,
            2 => Self::Inches,
            3 => Self::DisplayAspectRatio,
            _ => Self::Unknown,
        }
    }
}

/// Specify the possible modifications to the aspect ratio.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AspectRatioType {
    /// Unknown.
    Unknown,
    /// Allow free resizing.
    FreeResizing,
    /// Keep the aspect ratio.
    KeepAspectRatio,
    /// Fixed size.
    Fixed,
}

impl From<u64> for AspectRatioType {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::FreeResizing,
            1 => Self::KeepAspectRatio,
            2 => Self::Fixed,
            _ => Self::Unknown,
        }
    }
}

/// Type of the track.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrackType {
    /// Unknown.
    Unknown,
    /// Video track.
    Video,
    /// Audio track.
    Audio,
    /// A complex track.
    Complex,
    /// A logo.
    Logo,
    /// Subtitles.
    Subtitle,
    /// Buttons.
    Buttons,
    /// Controls.
    Control,
    /// Metadata.
    Metadata,
}

impl From<u64> for TrackType {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::Video,
            2 => Self::Audio,
            3 => Self::Complex,
            16 => Self::Logo,
            17 => Self::Subtitle,
            18 => Self::Buttons,
            32 => Self::Control,
            33 => Self::Metadata,
            _ => Self::Unknown,
        }
    }
}

/// A flag to declare if the video is known to be progressive or interlaced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FlagInterlaced {
    /// Unknown.
    Unknown,
    /// Interlaced.
    Interlaced,
    /// Progressive.
    Progressive,
}

impl From<u64> for FlagInterlaced {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::Interlaced,
            2 => Self::Progressive,
            _ => Self::Unknown,
        }
    }
}

/// Declare the field ordering of the video.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FieldOrder {
    /// Unknown.
    Unknown,
    /// Progressive.
    Progressive,
    /// Top Field First.
    Tff,
    /// Bottom Field First.
    Bff,
    /// Top Field First (swapped).
    BffSwapped,
    /// Bottom Field First (swapped).
    TffSwapped,
}

impl From<u64> for FieldOrder {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Progressive,
            1 => Self::Tff,
            6 => Self::Bff,
            9 => Self::BffSwapped,
            14 => Self::TffSwapped,
            _ => Self::Unknown,
        }
    }
}

/// Stereo-3D video mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StereoMode {
    /// Unknown.
    Unknown,
    /// Mono.
    Mono,
    /// Side by side (left eye first).
    SideBySideLeftEyeFirst,
    /// Top - bottom (right eye is first).
    TopBottomRightEyeFirst,
    /// Top - bottom (left eye is first).
    TopBottomLeftEyeFirst,
    /// Checkboard (right eye is first).
    CheckboardRightEyeFirst,
    /// Checkboard (left eye is first).
    CheckboardLeftEyeFirst,
    /// Row interleaved (right eye is first).
    RowInterleavedRightEyeFirst,
    /// Row interleaved (left eye is first).
    RowInterleavedLeftEyeFirst,
    /// Column interleaved (right eye is first).
    ColumnInterleavedRightEyeFirst,
    /// Column interleaved (left eye is first).
    ColumnInterleavedLeftEyeFirst,
    /// Anaglyph (cyan/red).
    AnaglyphCyanRed,
    /// Side by side (right eye first).
    SideBySideRightEyeFirst,
    /// Anaglyph (green/magenta).
    AnaglyphGreenMagenta,
    /// Both eyes laced in one Block (left eye is first).
    LacedLeftEyeFirst,
    /// Both eyes laced in one Block (right eye is first).
    LacedRightEyeFirst,
}

impl From<u64> for StereoMode {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Mono,
            1 => Self::SideBySideLeftEyeFirst,
            2 => Self::TopBottomRightEyeFirst,
            3 => Self::TopBottomLeftEyeFirst,
            4 => Self::CheckboardRightEyeFirst,
            5 => Self::CheckboardLeftEyeFirst,
            6 => Self::RowInterleavedRightEyeFirst,
            7 => Self::RowInterleavedLeftEyeFirst,
            8 => Self::ColumnInterleavedRightEyeFirst,
            9 => Self::ColumnInterleavedLeftEyeFirst,
            10 => Self::AnaglyphCyanRed,
            11 => Self::SideBySideRightEyeFirst,
            12 => Self::AnaglyphGreenMagenta,
            13 => Self::LacedLeftEyeFirst,
            14 => Self::LacedRightEyeFirst,
            _ => Self::Unknown,
        }
    }
}

/// How chroma is sub sampled horizontally.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChromaSitingHorz {
    /// Unknown.
    Unknown,
    /// Left collocated.
    LeftCollated,
    /// Half.
    Half,
}

impl From<u64> for ChromaSitingHorz {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::LeftCollated,
            2 => Self::Half,
            _ => Self::Unknown,
        }
    }
}

/// How chroma is sub sampled vertically.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChromaSitingVert {
    /// Unknown.
    Unknown,
    /// Left collocated.
    LeftCollated,
    /// Half.
    Half,
}

impl From<u64> for ChromaSitingVert {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::LeftCollated,
            2 => Self::Half,
            _ => Self::Unknown,
        }
    }
}

/// Clipping of the color ranges.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Range {
    /// Unknown.
    Unknown,
    /// Broadcast range.
    Broadcast,
    /// Full range (no clipping).
    Full,
    /// Defined by MatrixCoefficients / TransferCharacteristics.
    Defined,
}

impl From<u64> for Range {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::Broadcast,
            2 => Self::Full,
            3 => Self::Defined,
            _ => Self::Unknown,
        }
    }
}

/// The transfer characteristics of the video.
///
/// For clarity, the value and meanings for `TransferCharacteristics` are adopted
/// from Table 3 of ISO/IEC 23091-4 or ITU-T H.273.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransferCharacteristics {
    /// Unknown.
    Unknown,
    /// ITU-R BT.709.
    Bt709,
    /// Gamma 2.2 curve - BT.470M.
    Bt407m,
    /// Gamma 2.8 curve - BT.470BG.
    Bt407bg,
    /// SMPTE 170M.
    Smpte170,
    /// SMPTE 240M.
    Smpte240,
    /// Linear.
    Linear,
    /// Log.
    Log,
    /// Log Sqrt,
    LogSqrt,
    /// IEC 61966-2-4.
    Iec61966_2_4,
    /// ITU-R BT.1361 Extended Colour Gamut.
    Bt1361,
    /// IEC 61966-2-1.
    Iec61966_2_1,
    /// ITU-R BT.2020 10 bit.
    Bt220_10,
    /// ITU-R BT.2020 12 bit.
    Bt220_12,
    /// ITU-R BT.2100 Perceptual Quantization.
    Bt2100,
    /// SMPTE ST 428-1.
    SmpteSt428_1,
    /// ARIB STD-B67 (HLG).
    Hlg,
}

impl From<u64> for TransferCharacteristics {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::Bt709,
            4 => Self::Bt407m,
            5 => Self::Bt407bg,
            6 => Self::Smpte170,
            7 => Self::Smpte240,
            8 => Self::Linear,
            9 => Self::Log,
            10 => Self::LogSqrt,
            11 => Self::Iec61966_2_4,
            12 => Self::Bt1361,
            13 => Self::Iec61966_2_1,
            14 => Self::Bt220_10,
            15 => Self::Bt220_12,
            16 => Self::Bt2100,
            17 => Self::SmpteSt428_1,
            18 => Self::Hlg,
            _ => Self::Unknown,
        }
    }
}

/// The colour primaries of the video.
///
/// For clarity, the value and meanings for `Primaries` are adopted
/// from Table 2 of ISO/IEC 23091-4 or ITU-T H.273.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primaries {
    /// Unknown.
    Unknown,
    /// ITU-R BT.709.
    Bt709,
    /// ITU-R BT.470M.
    Bt470m,
    /// ITU-R BT.470BG - BT.601 625.
    Bt601,
    /// ITU-R BT.601 525 - SMPTE 170M.
    Smpte170,
    /// SMPTE 240M.
    Smpte240,
    /// FILM.
    Film,
    /// ITU-R BT.2020.
    Bt2020,
    /// SMPTE ST 428-1.
    SmpteSt428_1,
    /// SMPTE RP 432-2.
    SmpteRp432_2,
    /// SMPTE EG 432-2.
    SmpteEg432_2,
    /// EBU Tech. 3213-E - JEDEC P22 phosphors.
    JedecP22,
}

impl From<u64> for Primaries {
    fn from(d: u64) -> Self {
        match d {
            1 => Self::Bt709,
            4 => Self::Bt470m,
            5 => Self::Bt601,
            6 => Self::Smpte170,
            7 => Self::Smpte240,
            8 => Self::Film,
            9 => Self::Bt2020,
            10 => Self::SmpteSt428_1,
            11 => Self::SmpteRp432_2,
            12 => Self::SmpteEg432_2,
            22 => Self::JedecP22,
            _ => Self::Unknown,
        }
    }
}

/// Describing what kind of transformation is applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentEncodingType {
    /// Unknown.
    Unknown,
    /// Transformation is a compression.
    Compression,
    /// Transformation is a encryption.
    Encryption,
}

impl From<u64> for ContentEncodingType {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Compression,
            1 => Self::Encryption,
            _ => Self::Unknown,
        }
    }
}

/// The encryption algorithm used.
///
/// `NotEncrypted` means that the contents have not been encrypted but only signed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentEncAlgo {
    /// Unknown.
    Unknown,
    /// Not encrypted.
    NotEncrypted,
    /// DES - FIPS 46-3.
    Des,
    /// Triple DES - RFC 1851.
    TripleDes,
    /// Twofish.
    Twofish,
    /// Blowfish.
    Blowfish,
    /// AES - FIPS 187.
    Aes,
}

impl From<u64> for ContentEncAlgo {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::NotEncrypted,
            1 => Self::Des,
            2 => Self::TripleDes,
            3 => Self::Twofish,
            4 => Self::Blowfish,
            5 => Self::Aes,
            _ => Self::Unknown,
        }
    }
}

/// The AES cipher mode used in the encryption.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AesSettingsCipherMode {
    /// Unknown.
    Unknown,
    /// AES-CTR / Counter, NIST SP 800-38A.
    Ctr,
    /// AES-CBC / Cipher Block Chaining, NIST SP 800-38A.
    Cbc,
}

impl From<u64> for AesSettingsCipherMode {
    fn from(d: u64) -> Self {
        match d {
            0 => Self::Ctr,
            1 => Self::Cbc,
            _ => Self::Unknown,
        }
    }
}
