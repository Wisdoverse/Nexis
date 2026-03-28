//! Identity domain extensions for Nexis.

pub use nexis_protocol::MemberId;

#[derive(Debug, Clone)]
pub struct Identity {
    pub id: MemberId,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl Identity {
    pub fn new(id: MemberId) -> Self {
        Self {
            id,
            display_name: None,
            avatar_url: None,
        }
    }

    pub fn with_display_name(mut self, name: String) -> Self {
        self.display_name = Some(name);
        self
    }

    pub fn with_avatar(mut self, url: String) -> Self {
        self.avatar_url = Some(url);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nexis_protocol::{MemberId, MemberType};

    #[test]
    fn identity_new_has_no_optional_fields() {
        let id = MemberId::new(MemberType::Human, "alice").unwrap();
        let identity = Identity::new(id);
        assert!(identity.display_name.is_none());
        assert!(identity.avatar_url.is_none());
    }

    #[test]
    fn identity_builder_chain() {
        let id = MemberId::new(MemberType::Agent, "bot-1").unwrap();
        let identity = Identity::new(id)
            .with_display_name("Bot".into())
            .with_avatar("https://img.url/avatar.png".into());

        assert_eq!(identity.display_name.as_deref(), Some("Bot"));
        assert_eq!(identity.avatar_url.as_deref(), Some("https://img.url/avatar.png"));
    }

    #[test]
    fn identity_clone_preserves_all_fields() {
        let id = MemberId::new(MemberType::Ai, "gpt-4").unwrap();
        let original = Identity::new(id.clone())
            .with_display_name("AI".into());
        let cloned = original.clone();
        assert_eq!(cloned.id, id);
        assert_eq!(cloned.display_name, original.display_name);
    }
}
