use std::collections::HashMap;

use sb_sbity::{
    asset::{Costume, Sound},
    block::Block,
    broadcast::Broadcast,
    comment::Comment,
    list::List,
    string_hashmap::StringHashMap,
    target::{RotationStyle, Sprite, Stage, Target, VideoState},
    variable::Variable,
};

use crate::{
    asset::{CostumeBuilder, SoundBuilder},
    build_context::TargetContext,
    comment::CommentBuilder,
    data::{ListBuilder, VariableBuilder},
    resource::Resource,
    stack::StackBuilder,
    uid::Uid,
};
use crate::{build_context::GlobalVarListContext, custom_block::CustomBlockTy};

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct TargetBuilder {
    pub name:            String,
    pub variables:       HashMap<String, VariableBuilder>,
    pub lists:           HashMap<String, ListBuilder>,
    pub broadcasts:      HashMap<String, Uid>,
    pub block_stackes:   Vec<StackBuilder>,
    pub comments:        HashMap<Uid, Comment>,
    pub costumes:        Vec<CostumeBuilder>,
    pub sounds:          Vec<SoundBuilder>,
    pub current_costume: u64,
    pub layer_order:     u64,
    pub volume:          f64,
    pub custom_blocks:   Vec<CustomBlockTy>,
}

impl TargetBuilder {
    pub fn set_name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = name.into();
        self
    }

    pub fn set_costume(&mut self, index: u64) -> &mut Self {
        self.current_costume = index;
        self
    }

    pub fn set_layer_order(&mut self, layer: u64) -> &mut Self {
        self.layer_order = layer;
        self
    }

    pub fn add_variable<S: Into<String>>(
        &mut self,
        name: S,
        variable_builder: VariableBuilder,
    ) -> &mut Self {
        self.variables.insert(name.into(), variable_builder);
        self
    }

    pub fn add_list<S: Into<String>>(&mut self, name: S, list_builder: ListBuilder) -> &mut Self {
        self.lists.insert(name.into(), list_builder);
        self
    }

    pub fn add_broadcast<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.broadcasts.insert(name.into(), Uid::generate());
        self
    }

    pub fn add_block_stack(&mut self, stack_builder: StackBuilder) -> &mut Self {
        self.block_stackes.push(stack_builder);
        self
    }

    pub fn add_comment(&mut self, comment_builder: CommentBuilder) -> &mut Self {
        let comment = comment_builder.build();
        self.comments.insert(Uid::generate(), comment);
        self
    }

    pub fn add_costume(&mut self, costume_builder: CostumeBuilder) -> &mut Self {
        self.costumes.push(costume_builder);
        self
    }

    pub fn add_sound(&mut self, sound_builder: SoundBuilder) -> &mut Self {
        self.sounds.push(sound_builder);
        self
    }

    /// When global_varlist_buf suppose to be none when the Stage itself is building.
    /// The .1 return value is going to return Some when stage itself is also building.
    pub fn build(
        self,
        res_buf: &mut Vec<Resource>,
        global_varlist_ctx: Option<&GlobalVarListContext>,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> (Target, Option<GlobalVarListContext>) {
        let TargetBuilder {
            name,
            variables,
            lists,
            broadcasts,
            block_stackes,
            comments,
            costumes,
            sounds,
            current_costume,
            layer_order,
            volume,
            custom_blocks,
        } = self;
        let variables: HashMap<String, Variable> = variables
            .into_iter()
            .map(|(var_name, var_builder)| {
                let (var, uid) = var_builder.build(var_name);
                (uid.into_inner(), var)
            })
            .collect();
        let lists: HashMap<String, List> = lists
            .into_iter()
            .map(|(list_name, list_builder)| {
                let (list, uid) = list_builder.build(list_name);
                (uid.into_inner(), list)
            })
            .collect();
        let broadcasts: HashMap<String, Broadcast> = broadcasts
            .into_iter()
            .map(|(name, uid)| (uid.into_inner(), Broadcast { name }))
            .collect();

        let mut comments = comments;
        let variable_ctx: HashMap<String, Uid> = variables
            .iter()
            .map(|(uid, var)| (var.name.clone(), Uid::new(uid)))
            .collect();
        let list_ctx: HashMap<String, Uid> = lists
            .iter()
            .map(|(uid, list)| (list.name.clone(), Uid::new(uid)))
            .collect();
        let blocks: HashMap<String, Block> = block_stackes
            .into_iter()
            .flat_map(|stack_builder| {
                let builded_stack = stack_builder.build(
                    &Uid::generate(),
                    &mut comments,
                    &match global_varlist_ctx {
                        Some(global_varlist_ctx) => TargetContext {
                            global_vars: &global_varlist_ctx.vars,
                            global_lists: &global_varlist_ctx.lists,
                            this_sprite_vars: &variable_ctx,
                            this_sprite_lists: &list_ctx,
                            all_broadcasts,
                            custom_blocks: &custom_blocks,
                        },
                        None => TargetContext {
                            global_vars: &variable_ctx,
                            global_lists: &list_ctx,
                            this_sprite_vars: &variable_ctx,
                            this_sprite_lists: &list_ctx,
                            all_broadcasts,
                            custom_blocks: &custom_blocks,
                        },
                    },
                );
                builded_stack
                    .into_iter()
                    .map(|(uid, block)| (uid.into_inner(), block))
            })
            .collect();
        let comments: HashMap<String, Comment> = comments
            .into_iter()
            .map(|(uid, comment)| (uid.into_inner(), comment))
            .collect();
        let costumes: Vec<Costume> = costumes
            .into_iter()
            .map(|costume_builder| costume_builder.build(res_buf))
            .collect();
        let sounds: Vec<Sound> = sounds
            .into_iter()
            .map(|sound_builder| sound_builder.build(res_buf))
            .collect();
        let target = Target {
            name,
            variables: StringHashMap(variables),
            lists: StringHashMap(lists),
            broadcasts: StringHashMap(broadcasts),
            blocks: StringHashMap(blocks),
            comments: StringHashMap(comments),
            current_costume: current_costume as i64,
            costumes,
            sounds,
            layer_order: layer_order as i64,
            volume: volume.into(),
        };
        (
            target,
            match global_varlist_ctx {
                Some(_) => None,
                None => Some(GlobalVarListContext {
                    vars: variable_ctx,
                    lists: list_ctx,
                }),
            },
        )
    }
}

impl Default for TargetBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        TargetBuilder {
            name:            "".to_owned(),
            variables:       HashMap::default(),
            lists:           HashMap::default(),
            broadcasts:      HashMap::default(),
            block_stackes:   Vec::default(),
            comments:        HashMap::default(),
            costumes:        Vec::default(),
            sounds:          Vec::default(),
            current_costume: 0,
            layer_order:     0,
            volume:          100.,
            custom_blocks:   Vec::default(),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct StageBuilder {
    pub target:                  TargetBuilder,
    pub tempo:                   i64,
    pub video_state:             VideoState,
    pub video_transparency:      i64,
    custom_blocks:               HashMap<String, CustomBlockTy>,
    // Not availiable yet.
    // TODO: do this.
    // text_to_speech_language: (),
}

impl StageBuilder {
    pub fn set_set_target(&mut self, target: TargetBuilder) -> &mut Self {
        self.target = target;
        self
    }

    pub fn set_tempo(&mut self, tempo: i64) -> &mut Self {
        self.tempo = tempo;
        self
    }

    pub fn set_video_transparency(&mut self, video_transparency: i64) -> &mut Self {
        self.video_transparency = video_transparency;
        self
    }

    pub fn set_video_state(&mut self, video_state: VideoState) -> &mut Self {
        self.video_state = video_state;
        self
    }

    pub fn build(
        self,
        res_buf: &mut Vec<Resource>,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> (Stage, GlobalVarListContext) {
        let StageBuilder {
            target,
            tempo,
            video_state,
            video_transparency,
            ..
        } = self;
        let (target, Some(global_var_list)) = target.build(res_buf, None, all_broadcasts) else {
            panic!("stage suppose to return what global var they had");
        };
        let stage = Stage {
            target,
            tempo: tempo.into(),
            video_state,
            video_transparency: video_transparency.into(),
            text_to_speech_language: None,
            is_stage: true,
        };
        (stage, global_var_list)
    }
}

impl Default for StageBuilder {
    fn default() -> Self {
        StageBuilder {
            target: TargetBuilder {
                name: "Stage".to_owned(),
                ..Default::default()
            },
            tempo: 60,
            video_state: VideoState::On,
            video_transparency: 50,
            custom_blocks: HashMap::default(),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct SpriteBuilder {
    pub target:         TargetBuilder,
    pub visible:        bool,
    pub x:              f64,
    pub y:              f64,
    pub size:           f64,
    pub direction:      f64,
    pub draggable:      bool,
    pub rotation_style: RotationStyle,
}

impl SpriteBuilder {
    pub fn set_target(&mut self, target: TargetBuilder) -> &mut Self {
        self.target = target;
        self
    }

    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.visible = visible;
        self
    }

    pub fn set_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = y;
        self
    }

    pub fn set_size(&mut self, size: f64) -> &mut Self {
        self.size = size;
        self
    }

    pub fn set_direction(&mut self, direction: f64) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn set_draggable(&mut self, draggable: bool) -> &mut Self {
        self.draggable = draggable;
        self
    }

    pub fn set_rotation_style(&mut self, rotation_style: RotationStyle) -> &mut Self {
        self.rotation_style = rotation_style;
        self
    }

    pub fn build(
        self,
        res_buf: &mut Vec<Resource>,
        global_varlist_buf: &GlobalVarListContext,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> Sprite {
        let SpriteBuilder {
            target,
            visible,
            x,
            y,
            size,
            direction,
            draggable,
            rotation_style,
        } = self;
        Sprite {
            target: target
                .build(res_buf, Some(global_varlist_buf), all_broadcasts)
                .0,
            visible,
            x: x.into(),
            y: y.into(),
            size: size.into(),
            direction: direction.into(),
            draggable,
            rotation_style,
            is_stage: false,
        }
    }
}

impl Default for SpriteBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        SpriteBuilder {
            target:         TargetBuilder::default(),
            visible:        true,
            x:              0.,
            y:              0.,
            size:           100.,
            direction:      90.,
            draggable:      false,
            rotation_style: RotationStyle::AllAround,
        }
    }
}
