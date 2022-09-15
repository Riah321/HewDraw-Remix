// status imports
use super::*;
use globals::*;

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            FighterStatusUniqProcessDamage_leave_stop_hook,
            ftstatusuniqprocessdamage_init_common
        );
    }
}

// this runs as you leave hitlag
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_FighterStatusUniqProcessDamage_leave_stop)]
pub unsafe fn FighterStatusUniqProcessDamage_leave_stop_hook(fighter: &mut L2CFighterCommon, arg2: L2CValue, arg3: L2CValue) -> L2CValue {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if !arg3.get_bool() {
        return 0.into();
    }
    let hashmap = fighter.local_func__fighter_status_damage_2();
    // vanilla ASDI routine (only runs for paralyze/crumple attacks)
    // if hashmap["absolute_"].get_bool() {
    //     fighter.FighterStatusUniqProcessDamage_check_hit_stop_delay(hashmap);
    // }
    FighterUtil::cheer_damage(fighter.module_accessor);
    fighter.check_ryu_final_damage_03(L2CValue::Bool(true));
    let release_action = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    if release_action == *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_GROUND_TO_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), false);
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_AIR));
        fighter.global_table[PREV_SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
        GroundModule::set_correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_STOP_RELEASE_ACTION_NONE, *FIGHTER_STATUS_DAMAGE_WORK_INT_STOP_RELEASE_ACTION);
    let mut damage_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    let mut start_frame = 0.0;
    if damage_motion_kind == hash40("damage_fly_roll") {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
            damage_motion_kind = hash40("damage_fly_n");
        }
    }
    let damage_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    if damage_lr != 0.0 {
        if damage_lr * PostureModule::lr(fighter.module_accessor) >= 0.0 {
            PostureModule::set_lr(fighter.module_accessor, damage_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
        else if [*FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
            PostureModule::set_lr(fighter.module_accessor, damage_lr);
            PostureModule::update_rot_y_lr(fighter.module_accessor);   
        }
        else {
            if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY
            || (status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
                && damage_motion_kind != hash40("wall_damage")
                && MotionModule::motion_kind(fighter.module_accessor) != hash40("wall_damage"))
            {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_KNOCKOUT) {
                    let lr = PostureModule::lr(fighter.module_accessor);
                    TurnModule::set_turn(fighter.module_accessor, Hash40::new("back_damage"), lr, false, false, true);
                    PostureModule::reverse_lr(fighter.module_accessor);
                    let back_damage_effective_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("back_damage_effective_frame"));
                    WorkModule::set_int(fighter.module_accessor, back_damage_effective_frame, *FIGHTER_INSTANCE_WORK_ID_INT_BACK_DAMAGE_EFFECTIVE_FRAME);
                }
            }
        }
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_DAMAGE_LR);
    }
    if damage_motion_kind != hash40("invalid") {
        if damage_motion_kind == hash40("wall_damage") {
            start_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("wall_damage_start_frame"));
            if MotionModule::is_flag_start_1_frame_from_motion_kind(fighter.module_accessor, Hash40::new("wall_damage")) {
                start_frame -= 1.0;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
            if fighter.global_table[LEAVE_STOP_CALLBACK].get_bool() {
                let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(fighter.global_table[LEAVE_STOP_CALLBACK].get_ptr());
                damage_motion_kind = callable(fighter, L2CValue::U64(damage_motion_kind)).get_u64();
            }
        }
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(damage_motion_kind), start_frame, 1.0, false, 0.0, false, false);
        if status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL {
            if [*FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR].contains(&status_kind) {
                let is_pierce = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_TO_PIERCE);
                let rate = fighter.calc_damage_motion_rate(L2CValue::U64(damage_motion_kind), L2CValue::F32(start_frame), L2CValue::Bool(is_pierce)).get_f32();
                MotionModule::set_rate(fighter.module_accessor, rate);
                let damage_fly_angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose().get_i32();
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 360.0, MotionNodeRotateCompose{_address: damage_fly_angle_compose as u8});
                WorkModule::set_float(fighter.module_accessor, damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
                WorkModule::set_int64(fighter.module_accessor, hash40("invalid") as i64, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
                // <HDR>
                check_asdi(fighter);
                // </HDR>
                return 0.into();
            }
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                let damage_fly_angle_compose = fighter.sub_FighterStatusDamage_get_damage_fly_angle_compose().get_i32();
                let damage_fly_angle = FighterUtil::set_damage_fly_angle(fighter.module_accessor, 0.0, 1.0, 180.0, MotionNodeRotateCompose{_address: damage_fly_angle_compose as u8});
                WorkModule::set_float(fighter.module_accessor, damage_fly_angle, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_ROT_ANGLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_FLY_ROLL_SET_ANGLE);
            }
            let mut cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(damage_motion_kind), true);
            if cancel_frame <= 0.0 {
                cancel_frame = MotionModule::end_frame(fighter.module_accessor);
            }
            let reaction_frame_mul_speed_up = fighter.reaction_frame_mul_speed_up().get_f32();
            if 0.0 < reaction_frame_mul_speed_up {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINISH_CAMERA_TARGET) {
                    let something = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x255c556cd3);
                    let mut frame = reaction_frame_mul_speed_up - something;
                    frame %= cancel_frame;
                    if 0.0 < frame {
                        MotionModule::set_frame(fighter.module_accessor, frame, true);
                    }
                }
                else {
                    let rate = cancel_frame / reaction_frame_mul_speed_up;
                    MotionModule::set_rate(fighter.module_accessor, rate);
                }
            }
        }
        WorkModule::set_int64(fighter.module_accessor, hash40("invalid") as i64, *FIGHTER_STATUS_DAMAGE_WORK_INT_MOTION_KIND);
    }
    // <HDR>
    check_asdi(fighter);
    // </HDR>
    0.into()
}

unsafe extern "C" fn check_asdi(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR // prevents ASDI on wall bounces
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U // prevents ASDI on ceiling bounces
    && fighter.global_table[STATUS_KIND] != FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D // prevents ASDI on ground bounces
    && !(fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR && VarModule::is_flag(fighter.battle_object, vars::common::status::IS_SPIKE)) // prevents ASDI on grounded tumble-inducing spikes
    {
        let hashmap = fighter.local_func__fighter_status_damage_2();
        let sdi_mul = hashmap["stop_delay_"].get_f32();
        // get stick x/y length
        // uses cstick's value if cstick is on (for Double Stick DI)
        let stick_x = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            ControlModule::get_sub_stick_x(fighter.module_accessor)
        }
        else {
            ControlModule::get_stick_x(fighter.module_accessor)
        };
        let stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
            ControlModule::get_sub_stick_y(fighter.module_accessor)
        }
        else {
            ControlModule::get_stick_y(fighter.module_accessor)
        };
        // get base asdi distance
        let base_asdi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("hit_stop_delay_auto_mul"));
        // mul sdi_mul by hit_stop_delay_auto_mul = total sdi
        let asdi = sdi_mul * base_asdi;
        // mul stick x/y by total sdi
        let asdi_x = asdi * stick_x;
        let asdi_y = asdi * stick_y;
        // get current pos
        let mut pos = Vector3f {
            x: PostureModule::pos_x(fighter.module_accessor),
            y: PostureModule::pos_y(fighter.module_accessor),
            z: PostureModule::pos_z(fighter.module_accessor)
        };
        // add asdi x/y to pos
        pos.x += asdi_x;
        pos.y += asdi_y;
        PostureModule::set_pos(fighter.module_accessor, &Vector3f{x: pos.x, y: pos.y, z: pos.z});
        // make sure we can enter tech/missed tech on f1 of damage fly statuses (vanilla only allows them starting on f3)
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_ENABLE_DOWN);
    }
}

#[skyline::hook(replace = L2CFighterCommon_ftStatusUniqProcessDamage_init_common)]
unsafe fn ftstatusuniqprocessdamage_init_common(fighter: &mut L2CFighterCommon) {
    let reaction_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME);
    // println!("reaction frame: {}", reaction_frame);
    fighter.clear_lua_stack();
    lua_args!(fighter, 0xba5d667d4 as u64);
    sv_information::damage_log_value(fighter.lua_state_agent);
    let damage_speed_x = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value speed x probably: {}", damage_speed_x);
    fighter.clear_lua_stack();
    lua_args!(fighter, 0xbd2d15742 as u64);
    sv_information::damage_log_value(fighter.lua_state_agent);
    let damage_speed_y = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value speed y probably: {}", damage_speed_y);
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("attr"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let attr = fighter.pop_lua_stack(1).get_u64();
    // println!("damage log value attr: {}", attr);
    let _status = StatusModule::status_kind(fighter.module_accessor);
    // this isn't used in anyhthing???
    if !(0 < reaction_frame as i32) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_SPEED_UP);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME);
        WorkModule::set_float(fighter.module_accessor, reaction_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME_LAST);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DAMAGE_FLY_AIR);
        }
    }
    fighter.clear_lua_stack();
    lua_args!(fighter, hash40("angle"));
    sv_information::damage_log_value(fighter.lua_state_agent);
    let angle = fighter.pop_lua_stack(1).get_f32();
    // println!("damage log value angle: {}", angle);
    let degrees = angle.to_degrees();
    let meteor_vector_min = WorkModule::get_param_int(fighter.module_accessor, hash40("battle_object"), hash40("meteor_vector_min"));
    let meteor_vector_max = WorkModule::get_param_int(fighter.module_accessor, hash40("battle_object"), hash40("meteor_vector_max"));
    if degrees >= meteor_vector_min as f32
    && degrees <= meteor_vector_max as f32 {
        VarModule::on_flag(fighter.battle_object, vars::common::status::IS_SPIKE);
    }
    // println!("degrees: {}", degrees);
    // let speed_vector = sv_math::vec2_length(damage_speed_x, damage_speed_y);
    // println!("speed vector: {}", speed_vector);
    // if speed_vector >= 3.5 {
    //fighter.FighterStatusDamage_init_damage_speed_up(reaction_frame.into(), degrees.into(), false.into());
    // }
    let damage_cliff_no_catch_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_cliff_no_catch_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_cliff_no_catch_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CLIFF_NO_CATCH_FRAME);
    let cursor_fly_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("cursor_fly_speed"));
    // println!("cursor_fly_speed: {}", cursor_fly_speed);
    let pop1squared = damage_speed_x * damage_speed_x;
    // println!("pop1squared: {}", pop1squared);
    let pop2squared = damage_speed_y * damage_speed_y;
    // println!("pop2squared: {}", pop2squared);
    let combined = pop1squared + pop2squared;
    let cursor_fly_speed_squared = cursor_fly_speed * cursor_fly_speed;
    // println!("cursor_fly_speed_squared: {}", cursor_fly_speed_squared);
    if cursor_fly_speed_squared < combined {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_CURSOR);
        let cursor_fly_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("cursor_fly_frame"));
        WorkModule::set_int(fighter.module_accessor, cursor_fly_frame, *FIGHTER_INSTANCE_WORK_ID_INT_CURSOR_FRAME);
    }
    let damage_fly_attack_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_attack_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_attack_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ATTACK_DISABLE_FRAME);
    let damage_fly_escape_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("damage_fly_escape_frame"));
    WorkModule::set_int(fighter.module_accessor, damage_fly_escape_frame, *FIGHTER_STATUS_DAMAGE_WORK_INT_ESCAPE_DISABLE_FRAME);
    if [
        hash40("collision_attr_paralyze"),
        hash40("collision_attr_paralyze_ghost")
    ].contains(&attr) {
        let invalid_paralyze_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("invalid_paralyze_frame"));
        WorkModule::set_float(fighter.module_accessor, invalid_paralyze_frame, *FIGHTER_INSTANCE_WORK_ID_INT_INVALID_PARALYZE_FRAME);
    }
}