//! Enums for the performance and still_image facets, and the
//! stage_production/artwork/tabletop_game basic and composite types.

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "staging_type", rename_all = "snake_case")]
pub enum StagingType {
    Proscenium,
    Thrust,
    InTheRound,
    BlackBox,
    Immersive,
    SiteSpecific,
    Street,
    Arena,
    Stadium,
    Broadcast,
    Virtual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "visual_medium", rename_all = "snake_case")]
pub enum VisualMedium {
    Oil,
    Acrylic,
    Watercolor,
    Gouache,
    Ink,
    Pencil,
    Charcoal,
    Pastel,
    Digital,
    Photograph,
    Screenprint,
    Woodblock,
    Etching,
    Lithograph,
    Collage,
    MixedMedia,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "production_type", rename_all = "snake_case")]
pub enum ProductionType {
    Play,
    Musical,
    Opera,
    Operetta,
    Ballet,
    Dance,
    Concert,
    ConcertTour,
    Standup,
    Improv,
    Circus,
    Puppetry,
    PerformanceArt,
    Recital,
    Pantomime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "artwork_type", rename_all = "snake_case")]
pub enum ArtworkType {
    Painting,
    Illustration,
    Photograph,
    Poster,
    CoverArt,
    ConceptArt,
    CharacterSheet,
    Storyboard,
    ComicPage,
    DigitalArt,
    Sculpture,
    Installation,
    Print,
    Sketch,
    Infographic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "tabletop_type", rename_all = "snake_case")]
pub enum TabletopType {
    Board,
    Card,
    CollectibleCard,
    Ttrpg,
    Wargame,
    Miniatures,
    Party,
    Dexterity,
    Puzzle,
    EscapeRoom,
    PrintAndPlay,
    Legacy,
}
