use reqwest::header::*;
use reqwest::{Proxy, RedirectPolicy};
use std::collections::HashMap;
use std::time::Duration;

pub struct HttpOptions {
    check_ssl: bool,
    follow_redirects: bool,
    proxy: Option<Proxy>,
    user_agent: String,
    timeout: Duration,
    headers: HashMap<String, String>,
}

impl HttpOptions {
    pub fn new(
        check_ssl: bool,
        follow_redirects: bool,
        proxy: Option<Proxy>,
        user_agent: String,
        timeout: Duration,
        headers: HashMap<String, String>,
    ) -> HttpOptions {
        return Self {
            check_ssl,
            follow_redirects,
            proxy,
            user_agent,
            timeout,
            headers,
        };
    }

    pub fn user_agent(&self) -> &String {
        return &self.user_agent;
    }

    pub fn follow_redirects(&self) -> bool {
        return self.follow_redirects;
    }

    pub fn check_ssl(&self) -> bool {
        return self.check_ssl;
    }

    pub fn proxy(&self) -> Option<&Proxy> {
        return self.proxy.as_ref();
    }

    pub fn timeout(&self) -> &Duration {
        return &self.timeout;
    }
}

impl Default for HttpOptions {
    fn default() -> Self {
        return Self {
            check_ssl: true,
            follow_redirects: false,
            proxy: None,
            user_agent: "barrido".to_string(),
            timeout: Duration::from_secs(30),
            headers: HashMap::new(),
        };
    }
}

impl Into<reqwest::Client> for HttpOptions {
    fn into(self) -> reqwest::Client {
        let mut headers = HeaderMap::new();

        headers.insert(
            reqwest::header::USER_AGENT,
            self.user_agent.parse().expect("Error parsing User-Agent"),
        );

        for (header_name, header_value) in self.headers {
            headers.insert(
                HeaderName::from_bytes(header_name.as_bytes()).expect(
                    &format!("Error parsing header name {}", header_name),
                ),
                header_value.parse().expect(&format!(
                    "Error parsing value name {}",
                    header_value
                )),
            );
        }

        let mut client_builder = reqwest::Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(!self.check_ssl)
            .use_sys_proxy()
            .timeout(self.timeout);

        if !self.follow_redirects {
            client_builder = client_builder.redirect(RedirectPolicy::none())
        }

        if let Some(proxy) = self.proxy {
            client_builder = client_builder.proxy(proxy);
        }

        return client_builder.build().expect("Error building client");
    }
}
