mod mobs;

pub use crate::mobs::Mob;
pub use crate::mobs::boss::Boss;
pub use crate::mobs::members::{Member, Role};

// Alias for compatibility with external tests
pub mod member {
    pub use crate::mobs::members::Role;
}
