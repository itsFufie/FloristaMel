use serenity::all::{CreateEmbed, CreateEmbedFooter};

use crate::{
    api::get_sprite_url,
    response_structs::{MobDrop, MobInfo, SpawnInfo},
};

pub async fn get_drops_embed(drops: Vec<MobDrop>, mob_name: &str, id: u32) -> CreateEmbed {
    let footer = CreateEmbedFooter::new(format!("id: {}", id));
    let embed = CreateEmbed::new()
        .title(format!("{} - Drops", mob_name))
        .thumbnail(get_sprite_url(id).await)
        .footer(footer);
    let mut fields = Vec::new();
    for mob_drop in drops {
        fields.push((
            mob_drop.item.data.identifiedDisplayName,
            format!("{:.} %", (mob_drop.chance as f32 / 100.0)),
            true,
        ));
    }
    embed.fields(fields)
}

pub async fn get_mob_info_embed(mob: &MobInfo) -> CreateEmbed {
    let footer = CreateEmbedFooter::new(format!("id: {} ", mob.id));
    CreateEmbed::new()
        .title(format!("{} - Info", mob.name))
        .fields(vec![
            ("level", mob.level.to_string(), true),
            (
                "element",
                format!("{} {}", mob.element, mob.elementLevel),
                true,
            ),
            ("size", mob.size.clone(), true),
            ("race", mob.race.clone(), true),
            ("baseEXP", mob.baseExp.to_string(), true),
            ("jobEXP", mob.jobExp.to_string(), true),
        ])
        .image(get_sprite_url(mob.id).await)
        .footer(footer)
}

pub async fn get_spawns_embed(mut spawns: Vec<SpawnInfo>, mob_name: &str, id: u32) -> CreateEmbed {
    let footer = CreateEmbedFooter::new(format!("id: {}", id));
    let mut fields = Vec::new();
    spawns.sort_by(|a, b| b.amount.cmp(&a.amount));
    for spawn_info in spawns {
        fields.push((
            format!("mapa: {}", spawn_info.map.id),
            format!("qntd: {}", spawn_info.amount),
            true,
        ));
    }
    CreateEmbed::new()
        .title(format!("{} - Spawns", mob_name))
        .footer(footer)
        .thumbnail(get_sprite_url(id).await)
        .fields(fields)
}
