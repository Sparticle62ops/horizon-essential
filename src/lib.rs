//! essential (behavioral port of Essential by EssentialGG, source-available;
//! no upstream code is included).
//!
//! Essential is a session and social mod: host your world for friends through
//! invite codes, chat and share pictures, dress up with cosmetics and the
//! in-game skin manager. Upstream's license is view-only source-available, so
//! this port carries none of its code: the behavior below is written against
//! what the mod's page documents.
//!
//! The backend-dependent parts (upstream's relay for world hosting, their
//! social service for chat and cosmetics delivery) are not shippable from a
//! clean-room port; the local half of each feature is ported here and the
//! seams are in the README.
pub mod gallery;
pub mod session;
pub mod skins;
