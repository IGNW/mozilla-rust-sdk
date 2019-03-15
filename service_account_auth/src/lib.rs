use yup_oauth2::{
    self as oauth, GetToken, ServiceAccountAccess,
};

const KEY_PATH_ENV_VAR: &'static str =
    "GOOGLE_APPLICATION_CREDENTIALS";

pub struct TokenGenerator {
    inner: ServiceAccountAccess<hyper::Client>,
    scopes: Vec<&'static str>,
}

impl std::ops::Deref for TokenGenerator {
    type Target = ServiceAccountAccess<hyper::Client>;

    fn deref(
        &self,
    ) -> &ServiceAccountAccess<hyper::Client> {
        &self.inner
    }
}

impl Iterator for TokenGenerator
where
    ServiceAccountAccess<hyper::Client>: GetToken,
{
    type Item = oauth::Token;

    fn next(&mut self) -> Option<oauth::Token> {
        Some(self.token())
    }
}

impl TokenGenerator
where
    ServiceAccountAccess<hyper::Client>: GetToken,
{
    pub fn from_service_account_access(
        inner: ServiceAccountAccess<hyper::Client>,
        scopes: Vec<&'static str>,
    ) -> TokenGenerator {
        TokenGenerator { inner, scopes }
    }

    pub fn new(
        scopes: Vec<&'static str>,
    ) -> TokenGenerator {
        let creds_file_path = match std::env::var(KEY_PATH_ENV_VAR) {
            Ok(val) => val,
            Err(_) => panic!(
                "please set the {} environment variable to a service account json file",
                KEY_PATH_ENV_VAR
            ),
        };

        let client_secret =
            oauth::service_account_key_from_file(
                &creds_file_path,
            )
            .expect("could not parse creds file");
        let client = hyper::Client::with_connector(
            hyper::net::HttpsConnector::new(
                hyper_native_tls::NativeTlsClient::new()
                    .unwrap(),
            ),
        );
        TokenGenerator {
            inner: ServiceAccountAccess::new(
                client_secret,
                client,
            ),
            scopes: scopes,
        }
    }

    pub fn token(&mut self) -> oauth::Token {
        self.inner
            .token(&self.scopes)
            .expect("expect a token to be present")
    }
}
