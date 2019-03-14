use prost_build;
use tower_grpc_build;

fn main() {
    // Build spanner
    let mut prost_config = prost_build::Config::new();
    prost_config.compile_well_known_types();

    let mut tower_config =
        tower_grpc_build::Config::from_prost(prost_config);
    tower_config.enable_client(true);

    tower_config
        .build(
            &[
                "../googleapis/google/bigtable/v2/bigtable.proto",
                "../googleapis/google/pubsub/v1/pubsub.proto",
                "../googleapis/google/spanner/v1/spanner.proto",
                "../googleapis/google/longrunning/operations.proto",
            ],
            &["../googleapis/"],
        )
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
