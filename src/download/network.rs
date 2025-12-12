use colored::Colorize;
use reqwest::Client;
use reqwest::Response;
use reqwest::header;

pub struct Network {
    pub client: Client,
}

impl Default for Network {
    fn default() -> Network {
        Network {
            client: Client::new(),
        }
    }
}

impl Network {
    pub fn get(&self, url: &String, range: String) -> Response {
        let request = self.client.get(url).header(header::RANGE, range);

        request
            .send()
            .unwrap_or_else(|err| panic!("{}: {err:?}", "Could not send request.".red().bold()))
    }

    pub fn content_length(&self, url: &String) -> Option<u64> {
        self.get(url, "".to_string()).content_length()
    }
}
