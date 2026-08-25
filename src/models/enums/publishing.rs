//! Enums for the serialized facet, and the album/podcast/periodical basic types.

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "installment_unit", rename_all = "snake_case")]
pub enum InstallmentUnit {
    Episode,
    Chapter,
    Issue,
    Volume,
    Track,
    Part,
    Session,
    Act,
    Strip,
    Entry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "release_schedule", rename_all = "snake_case")]
pub enum ReleaseSchedule {
    Daily,
    Weekdays,
    Weekly,
    Biweekly,
    Monthly,
    Bimonthly,
    Quarterly,
    Seasonal,
    Annual,
    Irregular,
    Burst,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "album_type", rename_all = "snake_case")]
pub enum AlbumType {
    Studio,
    Live,
    Compilation,
    Ep,
    Single,
    Soundtrack,
    Mixtape,
    Remix,
    Demo,
    Bootleg,
    Split,
    BoxSet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "podcast_type", rename_all = "snake_case")]
pub enum PodcastType {
    Interview,
    Narrative,
    AudioDrama,
    News,
    Educational,
    Panel,
    Solo,
    Variety,
    Rebroadcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "periodical_type", rename_all = "snake_case")]
pub enum PeriodicalType {
    Magazine,
    Journal,
    Newspaper,
    Zine,
    Newsletter,
    AnthologyMagazine,
    TradePublication,
    ComicMagazine,
}
