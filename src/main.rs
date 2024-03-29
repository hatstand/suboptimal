use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde;
use std::collections::HashMap;
use std::fs;

include!(concat!(env!("OUT_DIR"), "/flags.rs"));

#[derive(Serialize, Deserialize, Debug)]
struct Audience {
    id: String,
    conditions: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrafficAllocation {
    #[serde(rename="entityId")]
    entity_id: String,
    #[serde(rename="endOfRange")]
    end_of_range: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeatureFlag {
    key: String,
    #[serde(rename="rolloutId")]
    rollout_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Experiment {
    id: String,
    key: String,
    #[serde(rename="layerId")]
    layer_id: String,
    #[serde(rename="trafficAllocation")]
    traffic_allocation: Vec<TrafficAllocation>,
    #[serde(rename="audienceIds")]
    audience_ids: Vec<String>,
    #[serde(rename="audienceConditions", default)]
    audience_conditions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Rollout {
    id: String,
    experiments: Vec<Experiment>,
}


#[derive(Serialize, Deserialize, Debug)]
struct OptimizelyFile {
    version: String,
    rollouts: Vec<Rollout>,
    #[serde(rename="featureFlags")]
    feature_flags: Vec<FeatureFlag>,
    audiences: Vec<Audience>,
}

async fn print_flags(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .text()
        .await?;

    let f = serde_json::from_str::<OptimizelyFile>(&resp)?;

    let rollout_id_to_rollout: HashMap<_, _> =
        f.rollouts.iter()
        .map(|rollout| (rollout.id.clone(), rollout))
        .collect();

    let audience_id_to_audience: HashMap<_, _> =
        f.audiences.iter()
        .map(|aud| (aud.id.clone(), aud))
        .collect();

    for flag in f.feature_flags.iter().sorted_by(|a, b| Ord::cmp(&a.key, &b.key)) {
        let rollout = rollout_id_to_rollout.get(&flag.rollout_id);
        match rollout {
            Some(r) => {
                println!("{} ({})", flag.key, flag.rollout_id);
                match r.experiments.len() {
                    1 => {
                        match r.experiments[0].traffic_allocation.len() {
                            0 => println!("\t disabled"),
                            1 => {
                                let t = &r.experiments[0].traffic_allocation[0];
                                println!("\t {}%", t.end_of_range / 100);
                            },
                            _ => println!("\t too complicated for me right now:-S"),
                        }
                    },
                    _ => {
                        for exp in r.experiments.iter() {
                            let audiences: Vec<String> = exp.audience_ids.iter()
                                .map(|aud_id| {
                                    match audience_id_to_audience.get(aud_id) {
                                        Some(aud) => aud.name.clone(),
                                        None => aud_id.clone(),
                                    }
                                })
                                .collect();
                            let merged_audience_name = match audiences.len() {
                                0 => "Everyone".to_owned(),
                                _ => audiences.join(", "),
                            };

                            let audience_condition_ids: Vec<String> = exp.audience_conditions.get(1..).unwrap_or(&vec![]).iter()
                                .map(|x| audience_id_to_audience.get(x).map_or(x.clone(), |aud| aud.name.clone()))
                                .sorted()
                                .collect();
                            let condition = exp.audience_conditions.get(0);
                            let condition_desc = match condition {
                                Some(c) => Some(audience_condition_ids.join(&format!(" {} ", c.to_uppercase()))),
                                None => None,
                            };

                            match exp.traffic_allocation.len() {
                                0 => println!("\t disabled for {}", condition_desc.unwrap_or(merged_audience_name)),
                                1 => println!("\t {}% for {}", exp.traffic_allocation[0].end_of_range / 100, condition_desc.unwrap_or(merged_audience_name)),
                                _ => println!("\t too complicated for me right now:-S"),
                            }
                        }
                    }
                }
            },
            None => println!("no matching rollout for flag {}", flag.key)
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_flags("https://flags.creditkudos.com/Production.json")
        .await?;

    let f = fs::read_to_string("./src/example.json")?;
    let proto = serde_json::from_str::<FlagsFile>(&f)?;
    // let proto = FlagsFile::decode(io::Cursor::new(f))?;

    println!("example: {:?}", proto);

    Ok(())
}
