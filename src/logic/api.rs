use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub name: String
}

#[tokio::main]
pub async fn make_req() -> Result<Data,  Box<dyn std::error::Error>> {
    let res = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto").await?;
    let body : Data = res.json().await?;
    println!("{:#?}", body);
    Ok(body)
}