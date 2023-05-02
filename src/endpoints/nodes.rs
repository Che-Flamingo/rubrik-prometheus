// use rubrikprom::configuration::get_config;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SupportTunnel {
    isTunnelEnabled: bool,
}


#[derive(Debug, Deserialize)]
pub enum Node {
    id(String),
    brikId(String),
    status(String),
    ipAddress(String),
    supportTunnel(SupportTunnel),
    hasUnavailableDisks(bool),
    hostname(String)
}


pub async fn get_nodes() -> Vec<Node>{
    //let config = get_config().expect("Failed to read config file");
    //let token = config.token;
    let token = "";
    let client = reqwest::Client::builder()
    .danger_accept_invalid_certs(true)
    .build();

    let response = client
    .expect("Failed to get result from endpoint")
    .get("https://192.168.1.110/api/v1/cluster/me")
    .header(AUTHORIZATION, "Bearer ".to_owned() + &token)
    .send()
    .await.unwrap();

    let body = response.json::<Node>().await.unwrap();
    dbg!("{:?}", body);
    todo!()
}