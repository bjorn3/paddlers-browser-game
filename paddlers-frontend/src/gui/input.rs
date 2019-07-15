use quicksilver::geom::{Vector, Shape, Rectangle};
use specs::prelude::*;
use specs::world::Index;
use crate::gui::{
    utils::*,
    sprites::WithSprite,
    gui_components::*,
};
use crate::game::movement::Position;
use crate::game::town_resources::TownResources;
use paddlers_api_lib::types::*;
use paddlers_api_lib::shop::*;

#[derive(Default, Clone, Copy)]
pub struct MouseState(pub Vector, pub bool);

#[derive(Default, Clone)]
pub struct UiState {
    pub selected_entity: Option<Index>,
    pub hovered_entity: Option<Index>,
    pub grabbed_item: Option<Grabbable>,
    pub menu_box_area: Rectangle,
}
pub struct MouseSystem;

#[derive(Default, Debug, Component)]
#[storage(NullStorage)]
pub struct Clickable;

#[derive(Clone)]
pub enum Grabbable {
    NewBuilding(BuildingType),
}

impl<'a> System<'a> for MouseSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseState>,
        Write<'a, UiState>,
        Read<'a, DefaultShop>,
        Read<'a, TownResources>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Clickable>,
     );

    fn run(&mut self, (entities, mouse_state, mut ui_state, shop, resis, position, clickable): Self::SystemData) {
        let MouseState(mouse_pos, clicking) = *mouse_state;
        if mouse_pos.overlaps_rectangle(&(*ui_state).menu_box_area) {
            if clicking && (*ui_state).selected_entity.is_none() {
                let maybe_grab = shop.click(mouse_pos);
                if let Some(Grabbable::NewBuilding(b)) = maybe_grab {
                    if (*resis).can_afford(&b.price()) {
                        (*ui_state).grabbed_item = maybe_grab;
                    }
                }
            }
            return;
        }

        (*ui_state).hovered_entity = None;
        if clicking {
            (*ui_state).selected_entity = None;
        }

        for (e, pos) in (&entities, &position).join() {
            if mouse_pos.overlaps_rectangle(&pos.area) {
                (*ui_state).hovered_entity = Some(e.id());
                let clickable: Option<&Clickable> = clickable.get(e);
                if clicking && clickable.is_some() {
                    (*ui_state).selected_entity = Some(e.id());
                }
            }
        }
    }

}

// TODO: Eventually, this should be split up between different buildings
#[derive(Clone)]
pub struct DefaultShop {
    pub ui: UiBox<BuildingType>,
}
impl Default for DefaultShop {
    fn default() -> Self {
        DefaultShop {
            ui : UiBox::new(Rectangle::default(), 3, 5)
        }
    }
}
impl DefaultShop {
    pub fn new(area: Rectangle) -> Self {
        let mut result = DefaultShop {
            ui : UiBox::new(area, 3, 5)
        };
        result.add_building(BuildingType::BlueFlowers);
        result.add_building(BuildingType::RedFlowers);
        result
    }

    fn add_building(&mut self, b: BuildingType) {
        self.ui.add_with_background_color_and_cost(b.sprite(), WHITE, b, b.cost());
    }

    fn click(&self, mouse: impl Into<Vector>) -> Option<Grabbable> {
        let buy_this = self.ui.click(mouse);
        if let Some(building_type) = buy_this {
            return Some(
                Grabbable::NewBuilding(building_type)
            )
        }
        None
    }
}