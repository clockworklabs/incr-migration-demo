use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

#[spacetimedb::table(name = character, public)]
struct Character {
    #[primary_key]
    player_id: Identity,
    nickname: String,
    level: u32,
    class: Class,
}

#[derive(SpacetimeType, Debug, Copy, Clone)]
enum Class {
    Fighter,
    Caster,
    Medic,
}

#[spacetimedb::reducer]
fn create_character(ctx: &ReducerContext, class: Class, nickname: String) {
    log::info!("Creating new level 1 {class:?} named {nickname}",);
    ctx.db.character().insert(Character {
        player_id: ctx.sender,
        nickname: nickname.clone(),
        level: 1,
        class,
    });
}

fn find_character_for_player(ctx: &ReducerContext) -> Character {
    ctx.db
        .character()
        .player_id()
        .find(ctx.sender)
        .expect("Player has not created a character")
}

fn update_character(ctx: &ReducerContext, character: Character) {
    ctx.db.character().player_id().update(character);
}

#[spacetimedb::reducer]
fn rename_character(ctx: &ReducerContext, new_name: String) {
    let character = find_character_for_player(ctx);
    log::info!("Renaming {} to {}", character.nickname, new_name,);
    update_character(
        ctx,
        Character {
            nickname: new_name,
            ..character
        },
    );
}

#[spacetimedb::reducer]
fn level_up_character(ctx: &ReducerContext) {
    let character = find_character_for_player(ctx);
    log::info!(
        "Leveling up {} from {} to {}",
        character.nickname,
        character.level,
        character.level + 1,
    );
    update_character(
        ctx,
        Character {
            level: character.level + 1,
            ..character
        },
    );
}
