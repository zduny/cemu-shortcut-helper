use reqwest::{Error, blocking::Client, header::{self, HeaderMap, HeaderValue}};

const CHROME_USER_AGENT: &str = 
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/94.0.4606.81 Safari/537.36";
const COOKIE_CONSENT_COOKIE: &str =
    "gf_dvi=ZjYxNmYyZjc1MDA1ZjA5MDJmMjhlNGNhNGZhYzcwYTUwNDU0YzIwZjljMzFmMTRjZjVkYWIwMmQ0ODU3NjE2ZjJmNzU%3D; gf_geo=OTEuMTQ5LjIxMi4yMzg6NjE2OjA%3D; fv20211020=1; spt=yes; OptanonConsent=isIABGlobal=false&datestamp=Tue+Oct+19+2021+22%3A50%3A02+GMT%2B0200+(czas+%C5%9Brodkowoeuropejski+letni)&version=6.7.0&hosts=&consentId=ac6461c8-4f5a-4ee9-b644-d8740effb9d2&interactionCount=1&landingPath=NotLandingPage&groups=C0002%3A0%2CC0003%3A0%2CC0004%3A0%2CC0005%3A0&AwaitingReconsent=false";

pub fn create_client() -> Result<Client, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, HeaderValue::from_static(COOKIE_CONSENT_COOKIE));

    Client::builder()
        .https_only(true)
        .default_headers(headers)
        .user_agent(CHROME_USER_AGENT)
        .build()
}