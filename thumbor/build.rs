fn main() {
    prost_build::Config::new()
        .out_dir("src/abi")
        .compile_protos(&["proto/abi.proto"], &["."])
        .unwrap();
}
