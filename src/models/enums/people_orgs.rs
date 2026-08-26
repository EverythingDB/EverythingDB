//! Enums for the `person` and `organization` root tables.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type, Default, Serialize, Deserialize)]
#[sqlx(type_name = "gender", rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
    #[default]
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "org_type", rename_all = "snake_case")]
pub enum OrgType {
    Studio,
    Publisher,
    Imprint,
    Developer,
    Distributor,
    RecordLabel,
    Network,
    StreamingService,
    ProductionCommittee,
    Licensor,
    Printer,
    TheatreCompany,
    Collective,
    Museum,
    Other,
}
