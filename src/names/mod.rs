use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HumanNames {
    pub male: Vec<String>,
    pub female: Vec<String>,
    pub non_binary: Vec<String>,
    pub surnames: Vec<String>,
}

impl Default for HumanNames {
    fn default() -> Self {
        HumanNames {
            male: vec![
                "Alaric".to_string(),
                "Archibald".to_string(),
                "Arne".to_string(),
                "Arthur".to_string(),
                "Bahram".to_string(),
                "Bard".to_string(),
                "Bartholomew".to_string(),
                "Benedict".to_string(),
            ],
            female: vec![
                "Adelaide".to_string(),
                "Amelia".to_string(),
                "Beatriz".to_string(),
                "Ella".to_string(),
                "Erika".to_string(),
                "Genevieve".to_string(),
                "Mirabel".to_string(),
                "Olive".to_string(),
            ],
            non_binary: vec![
                "Asmi".to_string(),
                "Clement".to_string(),
                "Drew".to_string(),
                "Felize".to_string(),
                "Peregrine".to_string(),
                "Quentin".to_string(),
                "Rogue".to_string(),
                "Stace".to_string(),
            ],
            surnames: vec![
                "Armstrong".to_string(),
                "Baker".to_string(),
                "Beaumont".to_string(),
                "Blackwood".to_string(),
                "Carpenter".to_string(),
                "Chamberlain".to_string(),
                "Davenport".to_string(),
            ],
        }
    }
}
