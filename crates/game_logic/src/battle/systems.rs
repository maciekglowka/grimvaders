use rand::prelude::*;

use crate::{
    commands,
    components::{Position, ORTHO},
    globals::{BOARD_H, BOARD_W},
    utils::get_entity_at,
    GameEnv, World,
};
