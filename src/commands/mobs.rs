use std::time::Duration;

use serenity::{
    all::{
        ButtonStyle, Context, CreateButton, CreateEmbedFooter, CreateInteractionResponse,
        CreateInteractionResponseMessage, CreateMessage, Message, ReactionType,
    },
    futures::StreamExt,
};

use crate::{
    api::{get_mob_data, search_mob_by_name},
    embeds::{get_drops_embed, get_mob_info_embed, get_spawns_embed},
};

pub async fn mob_info(ctx: &Context, msg: &Message, mob_name: &str) {
    let mobs = search_mob_by_name(mob_name).await;
    if mobs.total < 1 {
        let _res = msg
            .channel_id
            .say(&ctx.http, "Monstro não encontrado!")
            .await;
        return;
    }
    let mob = get_mob_data(mobs.items.first().unwrap().id.to_string()).await;
    // 3 pages - mob info, mob drops, mob spawns
    let mut info_embed = get_mob_info_embed(&mob.data).await;
    info_embed = info_embed.footer(CreateEmbedFooter::new(format!("id: {} - 1/3", mob.data.id)));
    let mut drops_embed = get_drops_embed(mob.drops, &mob.data.name, mob.data.id).await;
    drops_embed = drops_embed.footer(CreateEmbedFooter::new(format!("id: {} - 2/3", mob.data.id)));
    let mut spawns_embed = get_spawns_embed(mob.spawns, &mob.data.name, mob.data.id).await;
    spawns_embed =
        spawns_embed.footer(CreateEmbedFooter::new(format!("id: {} - 3/3", mob.data.id)));
    let (previous_button, next_button) = create_buttons();

    let mut current_page: i8 = 0;
    let pages = vec![&info_embed, &drops_embed, &spawns_embed];

    let msg = msg
        .channel_id
        .send_message(
            ctx,
            CreateMessage::new()
                .embed(pages[0].clone())
                .button(previous_button)
                .button(next_button),
        )
        .await
        .unwrap();

    let mut interaction_stream = msg
        .await_component_interaction(&ctx.shard)
        .timeout(Duration::from_secs(60 * 3))
        .stream();

    while let Some(interaction) = interaction_stream.next().await {
        let action = &interaction.data.custom_id;
        // Acknowledge the interaction and send a reply
        if action == "next" {
            current_page = if current_page + 1 > 2 {
                0
            } else {
                current_page + 1
            }
        }
        if action == "previous" {
            current_page = if current_page - 1 < 0 {
                2
            } else {
                current_page - 1
            }
        }

        interaction
            .create_response(
                &ctx.http,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::new()
                        .embed(pages[current_page as usize].clone()),
                ),
            )
            .await
            .unwrap();
    }

    // Delete the orig message or there will be dangling components (components that still
    // exist, but no collector is running so any user who presses them sees an error)
    msg.delete(&ctx).await.unwrap()
}

fn create_buttons() -> (CreateButton, CreateButton) {
    (
        CreateButton::new("previous")
            .emoji(ReactionType::Unicode("👈".to_string()))
            .style(ButtonStyle::Secondary),
        CreateButton::new("next")
            .emoji(ReactionType::Unicode("👉".to_string()))
            .style(ButtonStyle::Secondary),
    )
}
