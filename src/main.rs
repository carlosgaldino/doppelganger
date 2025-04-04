use std::collections::BTreeMap;

use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let file_path = std::env::args().nth(1).expect("missing file name");
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;
    let mut instances = BTreeMap::new();
    let mut total_count = 0;
    for entry in reader.deserialize() {
        let entry: Entry = entry?;
        if entry.cpu_set.is_empty() {
            continue;
        }
        // Group entries that share the same 1 second bucket.
        let truncated_date = entry.date.replace_millisecond(0)?;
        instances
            .entry(entry.instance_id.clone())
            .or_insert_with(BTreeMap::new)
            .entry(truncated_date)
            .or_insert_with(BTreeMap::new)
            .entry(entry.cpu_set.clone())
            .or_insert_with(Vec::new)
            .push(entry);
        total_count += 1;
    }

    let mut count = 0;
    for (instance_id, timestamps) in instances {
        eprintln!("instance_id: {}", instance_id);
        for cpu_sets in timestamps.values() {
            for entries in cpu_sets.values() {
                if entries.len() > 1 {
                    count += entries.len();
                    eprintln!("\t{:?}", entries);
                }
            }
        }
    }

    eprintln!(
        "duplicate: {}, total: {}, pct: {}%",
        count,
        total_count,
        (count as f32 / total_count as f32) * 100.0
    );

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Entry {
    #[serde(
        rename(deserialize = "Date"),
        deserialize_with = "time::serde::rfc3339::deserialize"
    )]
    date: time::OffsetDateTime,
    #[serde(rename(deserialize = "@cpuset"), deserialize_with = "remove_quotes")]
    cpu_set: String,
    #[serde(
        rename(deserialize = "@instance_id"),
        deserialize_with = "remove_quotes"
    )]
    instance_id: String,
    #[allow(unused)]
    #[serde(rename(deserialize = "cell_id"), deserialize_with = "remove_quotes")]
    cell_id: String,
}

fn remove_quotes<'a, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'a>,
{
    let s = String::deserialize(deserializer)?;
    let s = s.replace("\"", "");

    Ok(s)
}
