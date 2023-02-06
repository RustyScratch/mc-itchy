use std::collections::HashMap;

use sb_sbity::{block::Block, comment::Comment};

use crate::{
    block::{BlockBuilder, BlockNormalBuilder, BlockVarListBuilder},
    build_context::TargetContext,
    uid::Uid,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StackBuilder {
    stack: Vec<BlockBuilder>,
}

impl StackBuilder {
    pub fn start(block: BlockNormalBuilder) -> StackBuilder {
        StackBuilder::start_with_capacity(1, BlockBuilder::Normal(block))
    }

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

    pub fn new() -> StackBuilder {
        StackBuilder { stack: Vec::new() }
    }

    pub fn next(mut self, mut next_stack: StackBuilder) -> StackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    pub fn move_head(mut self, x: f64, y: f64) -> Self {
        match &mut self.stack[0] {
            BlockBuilder::Normal(n) => {
                n.mut_pos(x, y);
            }
            BlockBuilder::VarList(vl) => {
                vl.mut_pos(x, y);
            }
        }
        self
    }

    pub fn mut_move_head(&mut self, x: f64, y: f64) -> &mut Self {
        match &mut self.stack[0] {
            BlockBuilder::Normal(n) => {
                n.mut_pos(x, y);
            }
            BlockBuilder::VarList(vl) => {
                vl.mut_pos(x, y);
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
            &first_block_uid,
            comment_buff,
            &mut stack_b,
            target_context,
        );

        match first_block {
            Block::Normal(mut first_block) => {
                first_block.top_level = true;
                first_block.x = Some(0.into());
                first_block.y = Some(0.into());
                let mut previous_block = (first_block, first_block_uid.clone());
                for block_builder2 in self_stack_iter {
                    let (mut block1, block1_uid) = previous_block;
                    let block2_uid = Uid::generate();
                    let Block::Normal(mut block2) =
                        block_builder2.build(&block2_uid, comment_buff, &mut stack_b, target_context) else {
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
}
