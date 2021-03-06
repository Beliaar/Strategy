use crate::components::node_component::NodeComponent;
use crate::components::player::Player;
use crate::components::unit::Unit;
use crate::game_state::GameState;
use crate::game_state::State::Selected;
use gdnative::prelude::*;
use legion::{system, Entity};

pub mod dummy_unit;

#[system(par_for_each)]
pub fn update_units(
    entity: &Entity,
    node: &NodeComponent,
    unit: &Unit,
    player: &Player,
    #[resource] state: &GameState,
) {
    let node = match node.get_node() {
        Some(node) => node,
        None => return,
    };
    let integrity_label = node
        .get_node("Integrity")
        .and_then(|node| unsafe { node.assume_safe_if_sane() })
        .and_then(|node| node.cast::<Label>());
    let integrity_label = match integrity_label {
        None => {
            godot_error!("Node has no Integrity label");
            return;
        }
        Some(label) => label,
    };

    integrity_label.set_text(format!("{}", unit.integrity));

    let visible = if let Selected(selected) = state.state {
        *entity == selected
    } else {
        false
    };

    let outline = node.get_node("Outline");

    let outline = outline
        .and_then(|outline| unsafe { outline.assume_safe_if_sane() })
        .and_then(|outline| outline.cast::<CanvasItem>());
    let outline = match outline {
        None => {
            return;
        }
        Some(outline) => outline,
    };

    outline.set_visible(visible);
    let model = node
        .get_node("Model")
        .and_then(|node| unsafe { node.assume_safe_if_sane() })
        .and_then(|node| node.cast::<CanvasItem>());

    let model = match model {
        None => {
            godot_error!("Node has no Model CanvasItem node");
            return;
        }
        Some(model) => model,
    };
    let player = &state.players[player.0];
    let colour = player.get_colour();
    model.set_modulate(colour);
}
