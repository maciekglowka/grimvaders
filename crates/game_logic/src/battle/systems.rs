use wunderkammer::prelude::*;

use crate::{
    commands,
    components::{Position, ORTHO},
    globals::{BOARD_H, BOARD_W},
    scripting::run_command_script,
    utils::get_entity_at,
    GameEnv, World,
};
