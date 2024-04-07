//! Hello there!
//! Nothing much here be here are some note when building:
//!  - Scratch won't load if there are layer order collision; make sure all your Sprite have different layer order; Stage should be 0.
//!  - Scratch also won't load if there are no costume in Sprite or Stage; make sure to have atleast one!
//!
//! More documentation will made later if a lot of people actually uses this crate.
//!
//! Feel free to ask in github discussion. I will make sure to answer all of you questions if no one do so!

pub mod asset;
pub mod block;
pub mod comment;
pub mod custom_block;
pub mod data;
pub mod project;
pub mod stack;
pub mod target;

pub mod opcode;
pub mod resource;
pub mod uid;

pub mod build_context;

pub mod export;
pub mod import;

// pub mod block_definer;
pub mod blocks;

pub mod prelude {
    pub use self::{
        asset::{AssetBuilder, CostumeBuilder, SoundBuilder},
        block::{
            BlockBuilder, BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder,
            BlockVarListBuilder, FieldKind, StackOrValue, VarListFrom,
        },
        build_context::{GlobalVarListContext, TargetContext},
        comment::CommentBuilder,
        data::{ListBuilder, VariableBuilder},
        opcode::StandardOpCode,
        project::ProjectBuilder,
        resource::{Resource, ResourceError},
        stack::StackBuilder,
        target::{SpriteBuilder, StageBuilder, TargetBuilder},
        uid::Uid,
    };
    use super::*;
    pub use custom_block::*;
    pub use sb_sbity::block::{BlockInputValue, ShadowInputType};
}
