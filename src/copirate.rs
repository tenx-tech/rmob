//! Configuration for all known co-author names and emails.

use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::BoxResult;

/// Represents the details of a single co-author.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CoPirate {
    pub name: String,
    pub email: String,
}

impl Display for CoPirate {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{} <{}>", self.name, self.email)
    }
}

/// Configuration file which assigns convenient two-letter aliases to co-authors.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct CoPirates {
    copirates: HashMap<String, CoPirate>,
}

impl CoPirates {
    /// Creates an empty copirates file if it doesn't exist.
    /// Otherwise, opens the existing one.
    pub fn create_or_open(copirates_path: &Path) -> BoxResult<CoPirates> {
        if !copirates_path.exists() {
            let empty_copirates = CoPirates {
                copirates: HashMap::new(),
            };
            empty_copirates.save(copirates_path)?;
        }

        Self::open(copirates_path)
    }

    /// Parses the given co-authors file as JSON.
    pub fn open(copirates_path: &Path) -> BoxResult<CoPirates> {
        let raw_copirates = fs::read_to_string(copirates_path)?;
        let existing_copirates = serde_json::from_str(&raw_copirates)?;
        Ok(existing_copirates)
    }

    /// Returns the details of the given co-author, if known.
    pub fn get(&self, copirate: &String) -> BoxResult<&CoPirate> {
        let copirate = self.copirates.get(copirate).ok_or("Shiver me timbers! This be pirate be a stranger around these ports. Hint: run `rmob copirate add --help`")?;
        Ok(copirate)
    }

    /// Adds copirate by alias
    pub fn add(&mut self, alias: &str, copirate: CoPirate) -> BoxResult<()> {
        if !self.copirates.contains_key(alias) {
            self.copirates.insert(alias.to_owned(), copirate);
            Ok(())
        } else {
            Err(format!("Co-pirate with alias '{}' already exists", alias).into())
        }
    }

    /// Removes existing copirate
    pub fn remove(&mut self, alias: &str) -> BoxResult<()> {
        self.copirates
            .remove(alias)
            .ok_or(format!("Co-pirate with alias '{}' is not found!", alias))?;
        Ok(())
    }

    /// Saves copirates into file
    pub fn save(&self, copirates_path: &Path) -> BoxResult<()> {
        let json = serde_json::to_string(self)?;
        fs::write(copirates_path, json.as_bytes())?;
        Ok(())
    }
}

/// Adds copirate with specific credentials to copirates file. Creates new copirates file if it
/// didn't exist before. Won't add another copirate with the same alias
pub(crate) fn add(copirates_file: &str, alias: &str, name: &str, email: &str) -> BoxResult<()> {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let copirates_path = &ship.join(copirates_file);
    let mut existing_copirates = CoPirates::create_or_open(copirates_path)?;

    let copirate = CoPirate {
        name: name.to_owned(),
        email: email.to_owned(),
    };
    existing_copirates.add(alias, copirate)?;
    existing_copirates.save(copirates_path)?;

    Ok(())
}

/// Removes existing copirate by their alias from the copirates file
pub(crate) fn remove(copirates_file: &str, alias: &str) -> BoxResult<()> {
    let ship = dirs::home_dir().ok_or("Could not find yer ship oy!")?;
    let copirates_path = &ship.join(copirates_file);
    let mut existing_copirates = CoPirates::open(copirates_path)?;

    existing_copirates.remove(alias)?;
    existing_copirates.save(copirates_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! copirates {
        ($($alias:expr => { $name:expr, $email:expr }),*) => {{
            let mut copirates = HashMap::new();
            $(
                copirates.insert($alias.to_string(), CoPirate {
                    name: $name.to_string(),
                    email: $email.to_string(),
                });
            )*
            CoPirates { copirates }
        }};
    }

    #[test]
    fn deserialize_from_json() {
        let expected = copirates!("js" => { "John Smith", "jsmith@gmail.com" });
        let actual: CoPirates = serde_json::from_str(
            r#"{
                "copirates": {
                    "js": {
                        "name": "John Smith",
                        "email": "jsmith@gmail.com"
                    }
                }
            }"#,
        )
        .expect("Failed to parse JSON");

        assert_eq!(expected, actual);
    }

    #[test]
    fn converts_to_coauthor_format() {
        let expected = "John Smith <jsmith@gmail.com>";
        let actual = CoPirate {
            name: "John Smith".to_owned(),
            email: "jsmith@gmail.com".to_owned(),
        };

        assert_eq!(expected, actual.to_string());
    }
}
