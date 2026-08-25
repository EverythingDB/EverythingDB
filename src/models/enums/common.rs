//! Enums shared across many facets and tiers: naming, media satellites,
//! relationships, credits, and the facet/platform registries.

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "date_precision", rename_all = "snake_case")]
pub enum DatePrecision {
    Day,
    Month,
    Season,
    Year,
    Decade,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "media_status", rename_all = "snake_case")]
pub enum MediaStatus {
    Announced,
    InProduction,
    Releasing,
    OnHiatus,
    Completed,
    Cancelled,
    Lost,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "source_material", rename_all = "snake_case")]
pub enum SourceMaterial {
    Original,
    Novel,
    LightNovel,
    WebNovel,
    Manga,
    Comic,
    Game,
    VisualNovel,
    Tabletop,
    Film,
    Show,
    Music,
    Folklore,
    ReligiousText,
    Historical,
    Biography,
    News,
    Other,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "title_type", rename_all = "snake_case")]
pub enum TitleType {
    Primary,
    Native,
    Romanized,
    English,
    Localized,
    Alternative,
    Abbreviation,
    Working,
    Translated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "image_type", rename_all = "snake_case")]
pub enum ImageType {
    Cover,
    Poster,
    Banner,
    Backdrop,
    Logo,
    Thumbnail,
    Screenshot,
    Still,
    CharacterArt,
    ConceptArt,
    Promotional,
    Spine,
    BackCover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "relation_type", rename_all = "snake_case")]
pub enum RelationType {
    Sequel,
    Prequel,
    SideStory,
    ParentStory,
    Adaptation,
    AdaptedFrom,
    AlternativeVersion,
    AlternativeSetting,
    SpinOff,
    Summary,
    FullStory,
    CharacterShared,
    SettingShared,
    Remake,
    Remaster,
    Port,
    Localization,
    Abridgement,
    DerivedFrom,
    Contains,
    ContainedIn,
    SoundtrackOf,
    HasSoundtrack,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "collection_type", rename_all = "snake_case")]
pub enum CollectionType {
    Franchise,
    Series,
    SharedUniverse,
    Trilogy,
    BoxSet,
    Discography,
    Anthology,
    Arc,
    CrossoverEvent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "credit_department", rename_all = "snake_case")]
pub enum CreditDepartment {
    Direction,
    Writing,
    Art,
    Animation,
    Production,
    Performance,
    Voice,
    Music,
    Sound,
    Photography,
    Editing,
    Design,
    Engineering,
    Translation,
    Publishing,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "character_billing", rename_all = "snake_case")]
pub enum CharacterBilling {
    Main,
    Supporting,
    Recurring,
    Minor,
    Background,
    Cameo,
    Antagonist,
    Narrator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "facet_tier", rename_all = "snake_case")]
pub enum FacetTier {
    Property,
    Basic,
    Composite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "platform_kind", rename_all = "snake_case")]
pub enum PlatformKind {
    Console,
    Handheld,
    Arcade,
    Computer,
    Mobile,
    Browser,
    Vr,
    StreamingService,
    BroadcastNetwork,
    PrintChannel,
    Tabletop,
    Other,
}
