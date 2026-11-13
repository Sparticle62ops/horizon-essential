//! The in-game skin manager, local half: the client picks which skin to wear
//! from its saved list and the model layer redraws with the chosen texture.
//! Delivery of cosmetics from upstream's service is a README seam; this owns
//! the selection behavior.
use std::collections::HashMap;

/// One saved skin in the local manager.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Skin {
    /// The file name the player saved it under.
    pub file: String,
    /// Whether the skin reads as a slim (Alex) or wide (Steve) model.
    pub slim: bool,
}

/// The local skin manager: saved skins and the one currently worn.
#[derive(Default)]
pub struct SkinManager {
    /// Saved skins keyed by file name.
    skins: HashMap<String, Skin>,
    /// The file name currently worn, if any.
    worn: Option<String>,
}

impl SkinManager {
    /// Add or replace a saved skin; the model-width read comes from the file
    /// being present and valid.
    pub fn save(&mut self, file: &str, slim: bool) {
        if file.is_empty() {
            return;
        }
        self.skins.insert(
            file.to_string(),
            Skin {
                file: file.to_string(),
                slim,
            },
        );
    }

    /// Wear a saved skin by name. Wearing overwrites any prior choice; an
    /// unknown skin is rejected and leaves the current one on.
    pub fn wear(&mut self, file: &str) -> Option<&Skin> {
        if self.skins.contains_key(file) {
            self.worn = Some(file.to_string());
            self.skins.get(file)
        } else {
            None
        }
    }

    /// The skin the model layer should draw, or None when nothing is worn.
    pub fn current(&self) -> Option<&Skin> {
        self.skins.get(self.worn.as_deref()?)
    }

    /// Remove a saved skin; wearing stops when the worn skin is the one removed.
    pub fn remove(&mut self, file: &str) -> bool {
        if self.skins.remove(file).is_some() {
            if self.worn.as_deref() == Some(file) {
                self.worn = None;
            }
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wear_and_current() {
        let mut m = SkinManager::default();
        m.save("creeper.png", false);
        m.save("ninja.png", true);
        assert_eq!(m.wear("ninja.png").unwrap().slim, true);
        assert_eq!(m.current().unwrap().file, "ninja.png");
        assert!(
            m.wear("ghost.png").is_none(),
            "unknown skin keeps the worn one"
        );
        assert_eq!(m.current().unwrap().file, "ninja.png");
    }

    #[test]
    fn removing_the_worn_skin_undresses() {
        let mut m = SkinManager::default();
        m.save("creeper.png", false);
        m.wear("creeper.png");
        assert!(m.remove("creeper.png"));
        assert!(m.current().is_none());
    }
}
