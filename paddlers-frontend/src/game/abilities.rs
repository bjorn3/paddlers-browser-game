mod welcome;
pub use welcome::*;
use paddlers_shared_lib::prelude::*;
use crate::prelude::*;
use crate::net::graphql::query_types::parse_timestamp;
use crate::gui::{
    gui_components::UiBox,
    sprites::WithSprite,
};

/// A unit can learn a limited number of Abilities. (including walking)
/// Although this simplifies things on the technical side, this is mainly
/// motivated from a game-design perspective. (simplicity)
pub const MAX_ABILITIES: usize = 4;

/// Represent the abilities a single unit instance has.
pub struct AbilitySet {
    abilities: [Option<AbilityType>; MAX_ABILITIES],
    last_used: [Option<Timestamp>; MAX_ABILITIES],
}

use crate::net::graphql::village_units_query::VillageUnitsQueryVillageWorkersAbilities;
impl AbilitySet {
    pub fn from_gql(gql_abilities: &[VillageUnitsQueryVillageWorkersAbilities]) -> PadlResult<AbilitySet> {
        if gql_abilities.len() > MAX_ABILITIES {
            return PadlErrorCode::InvalidGraphQLData("Too many abilities").dev();
        }
        let mut abilities: [Option<AbilityType>; MAX_ABILITIES] = [None;MAX_ABILITIES];
        let mut last_used: [Option<Timestamp>; MAX_ABILITIES] = [None;MAX_ABILITIES];
        let mut i = 0;
        for gqla in gql_abilities {
            abilities[i] = Some((&gqla.ability_type).into());
            last_used[i] = gqla.last_used.as_ref().map(parse_timestamp);
            i += 1;
        }
        Ok(
            AbilitySet {
                abilities,
                last_used,
            }
        )
    }
    pub fn construct_ui_box(&self) -> UiBox {
        let rows = 2;
        let mut ui = UiBox::new(MAX_ABILITIES / rows, rows, 0.0, 1.0);
        for i in 0..MAX_ABILITIES {
            let a = self.abilities[i];
            let lu = self.last_used[i];
            if let Some(ability) = a {
                if let Some(t) = lu {
                    ui.add_with_cooldown(ability.sprite(), ability, t, t + ability.cooldown().num_microseconds().unwrap());
                } else {
                    ui.add(ability.sprite(), ability);
                }
            }
        }
        ui
    }
}