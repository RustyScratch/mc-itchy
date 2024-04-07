//! Collections of blocks in Scratch
//!
//! Blocks that ended with menu is a visual menu in scratch.
//! It's not required to be use in function argument in here
//! which might introduce some invalid argument to function that normally requires a menu in the editor.
//!
//! Some reserved input (you shouldn't try to name anything with thing in this list):
//!  - `"_random_"`
//!  - `"_mouse_"`
//!  - `"_random_"`
//!  - `"_mouse_"`
//!  - `"_edge_"`
//!  - `"_stage_"`
//!

use std::collections::HashMap;

use crate::{
    block::{BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder, BlockVarListBuilder},
    custom_block::{CustomBlockBuilder, CustomFuncCallBuilder},
    opcode::StandardOpCode,
    prelude::{BlockBuilder, FieldKind},
    stack::StackBuilder,
};
use sb_sbity::block::{BlockMutation, BlockMutationEnum};

// Control
// Event
// Looks
// Motion
// Operator
// Sensing
// Sound
// Data
// My Blocks

type Bfb = BlockFieldBuilder;
type Bib = BlockInputBuilder;

// Control =====================================================================
pub fn wait(duration: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_wait);
        b.add_input("DURATION", duration);
        b
    })
}

pub fn repeat(times: Bib, to_repeat: Option<Bib>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_repeat);
        b.add_input("TIMES", times);
        if let Some(to_repeat) = to_repeat {
            b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

pub fn forever(to_repeat: Option<Bib>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_forever);
        if let Some(to_repeat) = to_repeat {
            b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

pub fn if_(condition: Bib, if_true: Option<Bib>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_if);
        b.add_input("CONDITION", condition);
        if let Some(if_true) = if_true {
            b.add_input("SUBSTACK", if_true);
        }
        b
    })
}

pub fn if_else(condition: Bib, if_true: Option<Bib>, if_false: Option<Bib>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_if_else);
        b.add_input("CONDITION", condition);
        if let Some(if_true) = if_true {
            b.add_input("SUBSTACK", if_true);
        }
        if let Some(if_false) = if_false {
            b.add_input("SUBSTACK2", if_false);
        }
        b
    })
}

pub fn wait_until(condition: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_wait_until);
        b.add_input("CONDITION", condition);
        b
    })
}

pub fn repeat_until(condition: Bib, to_repeat: Option<Bib>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_if_else);
        b.add_input("CONDITION", condition);
        if let Some(to_repeat) = to_repeat {
            b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

/// <br/>
/// `stop_option` Accepts:
///  - `"this script"` and `has_next` should be `false`
///  - `"other scripts in sprite"` and `has_next` should be `true`
///  - `"all"` and `has_next` should be `false`
pub fn stop(stop_option: Bfb, has_next: bool) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_stop);
        b.add_field("STOP_OPTION", stop_option)
            .set_mutation(BlockMutation {
                tag_name: "mutation".to_owned(),
                children: vec![],
                mutation_enum: BlockMutationEnum::ControlStop { hasnext: has_next },
            });
        b
    })
}

pub fn when_i_start_as_a_clone() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::control_start_as_clone,
    ))
}

/// <br/>
/// Accepts:
///  - Sprite name
pub fn create_clone_of(sprite: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_create_clone_of);
        b.add_input("CLONE_OPTION", sprite);
        b
    })
}

/// Uses as an argument to [`create_clone_of`]
///
/// Accepts:
///  - Sprite name
pub fn create_clone_of_menu(sprite: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::control_create_clone_of);
        b.add_field("CLONE_OPTION", sprite).set_shadow(true);
        b
    })
}

pub fn delete_this_clone() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::control_delete_this_clone,
    ))
}

// Event =======================================================================
pub fn when_flag_clicked() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::event_whenflagclicked,
    ))
}

/// <br/>
/// Accepts:
///  - `"any"`
///  - `"space"`
///  - `"left arrow"`
///  - `"right arrow"`
///  - `"up arrow"`
///  - `"down arrow"`
///  - Number 0 - 9
///  - Letter a - z
pub fn when_key_pressed(key: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_whenkeypressed);
        b.add_field("KEY_OPTION", key);
        b
    })
}

pub fn when_this_sprite_clicked() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::event_whenthisspriteclicked,
    ))
}

/// <br/>
/// Accepts:
///  - Backdrop name
pub fn when_backdrop_switches_to(backdrop: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_whenbackdropswitchesto);
        b.add_field("BACKDROP", backdrop);
        b
    })
}

/// <br/>
/// Accepts:
/// - `"LOUDNESS"`
/// - `"TIMER"`
pub fn when_greater_than(variable: Bfb, value: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_whengreaterthan);
        b.add_input("VALUE", value)
            .add_field("WHENGREATERTHANMENU", variable);
        b
    })
}

pub fn when_broadcast_received(broadcast: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_whenbroadcastreceived);
        b.add_field("BROADCAST_OPTION", broadcast);
        b
    })
}

pub fn broadcast(broadcast: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_broadcast);
        b.add_input("BROADCAST_INPUT", broadcast);
        b
    })
}

pub fn broadcast_and_wait(broadcast: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::event_broadcastandwait);
        b.add_input("BROADCAST_INPUT", broadcast);
        b
    })
}

// Looks =======================================================================
pub fn think(message: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_think);
        b.add_input("MESSAGE", message);
        b
    })
}

pub fn think_for_secs(message: Bib, secs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_thinkforsecs);
        b.add_input("MESSAGE", message).add_input("SECS", secs);
        b
    })
}

pub fn say(message: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_say);
        b.add_input("MESSAGE", message);
        b
    })
}

pub fn say_for_secs(message: Bib, secs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_sayforsecs);
        b.add_input("MESSAGE", message).add_input("SECS", secs);
        b
    })
}

/// <br/>
/// Accepts:
///  - Costume name
pub fn switch_costume_to(costume: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_switchcostumeto);
        b.add_input("COSTUME", costume);
        b
    })
}

/// Uses as an argument to [`switch_costume_to`]
///
/// Accepts:
///  - Costume name
pub fn costume_menu(costume: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_costume);
        b.add_field("COSTUME", costume).set_shadow(true);
        b
    })
}

pub fn next_costume() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::looks_nextcostume))
}

/// <br/>
/// Accepts:
///  - Backdrop name
pub fn switch_backdrop_to(backdrop: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_switchbackdropto);
        b.add_input("BACKDROP", backdrop);
        b
    })
}

/// Uses as an argument to [`switch_backdrop_to`]
///
/// Accepts:
///  - Backdrop name
pub fn backdrop_menu(backdrop: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_backdrops);
        b.add_field("BACKDROP", backdrop).set_shadow(true);
        b
    })
}

pub fn next_backdrop() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::looks_nextbackdrop))
}

pub fn change_size_by(by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_changesizeby);
        b.add_input("CHANGE", by);
        b
    })
}

pub fn set_size_to(to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_setsizeto);
        b.add_input("SIZE", to);

        b
    })
}

/// <br/>
/// Accepts
///  - `"COLOR"`
///  - `"FISHEYE"`
///  - `"WHIRL"`
///  - `"PIXELATE"`
///  - `"MOSAIC"`
///  - `"BRIGHTNESS"`
///  - `"GHOST"`
pub fn change_looks_effect_by(effect: Bfb, by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_changeeffectby);
        b.add_input("CHANGE", by).add_field("EFFECT", effect);
        b
    })
}

/// <br/>
/// Accepts
///  - `"COLOR"`
///  - `"FISHEYE"`
///  - `"WHIRL"`
///  - `"PIXELATE"`
///  - `"MOSAIC"`
///  - `"BRIGHTNESS"`
///  - `"GHOST"`
pub fn set_looks_effect_to(effect: Bfb, to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_seteffectto);
        b.add_input("TO", to).add_field("EFFECT", effect);
        b
    })
}

pub fn clear_graphic_effects() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::looks_cleargraphiceffects,
    ))
}

pub fn show() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::looks_show))
}

pub fn hide() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::looks_hide))
}

/// <br/>
/// Accepts:
///  - `"front"`
///  - `"back"`
pub fn go_to_layer(layer: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_gotofrontback);
        b.add_field("FRONT_BACK", layer);
        b
    })
}

/// <br/>
/// `layer` Accepts:
///  - `"foward"`
///  - `"backward"`
pub fn change_layer(layer: Bfb, by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_goforwardbackwardlayers);
        b.add_input("NUM", by).add_field("FORWARD_BACKWORD", layer);
        b
    })
}

/// <br/>
/// Accepts:
/// - `"number"`
/// - `"name"`
pub fn costume(return_type: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_costumenumbername);
        b.add_field("NUMBER_NAME", return_type);
        b
    })
}

/// <br/>
/// Accepts:
/// - `"number"`
/// - `"name"`
pub fn backdrop(return_type: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::looks_backdropnumbername);
        b.add_field("NUMBER_NAME", return_type);
        b
    })
}

pub fn size() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::looks_size))
}

// Motion ======================================================================
pub fn move_steps(steps: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_movesteps);
        b.add_input("STEPS", steps);
        b
    })
}

pub fn turn_right(degress: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_turnright);
        b.add_input("DEGREES", degress);
        b
    })
}

pub fn turn_left(degress: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_turnleft);
        b.add_input("DEGREES", degress);
        b
    })
}

/// <br/>
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` go to mouse position
///  - `"_random_"` go to random position
pub fn go_to(to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_goto);
        b.add_input("TO", to);
        b
    })
}

/// Uses as an argument to [`goto`]
///
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` go to mouse position
///  - `"_random_"` go to random position
pub fn go_to_menu(to: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_goto_menu);
        b.add_field("TO", to).set_shadow(true);
        b
    })
}

pub fn goto_xy(x: Bib, y: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_gotoxy);
        b.add_input("X", x).add_input("Y", y);
        b
    })
}

/// <br/>
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` glide to mouse position
///  - `"_random_"` glide to random position
pub fn glide_to(duration_secs: Bib, to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_gotoxy);
        b.add_input("SECS", duration_secs).add_input("TO", to);
        b
    })
}

/// Uses as an argument for [`glide_to`] in `to`
///
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` glide to mouse position
///  - `"_random_"` glide to random position
pub fn glide_to_menu(to: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_glideto_menu);
        b.add_field("TO", to).set_shadow(true);
        b
    })
}

pub fn glide_to_xy(dur: Bib, x: Bib, y: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_glidesecstoxy);
        b.add_input("SECS", dur).add_input("X", x).add_input("Y", y);
        b
    })
}

pub fn point_in_direction(direction: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_pointindirection);
        b.add_input("DIRECTION", direction);
        b
    })
}

/// <br/>
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` glide to mouse position
pub fn point_towards(towards: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_pointtowards);
        b.add_input("TOWARDS", towards);
        b
    })
}

/// Uses as an argument for [`point_towards`]
///
/// Accepts:
///  - Sprite name
///  - `"_mouse_"` glide to mouse position
pub fn point_towards_menu(towards: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_pointtowards_menu);
        b.add_field("TOWARDS", towards).set_shadow(true);
        b
    })
}

pub fn set_x(x: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_setx);
        b.add_input("X", x);
        b
    })
}

pub fn set_y(y: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_setx);
        b.add_input("Y", y);
        b
    })
}

pub fn change_x_by(by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_changexby);
        b.add_input("DX", by);
        b
    })
}

pub fn change_y_by(by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_changeyby);
        b.add_input("DY", by);
        b
    })
}

pub fn if_on_edge_bounce() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::motion_ifonedgebounce,
    ))
}

/// <br/>
/// Accepts:
///  - `"left-right"`
///  - `"don't rotate"`
///  - `"all around"`
pub fn set_rotation_style(style: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::motion_setrotationstyle);
        b.add_field("STYLE", style);
        b
    })
}

pub fn x_position() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::motion_xposition))
}

pub fn y_position() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::motion_yposition))
}

pub fn direction() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::motion_direction))
}

// Operators ===================================================================
pub fn add(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_add);
        b.add_input("NUM1", lhs).add_input("NUM2", rhs);
        b
    })
}

pub fn sub(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_subtract);
        b.add_input("NUM1", lhs).add_input("NUM2", rhs);
        b
    })
}

pub fn mul(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_multiply);
        b.add_input("NUM1", lhs).add_input("NUM2", rhs);
        b
    })
}

pub fn div(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_divide);
        b.add_input("NUM1", lhs).add_input("NUM2", rhs);
        b
    })
}

pub fn random(from: Bib, to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_random);
        b.add_input("FROM", from).add_input("TO", to);
        b
    })
}

pub fn less_than(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_lt);
        b.add_input("OPERAND1", lhs).add_input("OPERAND2", rhs);
        b
    })
}

pub fn greater_than(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_gt);
        b.add_input("OPERAND1", lhs).add_input("OPERAND2", rhs);
        b
    })
}

pub fn equals(lhs: Bib, rhs: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_equals);
        b.add_input("OPERAND1", lhs).add_input("OPERAND2", rhs);
        b
    })
}

pub fn and(a: Bib, b: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut bl = BlockNormalBuilder::new(StandardOpCode::operator_and);
        bl.add_input("OPERAND1", a).add_input("OPERAND2", b);
        bl
    })
}

pub fn or(a: Bib, b: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut bl = BlockNormalBuilder::new(StandardOpCode::operator_or);
        bl.add_input("OPERAND1", a).add_input("OPERAND2", b);
        bl
    })
}

pub fn not(val: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_not);
        b.add_input("OPERAND", val);
        b
    })
}

pub fn join(a: Bib, b: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut bl = BlockNormalBuilder::new(StandardOpCode::operator_join);
        bl.add_input("STRING1", a).add_input("STRING2", b);
        bl
    })
}

pub fn letter_of(idx: Bib, text: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_letter_of);
        b.add_input("LETTER", idx).add_input("STRING", text);
        b
    })
}

pub fn length_of(text: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_length);
        b.add_input("STRING", text);
        b
    })
}

pub fn contains(text: Bib, contains: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_contains);
        b.add_input("STRING1", text).add_input("STRING2", contains);
        b
    })
}

pub fn modulo(dividend: Bib, divisor: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_mod);
        b.add_input("NUM1", dividend).add_input("NUM2", divisor);
        b
    })
}

pub fn round(val: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_round);
        b.add_input("NUM", val);
        b
    })
}

/// <br/>
/// `op` Accepts:
///  - `"abs"`
///  - `"floor"`
///  - `"ceiling"`
///  - `"sqrt"`
///  - `"sin"`
///  - `"cos"`
///  - `"tan"`
///  - `"asin"`
///  - `"acos"`
///  - `"atan"`
///  - `"ln"`
///  - `"log"`
///  - `"e ^"`
///  - `"10 ^"`
pub fn math_op(op: Bfb, val: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::operator_mathop);
        b.add_input("NUM", val).add_field("OPERATOR", op);
        b
    })
}

// Sensing =====================================================================

/// <br/>
/// Accepts:
///  - Sprite name
///  - `"_mouse_"`
///  - `"_edge_"`
pub fn touching(what: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_touchingobject);
        b.add_input("TOUCHINGOBJECTMENU", what);
        b
    })
}

/// Uses as an argument to [`touching`]
///
/// Accepts:
///  - Sprite name
///  - `"_mouse_"`
///  - `"_edge_"`
pub fn touching_menu(what: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_touchingobjectmenu);
        b.add_field("TOUCHINGOBJECTMENU", what).set_shadow(true);
        b
    })
}

pub fn touching_color(color: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_touchingcolor);
        b.add_input("COLOR", color);
        b
    })
}

pub fn color_touching_color(color_a: Bib, color_b: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_coloristouchingcolor);
        b.add_input("COLOR", color_a).add_input("COLOR2", color_b);
        b
    })
}

/// <br/>
/// Accepts:
///  - Sprite name
///  - `"_mouse_"`
pub fn distance_to(what: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_coloristouchingcolor);
        b.add_input("DISTANCETOMENU", what);
        b
    })
}

/// Uses as an argument to [`distance_to`]
///
/// Accepts:
///  - Sprite name
///  - `"_mouse_"`
pub fn distance_to_menu(what: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_coloristouchingcolor);
        b.add_field("DISTANCETOMENU", what).set_shadow(true);
        b
    })
}

pub fn ask_and_wait(prompt_message: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_askandwait);
        b.add_input("QUESTION", prompt_message);
        b
    })
}

pub fn answer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_answer))
}

/// <br/>
/// Accepts:
///  - `"any"`
///  - `"space"`
///  - `"left arrow"`
///  - `"right arrow"`
///  - `"up arrow"`
///  - `"down arrow"`
///  - Number 0 - 9
///  - Letter a - z
pub fn key_pressed(key: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_keypressed);
        b.add_input("KEY_OPTION", key);
        b
    })
}

/// Uses as an argument to [`key_pressed`]
///
/// Accepts:
///  - `"any"`
///  - `"space"`
///  - `"left arrow"`
///  - `"right arrow"`
///  - `"up arrow"`
///  - `"down arrow"`
///  - Number 0 - 9
///  - Letter a - z
pub fn key_menu(key: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_keyoptions);
        b.add_input("KEY_OPTION", key).set_shadow(true);
        b
    })
}

pub fn mouse_down() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_mousedown))
}

pub fn mouse_x() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_mousex))
}

pub fn mouse_y() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_mousey))
}

/// <br/>
/// Accepts:
///  - `"not draggable"`
///  - `"draggable"`
pub fn set_drag_mode(mode: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_setdragmode);
        b.add_field("DRAG_MODE", mode);
        b
    })
}

pub fn loudness() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_loudness))
}

pub fn timer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_timer))
}

pub fn reset_timer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_resettimer))
}

/// <br/>
/// `what` Accepts:
///   - Sprite name
///   - `"_stage_"`
///
/// If `what` is "_stage_"
/// <br/>
///    `var` Accepts:
///      - Stage's custom variable name
///      - `"backdrop #"`
///      - `"backdrop name"`
///      - `"volume"`
///
/// Else `what` is a Sprite name
/// <br/>
///    `var` Accepts:
///      - That sprite's custome variable name
///      - `"x position"`
///      - `"y position"`
///      - `"direction"`
///      - `"costume #"`
///      - `"costume name"`
///      - `"size"`
///      - `"volume"`
pub fn var_of(var: Bfb, what: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_of);
        b.add_input("OBJECT", what).add_field("PROPERTY", var);
        b
    })
}

/// Uses as an argument to [`var_of`]
///
/// `what` Accepts:
///   - Sprite name
///   - `"_stage_"`
pub fn var_of_object_menu(what: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_of_object_menu);
        b.add_field("OBJECT", what).set_shadow(true);
        b
    })
}

/// <br/>
/// Accepts:
///  - `"SECOND"`
///  - `"MINUTE"`
///  - `"HOUR"`
///  - `"DAYOFWEEK"`
///  - `"DATE"`
///  - `"MONTH"`
///  - `"YEAR"`
pub fn current_datetime(format: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sensing_current);
        b.add_field("CURRENTMENU", format);
        b
    })
}

pub fn days_since_2000() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        StandardOpCode::sensing_dayssince2000,
    ))
}

pub fn username() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sensing_username))
}

// Sound =======================================================================

/// <br/>
/// Accepts:
///  - Sound name
pub fn play_sound_until_done(sound: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_playuntildone);
        b.add_input("SOUND_MENU", sound);
        b
    })
}

/// <br/>
/// Accepts:
///  - Sound name
pub fn play_sound(sound: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_play);
        b.add_input("SOUND_MENU", sound);
        b
    })
}

/// Uses as an argument to [`play_sound_until_done`] and [`play_sound`]
///
/// Accepts:
///  - Sound name
pub fn sound_menu(sound: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_sounds_menu);
        b.add_field("SOUND_MENU", sound).set_shadow(true);
        b
    })
}

pub fn stop_all_sound() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sound_stopallsounds))
}

/// <br/>
/// Accepts:
///  - `"PITCH"`
///  - `"PAN"`
pub fn change_sound_effect_by(effect: Bfb, by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_changeeffectby);
        b.add_input("VALUE", by).add_field("EFFECT", effect);
        b
    })
}

/// <br/>
/// Accepts:
///  - `"PITCH"`
///  - `"PAN"`
pub fn set_sound_effect_to(effect: Bfb, to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_seteffectto);
        b.add_input("VALUE", to).add_field("EFFECT", effect);
        b
    })
}

pub fn clear_sound_effects() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sound_cleareffects))
}

pub fn set_volume_to(volume: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_setvolumeto);
        b.add_input("VOLUME", volume);
        b
    })
}

pub fn change_volume_by(by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::sound_changeeffectby);
        b.add_input("VOLUME", by);
        b
    })
}

pub fn volume() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(StandardOpCode::sound_volume))
}

// Data ========================================================================
pub fn sprite_var<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::sprite_var(name))
}

pub fn sprite_list<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::sprite_list(name))
}

pub fn global_var<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::global_var(name))
}

pub fn global_list<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::global_list(name))
}

pub fn set_var_to(var: Bfb, to: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_setvariableto);
        b.add_input("VALUE", to).add_field("VARIABLE", var);
        b
    })
}

pub fn change_var_by(var: Bfb, by: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_changevariableby);
        b.add_input("VALUE", by).add_field("VARIABLE", var);
        b
    })
}

pub fn show_var(var: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_showvariable);
        b.add_field("VARIABLE", var);
        b
    })
}

pub fn hide_var(var: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_hidevariable);
        b.add_field("VARIABLE", var);
        b
    })
}

pub fn add_to_list(list: Bfb, item: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_addtolist);
        b.add_input("ITEM", item).add_field("LIST", list);
        b
    })
}

pub fn delete_in_list(list: Bfb, idx: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_deleteoflist);
        b.add_input("INDEX", idx).add_field("LIST", list);
        b
    })
}

pub fn delete_all_in_list(list: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_deletealloflist);
        b.add_field("LIST", list);
        b
    })
}

pub fn insert_in_list(list: Bfb, idx: Bib, item: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_insertatlist);
        b.add_input("INDEX", idx)
            .add_input("ITEM", item)
            .add_field("LIST", list);
        b
    })
}

pub fn replace_in_list(list: Bfb, idx: Bib, item: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_replaceitemoflist);
        b.add_input("INDEX", idx)
            .add_input("ITEM", item)
            .add_field("LIST", list);
        b
    })
}

pub fn item_in_list(list: Bfb, idx: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_itemoflist);
        b.add_input("INDEX", idx).add_field("LIST", list);
        b
    })
}

pub fn count_of_item_in_list(list: Bfb, item: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_itemnumoflist);
        b.add_input("ITEM", item).add_field("LIST", list);
        b
    })
}

pub fn length_of_list(list: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_lengthoflist);
        b.add_field("LIST", list);
        b
    })
}

pub fn list_contains(list: Bfb, item: Bib) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_listcontainsitem);
        b.add_input("ITEM", item).add_field("LIST", list);
        b
    })
}

pub fn show_list(list: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_showlist);
        b.add_field("LIST", list);
        b
    })
}

pub fn hide_list(list: Bfb) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::data_hidelist);
        b.add_field("LIST", list);
        b
    })
}

// My Blocks ========================================================================
pub fn define_custom_block<S: Into<String>>(name: S) -> StackBuilder {
    let custom_block = CustomBlockBuilder::new(name);

    StackBuilder {
        stack: vec![BlockBuilder::CustomBlock(custom_block)],
    }
}

pub fn call_custom_block<S: Into<String>, T: Into<String>>(
    name: S,
    args: HashMap<T, Bib>,
) -> StackBuilder {
    let mut custom_func_call = CustomFuncCallBuilder::new();
    custom_func_call.set_name(name.into());
    for (k, v) in args {
        custom_func_call.add_input(k.into(), v);
    }

    StackBuilder {
        stack: vec![BlockBuilder::CustomBlockCall(custom_func_call)],
    }
}

pub fn custom_block_var_boolean<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::argument_reporter_boolean);
        b.add_field(
            "VALUE",
            BlockFieldBuilder::new_with_kind(name.into(), FieldKind::NoRefMaybe),
        );
        b
    })
}

pub fn custom_block_var_string_number<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(StandardOpCode::argument_reporter_string_number);
        b.add_field(
            "VALUE",
            BlockFieldBuilder::new_with_kind(name.into(), FieldKind::NoRefMaybe),
        );
        b
    })
}

// Translate ========================================================================
pub fn translate_to<S: Into<String>>(string: Bib, lang: S) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new("translate_getTranslate");
        b.add_input("WORDS", string);
        b.add_input(
            "LANGUAGE",
            BlockInputBuilder::shadow_stack(StackBuilder::start({
                let mut b = BlockNormalBuilder::new("translate_menu_languages");
                b.add_field(
                    "languages",
                    BlockFieldBuilder::new_with_kind(lang.into(), FieldKind::NoRefMaybe),
                );
                b.set_shadow(true);
                b
            })),
        );
        b
    })
}

pub fn get_viewer_language() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new("translate_getViewerLanguage"))
}
