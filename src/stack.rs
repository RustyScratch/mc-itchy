use std::collections::HashMap;

use sb_sbity::{block::Block, comment::Comment};

use crate::{
    block::{BlockBuilder, BlockNormalBuilder, BlockVarListBuilder},
    build_context::TargetContext,
    uid::Uid,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct StackBuilder {
    pub stack: Vec<BlockBuilder>,
}

impl StackBuilder {
    pub fn new() -> StackBuilder {
        StackBuilder { stack: Vec::new() }
    }

    pub fn start(block: BlockNormalBuilder) -> StackBuilder {
        StackBuilder::start_with_capacity(1, BlockBuilder::Normal(block))
    }

    /// Varlist is a reporter. You shouldn't continue after this... but nothing disllowed you.
    pub fn start_varlist(block: BlockVarListBuilder) -> StackBuilder {
        StackBuilder::start_with_capacity(1, BlockBuilder::VarList(block))
    }

    pub fn start_with_capacity(capacity: usize, block: BlockBuilder) -> StackBuilder {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(block);
        StackBuilder { stack }
    }

    pub fn with_capacity(capacity: usize) -> StackBuilder {
        let stack = Vec::with_capacity(capacity);
        StackBuilder { stack }
    }

    pub fn next(mut self, mut next_stack: StackBuilder) -> StackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    pub fn set_top_block_position(&mut self, x: f64, y: f64) -> &mut Self {
        match &mut self.stack[0] {
            BlockBuilder::Normal(n) => {
                n.set_pos(Some(x), Some(y));
            }
            BlockBuilder::VarList(vl) => {
                vl.set_pos(x, y);
            }
            BlockBuilder::CustomBlock(vl) => {
                vl.set_pos(x, y);
            }
            BlockBuilder::CustomBlockCall(vl) => {
                vl.set_pos(Some(x), Some(y));
            }
        }
        self
    }

    pub fn build(
        self,
        first_block_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> HashMap<Uid, Block> {
        let mut stack_b: HashMap<Uid, Block> = HashMap::default();
        let mut self_stack_iter = self.stack.into_iter();
        let first_block = self_stack_iter.next().unwrap().build(
            first_block_uid,
            comment_buff,
            &mut stack_b,
            target_context,
        );

        match first_block {
            Block::Normal(mut first_block) => {
                first_block.top_level = true;
                first_block.x = Some(first_block.x.unwrap_or_default());
                first_block.y = Some(first_block.y.unwrap_or_default());
                let mut previous_block = (first_block, first_block_uid.clone());
                for block_builder2 in self_stack_iter {
                    let (mut block1, block1_uid) = previous_block;
                    let block2_uid = Uid::generate();
                    let Block::Normal(mut block2) = block_builder2.build(
                        &block2_uid,
                        comment_buff,
                        &mut stack_b,
                        target_context,
                    ) else {
                        unreachable!("BlockVarList shouldn't exist here")
                    };

                    block1.next = Some(block2_uid.clone().into_inner());
                    block2.parent = Some(block1_uid.clone().into_inner());

                    previous_block = (block2, block2_uid);

                    stack_b.insert(block1_uid, Block::Normal(block1));
                }
                stack_b.insert(previous_block.1, Block::Normal(previous_block.0));
                stack_b
            }
            Block::VarList(vl) => {
                stack_b.insert(first_block_uid.clone(), Block::VarList(vl));
                stack_b
            }
        }
    }

    pub fn calc_block_height(&self, data: &BlockHeightData, is_input: bool) -> f64 {
        self.stack.iter().fold(0.0, |acc, block| {
            acc + block.calc_block_height(data, is_input)
        }) + if is_input { 0. } else { data.block_bump }
    }
}

pub struct BlockHeightData {
    pub input_block_height: f64,
    pub input_block_nest_height: f64,
    pub block_height: f64,
    pub block_nest_height: f64,
    pub block_bump: f64,
    pub custom_block_height: f64,
    pub event_block_height: f64,
}

impl Default for BlockHeightData {
    fn default() -> Self {
        BlockHeightData {
            input_block_height: 80.,
            input_block_nest_height: 16.,
            block_height: 96.,
            block_nest_height: 64.,
            block_bump: 16.,
            custom_block_height: 184.,
            event_block_height: 146.,
        }
    }
}
