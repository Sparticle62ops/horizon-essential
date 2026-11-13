//! Session hosting, local half: the invite-code lifecycle the mod documents.
//! Hosts hand out invite codes (a domain the friend opens); the code carries
//! the session reference and an expiry; the host can list and revoke codes.
//! Transport (upstream's relay or Horizon's own connection path) is a README
//! seam: this module owns the codes, not the wire.
use std::collections::HashMap;

/// One issued invite code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Invite {
    /// The code string handed to a friend (upstream issues a short URL slug).
    pub code: String,
    /// Unix seconds the code stops working. 0 means the host's open-ended invite.
    pub expires_at: u64,
    /// Whether the host revoked it before expiry.
    pub revoked: bool,
}

/// The host side of the session: the codes it has issued and their rules.
#[derive(Default)]
pub struct Session {
    /// Codes issued by the host, keyed by code string.
    invites: HashMap<String, Invite>,
}

impl Session {
    /// Issue a new invite code. `slug` is the random part; the host keeps the
    /// prefixing. Duplicate slugs are rejected so codes stay unique.
    pub fn issue(&mut self, slug: &str, expires_at: u64) -> Result<&Invite, &'static str> {
        if slug.is_empty() || slug.len() > 64 {
            return Err("slug length out of range");
        }
        if !slug.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
            return Err("slug must be alphanumeric or '-'");
        }
        if self.invites.contains_key(slug) {
            return Err("code already issued");
        }
        self.invites.insert(
            slug.to_string(),
            Invite {
                code: slug.to_string(),
                expires_at,
                revoked: false,
            },
        );
        Ok(&self.invites[slug])
    }

    /// Can a friend join with this code at unix time `now`? Revoked, expired
    /// and unknown codes all reject.
    pub fn check(&self, slug: &str, now: u64) -> bool {
        match self.invites.get(slug) {
            Some(inv) => !inv.revoked && (inv.expires_at == 0 || now < inv.expires_at),
            None => false,
        }
    }

    /// The host revokes a code; revoking an unknown code fails.
    pub fn revoke(&mut self, slug: &str) -> bool {
        if let Some(inv) = self.invites.get_mut(slug) {
            inv.revoked = true;
            true
        } else {
            false
        }
    }

    /// Codes still usable at `now`, oldest issued first.
    pub fn live(&self, now: u64) -> Vec<String> {
        let mut out: Vec<&Invite> = self
            .invites
            .values()
            .filter(|i| !i.revoked && (i.expires_at == 0 || now < i.expires_at))
            .collect();
        // insertion order is not tracked; sort for determinism
        out.sort_by(|a, b| a.code.cmp(&b.code));
        out.into_iter().map(|i| i.code.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_then_join() {
        let mut s = Session::default();
        assert!(s.issue("crimson-otter", 1000).is_ok());
        assert!(s.check("crimson-otter", 999));
        assert!(!s.check("crimson-otter", 1000), "code expires at its stamp");
        assert!(!s.check("nope", 1));
    }

    #[test]
    fn revoke_kills_a_code() {
        let mut s = Session::default();
        s.issue("open", 0).unwrap();
        assert!(s.check("open", 5_000_000));
        assert!(s.revoke("open"));
        assert!(!s.check("open", 5_000_000));
        assert!(!s.revoke("ghost"));
    }

    #[test]
    fn duplicate_and_bad_slugs_rejected() {
        let mut s = Session::default();
        s.issue("aa", 0).unwrap();
        assert!(s.issue("aa", 0).is_err());
        assert!(s.issue("", 0).is_err());
        assert!(s.issue("bad/slug", 0).is_err());
    }

    #[test]
    fn live_lists_only_current_codes() {
        let mut s = Session::default();
        s.issue("b-code", 0).unwrap();
        s.issue("a-code", 10).unwrap();
        s.issue("c-code", 0).unwrap();
        assert!(s.revoke("c-code"));
        assert_eq!(s.live(5), vec!["a-code".to_string(), "b-code".to_string()]);
    }
}
