// WARN: some AI slop - check it before doing anything with it
use crate::utils::types::Score;
use rustsec::{Advisory, Database, Vulnerability};

pub struct SecurityChecker {
    db: Database,
}

impl SecurityChecker {
    pub fn new() -> Result<Self> {
        // Fetches latest advisory database
        let db = Database::fetch()?;
        Ok(Self { db })
    }

    pub fn check_crate(&self, name: &str, version: &str) -> Score {
        let query = self
            .db
            .query()
            .package_name(name)
            .package_version(version.parse().ok());

        let advisories: Vec<&Advisory> = self.db.query(&query);

        if advisories.is_empty() {
            Score {
                value: 1.0,
                details: vec!["No known vulnerabilities".to_string()],
            }
        } else {
            Score {
                value: 0.2, // Severe penalty for vulnerabilities
                details: advisories
                    .iter()
                    .map(|adv| {
                        format!(
                            "{}: {} (severity: {:?})",
                            adv.id(),
                            adv.title(),
                            adv.severity()
                        )
                    })
                    .collect(),
            }
        }
    }
}

// WARN: check the caching strategy proposed here - it was suggested in the concept of controlling
// the update frequency
//
// ***********************************************************************************************
// impl SecurityChecker {
//     pub fn new_with_cache(max_age: Duration) -> Result<Self> {
//         let db = if Self::should_update(max_age)? {
//             Database::fetch()?  // Download fresh
//         } else {
//             Database::open(&Database::default_path())?  // Use cached
//         };
//         Ok(Self { db })
//     }
//
//     fn should_update(max_age: Duration) -> Result<bool> {
//         let db_path = Database::default_path();
//         if !db_path.exists() {
//             return Ok(true);
//         }
//
//         let metadata = std::fs::metadata(&db_path)?;
//         let age = metadata.modified()?.elapsed()?;
//         Ok(age > max_age)
//     }
// }
// ***********************************************************************************************

