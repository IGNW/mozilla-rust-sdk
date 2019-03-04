use yup_oauth2::{self as oauth, GetToken, ServiceAccountAccess};

const KEY_PATH_ENV_VAR: &'static str = "GOOGLE_APPLICATION_CREDENTIALS";

pub struct TokenGenerator<C> {
    inner: ServiceAccountAccess<C>,
    scopes: Vec<&'static str>,
}

impl<C> std::ops::Deref for TokenGenerator<C> {
    type Target = ServiceAccountAccess<C>;

    fn deref(&self) -> &ServiceAccountAccess<C> {
        &self.inner
    }
}

impl<C> Iterator for TokenGenerator<C>
where
    ServiceAccountAccess<C>: GetToken,
{
    type Item = oauth::Token;

    fn next(&mut self) -> Option<oauth::Token> {
        Some(self.token())
    }
}

impl<C> TokenGenerator<C>
where
    ServiceAccountAccess<C>: GetToken,
{
    pub fn from_service_account_access(
        inner: ServiceAccountAccess<C>,
        scopes: Vec<&'static str>,
    ) -> TokenGenerator<C> {
        TokenGenerator { inner, scopes }
    }

    pub fn new(scopes: Vec<&'static str>) -> TokenGenerator<hyper::Client> {
        let creds_file_path = match std::env::var(KEY_PATH_ENV_VAR) {
            Ok(val) => val,
            Err(_) => panic!(
                "please set the {} environment variable to a service account json file",
                KEY_PATH_ENV_VAR
            ),
        };

        let client_secret = oauth::service_account_key_from_file(&creds_file_path)
            .expect("could not parse creds file");
        let client = hyper::Client::with_connector(hyper::net::HttpsConnector::new(
            hyper_native_tls::NativeTlsClient::new().unwrap(),
        ));
        TokenGenerator {
            inner: ServiceAccountAccess::new(client_secret, client),
            scopes: scopes,
        }
    }

    pub fn token(&mut self) -> oauth::Token {
        self.inner
            .token(&self.scopes)
            .expect("expect a token to be present")
    }
}
