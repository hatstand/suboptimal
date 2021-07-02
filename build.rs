fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut config = prost_build::Config::new();
  config.type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]");
  config.type_attribute(".", "#[serde(rename_all = \"camelCase\")]");
  config.compile_protos(&["src/flags.proto"], &["src/"])?;
  Ok(())
}
