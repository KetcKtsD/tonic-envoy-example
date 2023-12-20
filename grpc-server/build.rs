fn main() {
    println!("cargo:rerun-if-changed=.proto");
    let generated_dir = "src/generated";
    remove_generated_rs(generated_dir);
    build_protoc(generated_dir);
}

fn remove_generated_rs(generated_dir: &str) {
    let Ok(read_dir) = std::fs::read_dir(generated_dir) else { return; };
    for entry in read_dir {
        let Ok(entry) = entry else { continue; };
        let Ok(f_type) = entry.file_type() else {
            continue;
        };
        if f_type.is_file() && entry.path().extension() == Some(std::ffi::OsStr::new("rs")) {
            let _ = std::fs::remove_file(entry.path());
        };
    }
}

fn build_protoc(generated_dir: &str) {
    let include_path = "proto";
    let _ = std::fs::create_dir(generated_dir);
    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .out_dir(generated_dir)
        .compile(
            &[&format!("{}/hello.proto", include_path)],
            &[&include_path],
        )
        .unwrap();
}