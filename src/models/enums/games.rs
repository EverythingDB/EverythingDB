//! Enums for the interactive and software facets, and the `game` basic type.

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "input_method", rename_all = "snake_case")]
pub enum InputMethod {
    Keyboard,
    Mouse,
    Gamepad,
    Touch,
    Stylus,
    Motion,
    Vr,
    LightGun,
    ArcadeStick,
    DancePad,
    Voice,
    EyeTracking,
    PhysicalComponent,
    Dice,
    Cards,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "save_system", rename_all = "snake_case")]
pub enum SaveSystem {
    None,
    Password,
    Checkpoint,
    ManualSlot,
    Autosave,
    SaveAnywhere,
    Permadeath,
    CloudOnly,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "branching_structure", rename_all = "snake_case")]
pub enum BranchingStructure {
    None,
    Linear,
    Hub,
    Branching,
    OpenWorld,
    Procedural,
    Sandbox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "software_license", rename_all = "snake_case")]
pub enum SoftwareLicense {
    Proprietary,
    Freeware,
    Shareware,
    OpenSource,
    PublicDomain,
    Abandonware,
    Subscription,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "server_status", rename_all = "snake_case")]
pub enum ServerStatus {
    NotApplicable,
    Online,
    Sunset,
    Preservation,
    PrivateServer,
    Announced,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "game_release_model", rename_all = "snake_case")]
pub enum GameReleaseModel {
    Retail,
    Digital,
    FreeToPlay,
    EarlyAccess,
    Subscription,
    Shareware,
    Arcade,
    Browser,
    Mod,
    Demo,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "monetization_model", rename_all = "snake_case")]
pub enum MonetizationModel {
    Premium,
    Free,
    Freemium,
    AdSupported,
    Microtransaction,
    BattlePass,
    Subscription,
    Donation,
    PayWhatYouWant,
    None,
}
