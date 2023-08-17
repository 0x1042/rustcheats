use std::io::Result;

fn main() -> Result<()> {
    let mut config = prost_build::Config::new();

    config.out_dir("src/meta");
    config.type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]");
    config.compile_protos(&["idl/meta.proto"], &["idl/"])?;
    Ok(())
}
