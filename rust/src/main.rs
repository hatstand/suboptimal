use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://flags.creditkudos.com/Production.json")
        .await?
        .text()
        .await?;

    let f = serde_json::from_str::<OptimizelyFile>(&resp)?;

    let rollout_id_to_rollout: HashMap<_, _> =
        f.rollouts.iter()
        .map(|rollout| (rollout.id.clone(), rollout))
        .collect();

    for flag in f.feature_flags.iter().sorted_by(|a, b| Ord::cmp(&a.key, &b.key)) {
        let rollout = rollout_id_to_rollout.get(&flag.rollout_id);
        match rollout {
            Some(r) => {
                match r.experiments.len() {
                    1 => {
                        println!("{} ({})", flag.key, flag.rollout_id);
                        match r.experiments[0].traffic_allocation.len() {
                            0 => println!("\t disabled"),
                            1 => {
                                let t = &r.experiments[0].traffic_allocation[0];
                                println!("\t {}%", t.end_of_range / 100)
                            },
                            _ => println!("\t too complicated for me right now:-S"),
                        }
                    },
                    _ => println!("{} ({}) is too complicated for me right now:-S", flag.key, flag.rollout_id),
                }
            },
            None => println!("no matching rollout for flag {}", flag.key)
        }
    }

    Ok(())
}
