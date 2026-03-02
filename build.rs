fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/generated")
        .compile(
            &glob::glob("protos/**/*.proto")
                .unwrap()
                .filter_map(Result::ok)
                .collect::<Vec<_>>(),
            &["protos"],
        )
        .unwrap();
}
