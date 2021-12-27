use twilight_model::user::User as DiscordUser;

#[derive(Clone)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub avatar: Option<String>,
}

impl From<DiscordUser> for User {
    fn from(user: DiscordUser) -> Self {
        User {
            id: user.id.0.into(),
            name: user.name,
            avatar: user.avatar,
        }
    }
}
