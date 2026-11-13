//! The screenshot gallery, local half: screenshots are captured, listed
//! newest-first, favorited, deleted, and marked for sharing. Upstream shares
//! through its social service; without that transport the share flag is the
//! outbox a transport reads. The seam is in the README.
use std::collections::HashMap;

/// One screenshot in the gallery.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shot {
    /// The file name on disk.
    pub file: String,
    /// Unix seconds of capture, the gallery's sort key.
    pub taken_at: u64,
    /// Whether the player starred it.
    pub favorite: bool,
    /// Whether it is queued for sharing.
    pub outbox: bool,
}

/// The local gallery.
#[derive(Default)]
pub struct Gallery {
    shots: HashMap<String, Shot>,
}

impl Gallery {
    /// Record a captured screenshot.
    pub fn add(&mut self, file: &str, taken_at: u64) {
        self.shots.insert(
            file.to_string(),
            Shot {
                file: file.to_string(),
                taken_at,
                favorite: false,
                outbox: false,
            },
        );
    }

    /// Every shot, newest first.
    pub fn list(&self) -> Vec<Shot> {
        let mut all: Vec<Shot> = self.shots.values().cloned().collect();
        all.sort_by(|a, b| {
            b.taken_at
                .cmp(&a.taken_at)
                .then_with(|| a.file.cmp(&b.file))
        });
        all
    }

    /// Toggle the favorite star. Returns the new state; unknown files fail.
    pub fn favorite(&mut self, file: &str) -> Option<bool> {
        self.shots.get_mut(file).map(|s| {
            s.favorite = !s.favorite;
            s.favorite
        })
    }

    /// Mark or unmark a shot for sharing. Returns the new state; unknown files fail.
    pub fn queue_share(&mut self, file: &str, queued: bool) -> Option<bool> {
        self.shots.get_mut(file).map(|s| {
            s.outbox = queued;
            s.outbox
        })
    }

    /// Delete a shot from the gallery (the file deletion itself rides the host).
    pub fn delete(&mut self, file: &str) -> bool {
        self.shots.remove(file).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newest_first_ordering() {
        let mut g = Gallery::default();
        g.add("a.png", 10);
        g.add("b.png", 30);
        g.add("c.png", 20);
        let names: Vec<String> = g.list().iter().map(|s| s.file.clone()).collect();
        assert_eq!(names, vec!["b.png", "c.png", "a.png"]);
    }

    #[test]
    fn favorite_share_delete() {
        let mut g = Gallery::default();
        g.add("shot.png", 1);
        assert_eq!(g.favorite("shot.png"), Some(true));
        assert_eq!(g.favorite("shot.png"), Some(false));
        assert_eq!(g.queue_share("shot.png", true), Some(true));
        assert_eq!(g.queue_share("ghost.png", true), None);
        assert!(g.delete("shot.png"));
        assert!(!g.delete("shot.png"));
    }
}
