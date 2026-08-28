//! Enums for the audiovisual, animation, audio and musical facets, the
//! `show` basic type, and the audiobook/music_video composite types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "color_system", rename_all = "snake_case")]
pub enum ColorSystem {
    BlackAndWhite,
    Color,
    Colorized,
    Tinted,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "capture_medium", rename_all = "snake_case")]
pub enum CaptureMedium {
    #[sqlx(rename = "film_8mm")]
    Film8mm,
    #[sqlx(rename = "film_16mm")]
    Film16mm,
    #[sqlx(rename = "film_35mm")]
    Film35mm,
    #[sqlx(rename = "film_65mm")]
    Film65mm,
    Imax,
    Videotape,
    Digital,
    Virtual,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "animation_technique", rename_all = "snake_case")]
pub enum AnimationTechnique {
    TraditionalCel,
    #[sqlx(rename = "digital_2d")]
    Digital2d,
    #[sqlx(rename = "cgi_3d")]
    Cgi3d,
    StopMotion,
    Claymation,
    Puppet,
    Cutout,
    Rotoscope,
    PixelArt,
    MotionGraphics,
    Sand,
    PaintOnGlass,
    LiveHybrid,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, Default)]
#[sqlx(type_name = "recording_type", rename_all = "snake_case")]
pub enum RecordingType {
    Studio,
    Live,
    Field,
    Remote,
    Synthetic,
    Archival,
    #[default]
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "vocal_type", rename_all = "snake_case")]
pub enum VocalType {
    Lead,
    Duet,
    Group,
    Choral,
    SpokenWord,
    Rap,
    Instrumental,
    Vocaloid,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "show_type", rename_all = "snake_case")]
pub enum ShowType {
    Tv,
    Ona,
    Ova,
    Web,
    Miniseries,
    Special,
    TvMovie,
    Pilot,
    AnthologySeries,
    Variety,
    DocumentarySeries,
    Reality,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "voiced_extent", rename_all = "snake_case")]
pub enum VoicedExtent {
    None,
    Partial,
    ProtagonistExcluded,
    Full,
    FullIncludingProtagonist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize, Default)]
#[sqlx(type_name = "narration_style", rename_all = "snake_case")]
pub enum NarrationStyle {
    #[default]
    SingleNarrator,
    DualNarrator,
    FullCast,
    AuthorRead,
    Dramatized,
    Synthetic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "music_video_type", rename_all = "snake_case")]
pub enum MusicVideoType {
    Official,
    Lyric,
    Performance,
    Live,
    Animated,
    Concept,
    FanMade,
    Teaser,
}
