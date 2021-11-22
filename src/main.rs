use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Mountain {
    name: String,
    name_kana: String,
    area: String,
    prefectures: Vec<String>,
    elevation: u32,
    location: Location,
    tags: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Location {
    latitude: f32,
    longitude: f32,
    gsi_url: String,
}

fn main() {
    let resp = print_mountain();
    match resp {
        Ok(resp) => println!("{}", resp),
        _ => ()
    }
}

fn print_mountain() -> Result<String> {
    let location = Location {
        latitude: 145.2738888888889,
        longitude: 44.23583333333333,
        gsi_url: "https://dottrail.codemountains.org".to_string()
    };

    let mountain = Mountain {
        name: "唐松岳".to_string(),
        name_kana: "からまつだけ".to_string(),
        area: "飛騨山脈".to_string(),
        prefectures: vec!["北海道".to_string()],
        elevation: 2678,
        location,
        tags: vec!["百名山".to_string()]
    };

    let mountain_json = serde_json::to_string(&mountain)?;

    Ok(mountain_json)
}
