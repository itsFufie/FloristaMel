// connect and serialize data

use reqwest::Url;

use crate::response_structs::{MobWrapper, Mobs};

const RAGNAPLACE_LIST_URI: &str = "https://api.ragnaplace.com/api/db/list/";
const RAGNAPLACE_MOB_INFO: &str = "https://api.ragnaplace.com/api/db/mob/";
pub const RAGNAPLACE_MOB_SPRITES: &str = "https://game.ragnaplace.com/ro/job/";

pub async fn search_mob_by_name(mob_name: &str) -> Mobs {
    let uri = Url::parse_with_params(
        RAGNAPLACE_LIST_URI,
        &[
            ("db", "mob"),
            ("gateway", "rolatam-pt"),
            ("skip", "0"),
            ("take", "20"),
            ("search", mob_name.trim()),
        ],
    )
    .unwrap();
    let result = reqwest::get(uri).await.unwrap();
    result.json().await.unwrap()
}

pub async fn get_mob_data(id: String) -> MobWrapper {
    let mut uri = Url::parse(RAGNAPLACE_MOB_INFO).unwrap().join(&id).unwrap();

    uri.set_query(Some("gateway=rolatam-pt"));
    println!("{}", uri);
    let result = reqwest::get(uri).await.unwrap();
    result.json().await.unwrap()
}

pub async fn get_sprite_url(id: u32) -> Url {
    Url::parse(RAGNAPLACE_MOB_SPRITES)
        .unwrap()
        .join(&format!("{}/{}", id.to_string(), "0.png"))
        .unwrap()
}
