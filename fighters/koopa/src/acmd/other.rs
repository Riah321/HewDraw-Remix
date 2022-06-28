use super::*;

#[acmd_script( agent = "koopa", script = "expression_landingheavy" , category = ACMD_EXPRESSION , low_priority)]
unsafe fn expression_landingheavy(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_landl"), 0, false, 0x50000000 /* default value */);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    } 
}

#[acmd_script( agent = "koopa", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "koopa", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.83, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "koopa", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.875);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("top"), 7.0, 0.0, 8.0, 0.0, Some(0.0), Some(8.0), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 3.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "koopa", script = "game_appealhir" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_appeal_hi_r_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if  !VarModule::is_flag(fighter.battle_object, vars::koopa::instance::NOKNOK_SHELL) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
                VarModule::on_flag(fighter.battle_object, vars::koopa::instance::NOKNOK_SHELL);
                VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1);
            }
        }
    }
    
}

#[acmd_script( agent = "koopa", script = "game_appealhil" , category = ACMD_GAME , low_priority)]
unsafe fn koopa_appeal_hi_l_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if  !VarModule::is_flag(fighter.battle_object, vars::koopa::instance::NOKNOK_SHELL) {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW) {
                ItemModule::have_item(boma, app::ItemKind(*ITEM_KIND_GREENSHELL), 0, 0, false, false);
                VarModule::on_flag(fighter.battle_object, vars::koopa::instance::NOKNOK_SHELL);
                VarModule::set_int(fighter.battle_object, vars::common::instance::GIMMICK_TIMER, 1);
            }
        }
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        //dash_effect,
        koopa_turn_dash_game,
		koopa_catch_game,
        koopa_appeal_hi_r_game,
        koopa_appeal_hi_l_game,
        expression_landingheavy,
    );
}

