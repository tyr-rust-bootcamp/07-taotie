use anyhow::Result;
use arrow::{
    datatypes::{DataType, Field, Schema},
    json::{writer::JsonArray, ReaderBuilder, WriterBuilder},
};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize};
use std::{fs::File, io::BufReader, sync::Arc};

/**
* /*
       CREATE TABLE user_stats(
     email varchar(128) NOT NULL PRIMARY KEY,
     name varchar(64) NOT NULL,
     gender gender DEFAULT 'unknown',
     created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
     last_visited_at timestamptz,
     last_watched_at timestamptz,
     recent_watched int[],
     viewed_but_not_started int[],
     started_but_not_finished int[],
     finished int[],
     last_email_notification timestamptz,
     last_in_app_notification timestamptz,
     last_sms_notification timestamptz
   ); */
*/
#[derive(Debug, Serialize, Deserialize)]
struct User {
    email: String,
    name: String,
    gender: String,
    #[serde(deserialize_with = "deserialize_string_date")]
    created_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_visited_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_watched_at: Option<DateTime<Utc>>,
    recent_watched: Vec<i32>,
    viewed_but_not_started: Vec<i32>,
    started_but_not_finished: Vec<i32>,
    finished: Vec<i32>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_email_notification: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_in_app_notification: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_sms_notification: Option<DateTime<Utc>>,
}

fn main() -> Result<()> {
    /*
        CREATE TABLE user_stats(
      email varchar(128) NOT NULL PRIMARY KEY,
      name varchar(64) NOT NULL,
      gender gender DEFAULT 'unknown',
      created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
      last_visited_at timestamptz,
      last_watched_at timestamptz,
      recent_watched int[],
      viewed_but_not_started int[],
      started_but_not_finished int[],
      finished int[],
      last_email_notification timestamptz,
      last_in_app_notification timestamptz,
      last_sms_notification timestamptz
    ); */
    // help me generate the schema based on the above SQL
    let schema = Schema::new(vec![
        Field::new("email", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("gender", DataType::Utf8, false),
        Field::new("created_at", DataType::Date64, false),
        Field::new("last_visited_at", DataType::Date64, true),
        Field::new("last_watched_at", DataType::Date64, true),
        Field::new(
            "recent_watched",
            DataType::List(Arc::new(Field::new(
                "recent_watched",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "viewed_but_not_started",
            DataType::List(Arc::new(Field::new(
                "viewed_but_not_started",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "started_but_not_finished",
            DataType::List(Arc::new(Field::new(
                "started_but_not_finished",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "finished",
            DataType::List(Arc::new(Field::new("finished", DataType::Int32, false))),
            true,
        ),
        Field::new("last_email_notification", DataType::Date64, true),
        Field::new("last_in_app_notification", DataType::Date64, true),
        Field::new("last_sms_notification", DataType::Date64, true),
    ]);

    // load the data from the ndjson file
    let reader = BufReader::new(File::open("assets/users.ndjson")?);
    let reader = ReaderBuilder::new(Arc::new(schema)).build(reader)?;
    for batch in reader {
        let batch = batch?;

        let data: Vec<u8> = Vec::new();
        let mut writer = WriterBuilder::new()
            .with_explicit_nulls(true)
            .build::<_, JsonArray>(data);
        writer.write_batches(&[&batch])?;
        writer.finish()?;
        let data = writer.into_inner();

        // deserialize the data
        let users: Vec<User> = serde_json::from_slice(&data)?;
        for user in users {
            println!("{:?}", user);
        }
    }
    Ok(())
}

fn deserialize_string_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    // format 2019-12-28T05:35:42.771
    let from: NaiveDateTime = s.parse().map_err(serde::de::Error::custom)?;
    let date_time = Utc.from_local_datetime(&from).unwrap();

    Ok(date_time)
}

fn deserialize_string_date_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    match s {
        Some(s) => {
            let from: NaiveDateTime = s.parse().map_err(serde::de::Error::custom)?;
            let date_time = Utc.from_local_datetime(&from).unwrap();
            Ok(Some(date_time))
        }
        None => Ok(None),
    }
}
