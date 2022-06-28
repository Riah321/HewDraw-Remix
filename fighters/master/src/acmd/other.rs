use super::*;


#[acmd_script( agent = "master", script = "game_catch" , category = ACMD_GAME , low_priority)]
unsafe fn master_catch_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.200);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        GrabModule::set_rebound(boma, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.000);
        CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 8.0, 0.0, Some(0.0), Some(8.0), Some(12.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    game_CaptureCutCommon(fighter);
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(boma, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(boma, false);
    }
    
}

#[acmd_script( agent = "master", script = "game_dash" , category = ACMD_GAME , low_priority)]
unsafe fn dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.5);
    }
	frame(lua_state, 11.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
		WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "master", script = "effect_dash" , category = ACMD_EFFECT , low_priority)]
unsafe fn dash_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.63, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }    
}

#[acmd_script( agent = "master", script = "game_turndash" , category = ACMD_GAME , low_priority)]
unsafe fn turn_dash_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.3);
		WorkModule::on_flag(boma, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(lua_state, 13.0); // Effectively F16
    if is_excute(fighter) {
		FT_MOTION_RATE(fighter, 1.0);
        WorkModule::enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    
}

#[acmd_script( agent = "master_arrow1", script = "game_fly" , category = ACMD_GAME , low_priority)]
unsafe fn master_arrow1_fly_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 40, 74, 0, 40, 1.6, 0.0, 0.0, -1.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting_bowarrow"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(boma);
    }
    
}

#[acmd_script( agent = "master_arrow2", script = "game_search" , category = ACMD_GAME , low_priority)]
unsafe fn master_arrow2_search_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, 2.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, -2.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 30.0, 361, 60, 0, 53, 1.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_ARROW_MAX, *ATTACK_REGION_ENERGY);
        AttackModule::disable_tip(boma);
    }
    
}

#[acmd_script( agent = "master_axe", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn master_axe_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) > 0 {
            WorkModule::set_int(boma, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
        }
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 0 {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 18.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 1 {
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 30.0, 275, 50, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else {
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 270, 0, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 60.0, 275, 50, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);

        }
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "master_axe", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn master_axe_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    frame(lua_state, 61.0);
    if is_excute(fighter) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) > 0 {
            WorkModule::set_int(boma, 0, *WEAPON_MASTER_AXE_INSTANCE_WORK_ID_INT_CRITICAL_ATTACK_ID);
        }
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 0 {
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 18.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 1 {
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 51, 83, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 30.0, 275, 50, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
        }
        else {
            // Ground-only
            ATTACK(fighter, 0, 0, Hash40::new("haver"), 30.0, 270, 0, 0, 60, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);
            // Air-only
            ATTACK(fighter, 1, 0, Hash40::new("haver"), 60.0, 275, 50, 0, 20, 5.7, 0.0, 14.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MASTER_AXE, *ATTACK_REGION_OBJECT);

        }
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "master_axe", script = "effect_speciallwhit" , category = ACMD_EFFECT , low_priority)]
unsafe fn master_axe_special_lw_hit_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if is_excute(fighter) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 2 {
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_axe_aura"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp1"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp2"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp3"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_axe_aura"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("master_axe_slash_edge"), -1);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        if VarModule::get_int(owner_module_accessor.object(), vars::master::status::AYMR_CHARGE_LEVEL) == 2 {
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp1_end"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp2_end"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp3_end"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        else{
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp1_end"), Hash40::new("blade1"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp2_end"), Hash40::new("blade2"), 0, 0, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("master_axeflare_sp3_end"), Hash40::new("axe"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare_sp1"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare_sp2"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("master_axeflare_sp3"), false, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("master_axe_aura"), false, true);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        master_catch_game,
        dash_game,
        //dash_effect,
        turn_dash_game,
        master_arrow1_fly_game,
        master_arrow2_search_game,
        master_axe_special_lw_game,
        master_axe_special_air_lw_game,
        master_axe_special_lw_hit_effect,
    );
}

