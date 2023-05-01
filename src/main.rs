use reqwest::{header::{AUTHORIZATION, CONTENT_TYPE}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Data {
    id: String,
    version: String,
    name: String,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyYjRiM2UxNS0yMDc3LTRhOWYtOGZjMi00NDNlYWIzZDliODMiLCJpc01mYVJlbWVtYmVyVG9rZW4iOmZhbHNlLCJpc3MiOiI3NTg2MDU1My1hN2QwLTRhOGItOWVlNy1kYTIxZDZkZDQzZTMiLCJpc1JlZHVjZWRTY29wZSI6ZmFsc2UsImlhdCI6MTY4MjkzNjg0NCwianRpIjoiNWJkNjYxZTgtZTcxOC00NmE4LWI1ZjctOGYxNWRjOWRhMzZkIn0.SPx9AI37W3yISjg9rFefjZJ1RUMhqQGPwfj4F9C1BLQ";

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