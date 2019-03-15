use std::net::ToSocketAddrs;

use futures::{Future, Poll};
use native_tls::TlsConnector as NativeTlsConnector;
use tokio::executor::DefaultExecutor;
use tokio::net::tcp::TcpStream;
use tokio_tls::TlsConnector;
use tower_grpc::{
    metadata::{MetadataKey, MetadataValue},
    Request,
};
use tower_h2::client;
use tower_service::Service;
use tower_util::MakeService;

use pbgen::google::bigtable::v2::{
    client::Bigtable, SampleRowKeysRequest,
};
use service_account_auth::TokenGenerator;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    /// Instance name
    #[structopt(short = "i", long = "instance")]
    instance: String,

    /// Project name
    #[structopt(short = "p", long = "project")]
    project: String,
}

const BIGTABLE_ADDR: &'static str = "googleapis.com:443";
const BIGTABLE_DOMAIN: &'static str = "googleapis.com";

pub mod scopes {
    /// DATA is the OAuth scope for Cloud Bigtable data operations.
    pub const DATA: &'static str =
        "https://www.googleapis.com/auth/bigtable.data";

    /// READ_ONLY is the OAuth scope for
    /// Cloud Bigtable read-only data operations.
    pub const READ_ONLY: &'static str =
        "https://www.googleapis.com/auth/bigtable.readonly";

    /// ADMIN is the OAuth scope for Cloud Bigtable
    /// table admin operations.
    pub const ADMIN: &'static str =
        "https://www.googleapis.com/auth/bigtable.admin.table";

    /// INSTANCE_ADMIN is the OAuth scope for Cloud Bigtable
    /// instance (and cluster) admin operations.
    pub const INSTANCE_ADMIN: &'static str =
        "https://www.googleapis.com/auth/bigtable.admin.cluster";
}

pub struct BigtableClient<T> {
    project: String,
    instance: String,
    inner: Bigtable<T>,
    token_gen: TokenGenerator,
}

impl<T> BigtableClient<T> {
    fn full_table_name(&self, table: &str) -> String {
        format!(
            "projects/{}/instances/{}/tables/{}",
            self.project, self.instance, table
        )
    }

    fn add_bearer<R: std::fmt::Debug>(
        &mut self,
        request: R,
    ) -> Request<R> {
        /*
        let mut request = http::Request::new(request);
        *request.uri_mut() = scopes::ADMIN.parse().unwrap();

        let mut request = Request::from_http(request);
        */

        let mut request = Request::new(request);

        let token: String =
            self.token_gen.token().access_token;

        let mut header = String::from("Bearer ");
        header.push_str(&token);

        request.metadata_mut().insert(
            MetadataKey::from_bytes(
                "Authorization".as_bytes(),
            )
            .unwrap(),
            MetadataValue::try_from_bytes(
                header.as_bytes(),
            )
            .unwrap(),
        );

        println!("request: {:?}", request);

        request
    }
}

impl<T> std::ops::Deref for BigtableClient<T> {
    type Target = Bigtable<T>;

    fn deref(&self) -> &Bigtable<T> {
        &self.inner
    }
}

fn main() {
    let opt = Opt::from_args();

    let token_gen =
        TokenGenerator::new(vec![scopes::ADMIN]);

    let h2_settings = Default::default();

    let mut make_client = client::Connect::new(
        Tls,
        h2_settings,
        DefaultExecutor::current(),
    );

    let google_uri: http::Uri =
        "https://googleapis.com".parse().unwrap();

    let client = make_client
        .make_service((BIGTABLE_ADDR, BIGTABLE_DOMAIN))
        .map(|s| {
            println!("creating client");
            let mut with_origin: tower_add_origin::Builder =
                tower_add_origin::Builder::new();

            with_origin.uri(google_uri);

            let with_origin = with_origin.build(s).unwrap();

            BigtableClient {
                project: opt.project.into(),
                instance: opt.instance.into(),
                inner: Bigtable::new(with_origin),
                token_gen,
            }
        });

    let read_rows = client
        .and_then(|mut client| {
            println!("sending request to client");
            let table =
                client.full_table_name("Hello-Bigtable");

            let request = SampleRowKeysRequest {
                table_name: table,
                ..Default::default()
            };
            let request = client.add_bearer(request);
            client.inner.sample_row_keys(request).map_err(
                |e| {
                    panic!(
                        "gRPC request failed; err={:?}",
                        e
                    )
                },
            )
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

#[derive(Debug)]
enum ConnectError {
    Io(std::io::Error),
    Tls(native_tls::Error),
}

impl From<std::io::Error> for ConnectError {
    fn from(io: std::io::Error) -> ConnectError {
        ConnectError::Io(io)
    }
}

impl From<native_tls::Error> for ConnectError {
    fn from(tls: native_tls::Error) -> ConnectError {
        ConnectError::Tls(tls)
    }
}

struct Tls;

type FuResponse = tokio_tls::TlsStream<TcpStream>;
type FuError = ConnectError;
type Fuuu =
    Future<Item = FuResponse, Error = FuError> + Send;

impl Service<(&'static str, &'static str)> for Tls {
    type Response = FuResponse;
    type Error = FuError;
    type Future = Box<Fuuu>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        Ok(().into())
    }

    fn call(
        &mut self,
        (addr, domain): (&'static str, &'static str),
    ) -> Self::Future {
        let addr =
            addr.to_socket_addrs().unwrap().next().unwrap();

        println!("got addr: {:?}", addr);

        let tcp_connection = TcpStream::connect(&addr)
            .map_err(|e: std::io::Error| {
                panic!("failed to connect: {}", e);
            });

        let tls_connection = tcp_connection
            .and_then(move |socket| {
                // upgrade to TLS
                println!(
                    "TCP connection established, \
                     now starting TLS handshake"
                );

                let native_connector =
                    NativeTlsConnector::new().unwrap();

                let connector: TlsConnector =
                    native_connector.into();

                connector.connect(domain, socket)
            })
            .map_err(|e: native_tls::Error| {
                ConnectError::from(e)
            });

        Box::new(tls_connection)
    }
}
