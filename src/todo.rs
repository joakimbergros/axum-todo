use chrono::{DateTime, Utc};
use std::time::SystemTime;

#[derive(serde::Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub content: String,
    pub done: bool,
    #[serde(with = "time_ser")]
    created_at: DateTime<Utc>,
    #[serde(with = "time_ser_option")]
    updated_at: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn new(id: u64, title: String, content: String) -> Self {
        Self {
            id,
            title,
            content,
            done: false,
            created_at: SystemTime::now().into(),
            updated_at: None,
        }
    }

    pub fn update(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
        self.updated_at = Some(SystemTime::now().into());
    }

    pub fn toggle_state(&mut self) {
        self.done = !self.done;
        self.updated_at = Some(SystemTime::now().into());
    }
}

#[derive(serde::Deserialize)]
pub struct SetTodo {
    pub title: String,
    pub content: String,
}

mod time_ser {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.collect_str(&date.to_rfc3339())?)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date_time = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(date_time.with_timezone(&Utc))
    }
}

mod time_ser_option {
    use chrono::{DateTime, Utc};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Some(date) = date else {
            return serializer.serialize_none();
        };

        Ok(serializer.collect_str(&date.to_rfc3339())?)
    }

    /*pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() {
            return Ok(None);
        }

        let date_time = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(date_time.with_timezone(&Utc)))
    }*/
}
