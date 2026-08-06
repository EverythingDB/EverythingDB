use self::structs::traits;

pub struct media {
    id: i64,
    title: String,
}

impl HasTitle for media {}
impl HasID for media {}
