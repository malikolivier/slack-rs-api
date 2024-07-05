//! Functionality for sending requests to Slack.

#[cfg(feature = "reqwest_blocking")]
mod reqwest_support {
    pub use self::reqwest::Error;
    use reqwest_ as reqwest;

    use crate::sync::SlackWebRequestSender;

    type Client = reqwest::blocking::Client;

    impl SlackWebRequestSender for Client {
        type Error = reqwest::Error;

        fn get<S>(&self, method_url: S, params: &[(&str, &str)]) -> Result<String, Self::Error>
        where
            S: AsRef<str> + Send,
        {
            let mut url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");

            let mut token = None;
            for (name, value) in params {
                if *name == "token" {
                    token = Some(*value);
                } else {
                    url.query_pairs_mut().append_pair(name, value);
                }
            }

            let mut request = self.get(url);

            if let Some(token) = token {
                request = request.bearer_auth(token);
            }

            Ok(request.send()?.text()?)
        }

        fn post<S>(
            &self,
            method_url: S,
            form: &[(&str, &str)],
            headers: &[(&str, &str)],
        ) -> Result<String, Self::Error>
        where
            S: AsRef<str> + Send,
        {
            let url = reqwest::Url::parse(method_url.as_ref()).expect("Unable to parse url");
            let mut req = self.post(url).form(form);
            for (k, v) in headers {
                if *k == "token" {
                    req = req.bearer_auth(*v);
                } else {
                    req = req.header(*k, *v);
                }
            }
            Ok(req.send()?.text()?)
        }
    }

    /// Provides a default `reqwest` client to give to the API functions to send requests.
    ///
    /// # Examples
    ///
    /// ```
    /// # let token = "some_token";
    /// let client = slack_api::requests::default_client().unwrap();
    /// let params = Default::default();
    /// let response = slack_api::conversations::list(&client, &token, &params);
    /// ```
    pub fn default_client() -> Result<Client, reqwest::Error> {
        Ok(Client::new())
    }
}

#[cfg(feature = "reqwest_blocking")]
pub use self::reqwest_support::*;
