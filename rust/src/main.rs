use serde::{Deserialize, Serialize};

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

    match serde_json::from_str::<OptimizelyFile>(&resp) {
        Ok(f) => println!("{:#?}", f),
        Err(e) => println!(":-( {}", e),
    }

    Ok(())
}
