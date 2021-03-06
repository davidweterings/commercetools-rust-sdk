#[macro_use]
extern crate maplit;
extern crate reqwest;
extern crate serde_yaml;

use oauth2::AuthType;
use oauth2::Config;
use std::{env};

mod commercetools;


fn main() -> Result<(), Box<std::error::Error>> {
    let client_id = env::var("CTP_CLIENT_ID").unwrap();
    let client_secret = env::var("CTP_CLIENT_SECRET").unwrap();
    let mut config = Config::new(
        client_id,
        client_secret,
        "https://auth.sphere.io",
        "https://auth.sphere.io/oauth/token",
    );
    config = config.add_scope("manage_project:nl-dev-david");
    config = config.set_auth_type(AuthType::BasicAuth);

    let token_result = config.exchange_client_credentials().unwrap();
    println!("Token result: {:?}", token_result);

    let client = reqwest::Client::new();
    let mut response = client
        .get("https://api.sphere.io/nl-dev-david/products/")
        .bearer_auth(&token_result.access_token)
        .send()?;
    let cats: commercetools::product::ProductPagedQueryResponse = response.json()?;


    println!("Cats: {:?}", cats);
    /*

    let mut config = Config::new(
        client_id,
        client_secret,
        "https://auth.sphere.io",
        "https://auth.sphere.io/oauth/token",
    );
    config = config.add_scope("manage_project:nl-dev-david");
    config = config.set_auth_type(AuthType::BasicAuth);

    let token_result = config.exchange_client_credentials().unwrap();
    println!("Token result: {:?}", token_result);

    let channel_update = ChannelUpdate {
        version: 5,
        actions: vec![
            ChannelUpdateAction::ChangeKey {
                key: "Shipwire".to_owned(),
            },
            ChannelUpdateAction::ChangeName {
                name: hashmap! {"en".to_owned() => "Shipwire".to_owned()},
            },
        ],
    };

    let client = reqwest::Client::new();
    let mut response = client
        .post("https://api.sphere.io/nl-dev-david/channels/7d0dc66a-1b3c-4d80-aaa2-7b6773e8323f")
        .bearer_auth(&token_result.access_token)
        .json(&channel_update)
        .send()?;

    println!("Channel update: {:?}", response.text()?);

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Got filename: {}", filename);
    let f = std::fs::File::open(filename)?;
    let d: RamlDefinition = serde_yaml::from_reader(f)?;
    println!("Read YAML string: {:?}", d);
    */
    Ok(())
}
