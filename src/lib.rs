use spacetimedb::{Identity, ReducerContext, SpacetimeType, Table};

#[spacetimedb::table(name = character, public)]
struct Character {
    #[primary_key]
    player_id: Identity,
    nickname: String,
    level: u32,
    class: Class,
}

#[spacetimedb::table(name = character_v2, public)]
struct CharacterV2 {
    #[primary_key]
    player_id: Identity,
    nickname: String,
    level: u32,
    class: Class,
    alliance: Alliance,
}

#[derive(SpacetimeType, Debug, Copy, Clone)]
enum Class {
    Fighter,
    Caster,
    Medic,
}

#[derive(SpacetimeType, Debug, Copy, Clone)]
enum Alliance {
    Good,
    Neutral,
    Evil,
}

#[spacetimedb::reducer]
fn create_character(ctx: &ReducerContext, class: Class, nickname: String) {
    log::info!(
        "Creating new level 1 {class:?} named {nickname} for player {}",
        ctx.sender,
    );

    ctx.db.character().insert(Character {
        player_id: ctx.sender,
        nickname: nickname.clone(),
        level: 1,
        class,
    });

    ctx.db.character_v2().insert(CharacterV2 {
        player_id: ctx.sender,
        nickname,
        level: 1,
        class,
        alliance: Alliance::Neutral,
    });
}

fn find_character_for_player(ctx: &ReducerContext) -> CharacterV2 {
    if let Some(character) = ctx.db.character_v2().player_id().find(ctx.sender) {
        // Already migrated; just return the new player.
        return character;
    }

    // Not yet migrated; look up an old character and update it.
    let old_character = ctx
        .db
        .character()
        .player_id()
        .find(ctx.sender)
        .expect("Player has not created a character");

    ctx.db.character_v2().insert(CharacterV2 {
        player_id: old_character.player_id,
        nickname: old_character.nickname,
        level: old_character.level,
        class: old_character.class,
        alliance: Alliance::Neutral,
    })
}

fn update_character(ctx: &ReducerContext, character: CharacterV2) {
    ctx.db.character().player_id().update(Character {
        player_id: character.player_id,
        nickname: character.nickname.clone(),
        level: character.level,
        class: character.class,
    });
    ctx.db.character_v2().player_id().update(character);
}

#[spacetimedb::reducer]
fn rename_character(ctx: &ReducerContext, new_name: String) {
    let character = find_character_for_player(ctx);
    log::info!("Renaming {} to {}", character.nickname, new_name,);
    update_character(
        ctx,
        CharacterV2 {
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
        CharacterV2 {
            level: character.level + 1,
            ..character
        },
    );
}

#[spacetimedb::reducer]
fn choose_alliance(ctx: &ReducerContext, alliance: Alliance) {
    let character = find_character_for_player(ctx);
    log::info!(
        "Setting alliance of {} to {:?}",
        character.nickname,
        alliance,
    );
    update_character(
        ctx,
        CharacterV2 {
            alliance,
            ..character
        },
    );
}
