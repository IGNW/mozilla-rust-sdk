use std::net::ToSocketAddrs;

use pbgen::google::bigtable::v2::{
    client::Bigtable, ReadRowsRequest,
};
use service_account_auth::TokenGenerator;

use native_tls::TlsConnector as NativeTlsConnector;
use tokio::executor::DefaultExecutor;
use tokio::net::tcp::TcpStream;
use tokio_tls::TlsConnector;
use tower_grpc::{metadata::MetadataValue, Request};
use tower_h2::client;
use tower_service::Service;
use tower_util::MakeService;

use futures::{Future, Poll};

const BIGTABLE_ADDR: &'static str = "googleapis.com:50051";
const BIGTABLE_DOMAIN: &'static str = "googleapis.com";

pub mod scopes {
    /// DATA is the OAuth scope for Cloud Bigtable data operations.
    pub const DATA: &'static str =
        "https://www.googleapis.com/auth/bigtable.data";

    /// READ_ONLY is the OAuth scope for Cloud Bigtable read-only data operations.
    pub const READ_ONLY: &'static str =
        "https://www.googleapis.com/auth/bigtable.readonly";

    /// ADMIN is the OAuth scope for Cloud Bigtable table admin operations.
    pub const ADMIN: &'static str = "https://www.googleapis.com/auth/bigtable.admin.table";

    /// INSTANCE_ADMIN is the OAuth scope for Cloud Bigtable instance (and cluster) admin operations.
    pub const INSTANCE_ADMIN: &'static str = "https://www.googleapis.com/auth/bigtable.admin.cluster";
}

pub struct BigtableClient<S> {
    inner: Bigtable<S>,
    token_gen: TokenGenerator,
}

impl<R> BigtableClient<R> {
    fn read_rows<T>(&mut self) -> impl Future
    where
        T: tower::HttpService<R>,
    {
        let mut request = Request::new(ReadRowsRequest {
            table_name: "my_table".to_string(),
            ..Default::default()
        });

        let token: String =
            self.token_gen.token().access_token;

        request.metadata_mut().insert(
            "Bearer",
            MetadataValue::try_from_bytes(token.as_bytes())
                .unwrap(),
        );

        self.inner.read_rows(request)
    }
}

fn main() {
    let token_gen =
        TokenGenerator::new(vec![scopes::ADMIN]);

    let h2_settings = Default::default();

    let mut make_client = client::Connect::new(
        Tls::new(BIGTABLE_ADDR),
        h2_settings,
        DefaultExecutor::current(),
    );

    let client = make_client.make_service(()).map(|s| {
        BigtableClient {
            inner: Bigtable::new(s),
            token_gen,
        }
    });

    let read_rows = client
        .and_then(|mut client| {
            client
                .read_rows(Request::new(ReadRowsRequest {
                    table_name: "my_table".to_string(),
                    ..Default::default()
                }))
                .map_err(|e| {
                    panic!(
                        "gRPC request failed; err={:?}",
                        e
                    )
                })
        })
        .and_then(|response| {
            println!("RESPONSE = {:?}", response);
            Ok(())
        })
        .map_err(|e| {
            println!("ERR = {:?}", e);
        });

    tokio::run(read_rows);
}

struct Tls {
    addr: &'static str,
}

impl Tls {
    fn new(addr: &'static str) -> Tls {
        Tls { addr }
    }
}

impl Service<()> for Tls {
    type Response = tokio_tls::TlsStream<TcpStream>;
    type Error = native_tls::Error;
    type Future = Box<
        Future<
                Item = tokio_tls::TlsStream<TcpStream>,
                Error = native_tls::Error,
            > + Send,
    >;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(&mut self, _: ()) -> Self::Future {
        let addr = self
            .addr
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap();

        let tcp_connection = TcpStream::connect(&addr)
            .map_err(|e| {
                panic!("unable to connect: {}", e)
            });

        Box::new(tcp_connection.and_then(move |socket| {
            // upgrade to TLS
            println!("TCP connection established, now starting TLS handshake");

            let native_connector =
            NativeTlsConnector::new().unwrap();

            let connector: TlsConnector = native_connector.into();

            connector.connect(BIGTABLE_DOMAIN, socket)
        }))
    }
}
