extern crate bytes;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate tower_grpc;

pub mod google {
    pub mod protobuf {
        include!(concat!(
            env!("OUT_DIR"),
            "/google.protobuf.rs"
        ));
    }

    pub mod api {
        include!(concat!(
            env!("OUT_DIR"),
            "/google.api.rs"
        ));
    }

    pub mod longrunning {
        include!(concat!(
            env!("OUT_DIR"),
            "/google.longrunning.rs"
        ));
    }

    pub mod rpc {
        include!(concat!(
            env!("OUT_DIR"),
            "/google.rpc.rs"
        ));
    }

    pub mod bigtable {
        pub mod v2 {
            include!(concat!(
                env!("OUT_DIR"),
                "/google.bigtable.v2.rs"
            ));
        }
    }

    pub mod pubsub {
        pub mod v1 {
            include!(concat!(
                env!("OUT_DIR"),
                "/google.pubsub.v1.rs"
            ));
        }
    }

    pub mod spanner {
        pub mod v1 {
            include!(concat!(
                env!("OUT_DIR"),
                "/google.spanner.v1.rs"
            ));
        }
    }
}
