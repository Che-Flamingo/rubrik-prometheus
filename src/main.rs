use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use rubrikprom::configuration::{get_config};

#[derive(Debug, Deserialize)]
struct Data {
    id: String,
    version: String,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config().expect("Failed to read config file");
    let token = config.token;
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client
        .get("https://192.168.1.110/api/v1/cluster/me")
        .header(AUTHORIZATION, "Bearer ".to_owned() + &token)
        .send()
        .await?;

    let body = res.json::<Data>().await?;

    println!("{:#?}", body);
    Ok(())
}

// .get("https://192.168.1.110/api/v1/cluster/me")
// .header(AUTHORIZATION, "Bearer ".to_owned() + &token)
// .header(CONTENT_TYPE, "application/json")
// .send()
// .await;

// println!("{:?}", response);

// curl -k -s -X POST https://192.168.1.110/api/v1/service_account/session -H "accept: application/json" -H "Content-Type: application/json" -d "{ \"serviceAccountId\": \"$USER\", \"secret\": \"$SECRET\"}"

// "{
//     \"id\":\"db48993a-b89d-41c4-9ef4-5327dab4f5c6\",
//     \"version\":\"8.0.2-22392\",
//     \"apiVersion\":\"1\",
//     \"name\":\"CHME-CRISTIETEST\",
//     \"timezone\":{
//         \"timezone\":\"Europe/Amsterdam\"
//     },
//     \"acceptedEulaVersion\":\"1.3\",
//     \"latestEulaVersion\":\"1.3\",
//     \"registeredMode\":\"LifeOfDevice\"
// }"
