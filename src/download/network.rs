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
    pub fn make_request(&self, url: &String, range: String) -> Response {
        let request = self.client.get(url).header(header::RANGE, range);

        request.send().expect("Could not send request.")
    }

    pub fn get_content_length(&self, url: &String) -> Option<u64> {
        self.make_request(url, "".to_string()).content_length()
    }
}
