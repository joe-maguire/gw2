//! Provides an interface forinteracting with the Guild Wars 2 offical API.

extern crate reqwest;

use serde::{Serialize, Deserialize};

pub mod schema;

#[derive(Serialize, Deserialize, Debug)]
pub struct Build {
    id: u32,
}

impl Build {
    pub async fn get() -> Result<Build> { 
        reqwest::get("https://api.guildwars2.com/v2/build").await?.json::<Build>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_build() {
        assert_eq!(Build.get().await? , Build { id: 101113 });
    }
}
