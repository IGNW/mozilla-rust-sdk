use std::error::Error;
use std::sync::Arc;

use futures::prelude::*;
use googleapis_raw::cloud::language::v1::{
    language_service::{Document, AnalyzeEntitiesRequest, Document_Type, EncodingType},
    language_service_grpc::LanguageServiceClient,
};
use grpcio::{CallOption, ChannelBuilder, ChannelCredentials, EnvBuilder, MetadataBuilder};

fn main() -> Result<(), Box<dyn Error>> {

    // Google Cloud configuration.
    let endpoint = "language.googleapis.com:443";

    // Set up the gRPC environment.
    let env = Arc::new(EnvBuilder::new().build());

    let creds = ChannelCredentials::google_default_credentials()?;
    let chan = ChannelBuilder::new(env.clone())
        .max_send_message_len(100 << 20)
        .max_receive_message_len(100 << 20)
        .secure_connect(&endpoint, creds);
    let client = LanguageServiceClient::new(chan);

    let mut document = Document::new();
    document.set_content(String::from("The rain in Spain stays mainly in the plain."));
    document.field_type = Document_Type::PLAIN_TEXT;

    let mut req = AnalyzeEntitiesRequest::new();
    req.set_document(document);
    req.encoding_type = EncodingType::UTF8;

    // Make the call.
    let mut meta = MetadataBuilder::new();
    meta.add_str("x-goog-api-client", "googleapis-rs")?;
    let opt = CallOption::default().headers(meta.build());
    let out = client.analyze_entities_opt(&req, opt)?;
    dbg!(out);

    // Make the call asynchronously.
    let mut meta = MetadataBuilder::new();
    meta.add_str("x-goog-api-client", "googleapis-rs")?;
    let opt = CallOption::default().headers(meta.build());
    let fut = client.analyze_entities_async_opt(&req, opt)?;
    let out = fut.wait()?;
    dbg!(out);

    Ok(())
}
