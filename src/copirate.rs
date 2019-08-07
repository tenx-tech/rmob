use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::BoxResult;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CoPirate {
    pub name: String,
    pub email: String,
}

impl Display for CoPirate {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{} <{}>", self.name, self.email)
    }
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CoPirates {
    copirates: HashMap<String, CoPirate>,
}

impl CoPirates {
    pub fn open(copirates_path: &Path) -> BoxResult<CoPirates> {
        let raw_copirates = fs::read_to_string(copirates_path)?;
        let existing_copirates = serde_json::from_str(&raw_copirates)?;
        Ok(existing_copirates)
    }

    pub fn get(&self, copirate: &String) -> BoxResult<&CoPirate> {
        let copirate = self.copirates.get(copirate).ok_or("Shiver me timbers! This be pirate be a stranger around these ports. Hint: Add it to ~/.git-copirates!")?;
        Ok(copirate)
    }
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
