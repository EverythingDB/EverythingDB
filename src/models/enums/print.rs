//! Enums for the narrative, print, sequential_art and publication facets,
//! plus the book/comic basic types built on top of them.

use std::default;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "narrative_form", rename_all = "snake_case")]
pub enum NarrativeForm {
    Linear,
    Nonlinear,
    Episodic,
    Anthology,
    Branching,
    Vignette,
    FrameStory,
    Experimental,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "point_of_view", rename_all = "snake_case")]
pub enum PointOfView {
    FirstPerson,
    SecondPerson,
    ThirdLimited,
    ThirdOmniscient,
    Multiple,
    Objective,
    Epistolary,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Default, Serialize, Deserialize)]
#[sqlx(type_name = "prose_format", rename_all = "snake_case")]
pub enum ProseFormat {
    #[default]
    Prose,
    Verse,
    Script,
    Screenplay,
    Epistolary,
    Diary,
    Reference,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Default, Serialize, Deserialize)]
#[sqlx(type_name = "reading_direction", rename_all = "snake_case")]
pub enum ReadingDirection {
    #[default]
    Ltr,
    Rtl,
    VerticalRtl,
    VerticalLtr,
    Boustrophedon,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "panel_layout", rename_all = "snake_case")]
pub enum PanelLayout {
    Page,
    DoublePage,
    VerticalScroll,
    HorizontalStrip,
    FourKoma,
    SinglePanel,
    Freeform,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "coloring_mode", rename_all = "snake_case")]
pub enum ColoringMode {
    Monochrome,
    Greyscale,
    Duotone,
    SpotColor,
    FullColor,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "publication_model", rename_all = "snake_case")]
pub enum PublicationModel {
    Traditional,
    SmallPress,
    Academic,
    SelfPublished,
    Vanity,
    WebSerial,
    FanPublished,
    Commissioned,
    Government,
    Unpublished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "distribution_format", rename_all = "snake_case")]
pub enum DistributionFormat {
    PrintOnly,
    DigitalOnly,
    PrintAndDigital,
    Broadcast,
    Streaming,
    PhysicalMedia,
    Download,
    Cartridge,
    Disc,
    Tape,
    LiveOnly,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "binding_format", rename_all = "snake_case")]
pub enum BindingFormat {
    Hardcover,
    TradePaperback,
    MassMarket,
    Tankobon,
    Bunko,
    Aizoban,
    Kanzenban,
    Omnibus,
    BoxSet,
    SaddleStitch,
    Spiral,
    Ebook,
    LooseLeaf,
    Scroll,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "audience_demographic", rename_all = "snake_case")]
pub enum AudienceDemographic {
    Children,
    MiddleGrade,
    YoungAdult,
    Shounen,
    Shoujo,
    Seinen,
    Josei,
    Kodomomuke,
    General,
    Adult,
    Academic,
    Professional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "book_type", rename_all = "snake_case")]
pub enum BookType {
    Novel,
    Novella,
    ShortStory,
    StoryCollection,
    Anthology,
    Poetry,
    EssayCollection,
    Memoir,
    Biography,
    Reference,
    Textbook,
    Manual,
    PictureBook,
    Artbook,
    Cookbook,
    Religious,
    AcademicMonograph,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "comic_format", rename_all = "snake_case")]
pub enum ComicFormat {
    SingleIssue,
    CollectedVolume,
    OneShot,
    GraphicNovel,
    Strip,
    WebSeries,
    AnthologyContribution,
    MiniSeries,
}
