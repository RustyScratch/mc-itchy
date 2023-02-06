use std::collections::HashMap;

use sb_sbity::{
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, BlockNormal,
        BlockVarListReporterTop, ListOrVariable, ShadowInputType, UidOrValue,
    },
    comment::Comment,
    string_hashmap::StringHashMap,
    value::OpCode,
};

use crate::{build_context::TargetContext, comment::CommentBuilder, stack::StackBuilder, uid::Uid};

#[derive(Debug, Clone, PartialEq)]
pub enum StackOrValue {
    Value(BlockInputValue),
    Stack(StackBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputBuilder {
    shadow: ShadowInputType,
    values: Vec<Option<StackOrValue>>,
}

impl BlockInputBuilder {
    pub fn new() -> BlockInputBuilder {
        BlockInputBuilder {
            shadow: ShadowInputType::NoShadow,
            values: vec![],
        }
    }

    pub fn shadow(mut self, shadow: ShadowInputType) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn input(mut self, input: Option<StackOrValue>) -> Self {
        self.values.push(input);
        self
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::Shadow)
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn value(value: BlockInputValue) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::Shadow)
            .input(Some(StackOrValue::Value(value)))
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::NoShadow)
    ///     .input(Some(StackOrValue::Stack(stack)))
    /// ```
    pub fn stack(stack: StackBuilder) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::NoShadow)
            .input(Some(StackOrValue::Stack(stack)))
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::ShadowObscured)
    ///     .input(Some(StackOrValue::Stack(stack)))
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn stack_with_value_obscured(stack: StackBuilder, value: BlockInputValue) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::ShadowObscured)
            .input(Some(StackOrValue::Stack(stack)))
            .input(Some(StackOrValue::Value(value)))
    }

    pub fn build(
        self,
        this_block_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockInput {
        let BlockInputBuilder { shadow, values } = self;
        let mut values_b: Vec<Option<UidOrValue>> = vec![];
        for value in values {
            match value {
                Some(StackOrValue::Value(v)) => values_b.push(Some(UidOrValue::Value(v))),
                Some(StackOrValue::Stack(s)) => {
                    let first_block_uid = Uid::generate();
                    let mut s_builded = s.build(&first_block_uid, comment_buff, target_context);
                    let first_block = s_builded.get_mut(&first_block_uid).unwrap();
                    match first_block {
                        Block::Normal(n) => {
                            n.parent = Some(this_block_uid.clone().into_inner());
                            n.top_level = false;
                            n.x = None;
                            n.y = None;
                        }
                        Block::VarList(_) => {
                            let Block::VarList(vl) = s_builded.remove(&first_block_uid).unwrap() else {
                                unreachable!()
                            };
                            let BlockVarListReporterTop { kind, name, id, .. } = vl;
                            values_b.push(Some(UidOrValue::Value(match kind {
                                ListOrVariable::Variable => BlockInputValue::Variable { name, id },
                                ListOrVariable::List => BlockInputValue::List { name, id },
                            })))
                        }
                    }
                    final_stack.extend(s_builded);
                    values_b.push(Some(UidOrValue::Uid(first_block_uid.into_inner())))
                }
                None => values_b.push(None),
            }
        }
        BlockInput {
            shadow,
            inputs: values_b,
        }
    }
}

/// Raw block creation
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockNormalBuilder {
    opcode: OpCode,
    comment: Option<CommentBuilder>,
    inputs: HashMap<String, BlockInputBuilder>,
    fields: HashMap<String, BlockFieldBuilder>,
    mutation: Option<BlockMutation>,
    shadow: bool,
    x: Option<f64>,
    y: Option<f64>,
}

impl BlockNormalBuilder {
    pub fn new<O: Into<OpCode>>(opcode: O) -> BlockNormalBuilder {
        BlockNormalBuilder {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn add_input<K: Into<String>>(
        mut self,
        key: K,
        block_input_builder: BlockInputBuilder,
    ) -> Self {
        self.inputs.insert(key.into(), block_input_builder);
        self
    }

    pub fn add_field<S: Into<String>>(
        mut self,
        key: S,
        block_field_builder: BlockFieldBuilder,
    ) -> Self {
        self.fields.insert(key.into(), block_field_builder);
        self
    }

    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.shadow = is_shadow;
        self
    }

    pub fn comment(mut self, comment_builder: CommentBuilder) -> Self {
        self.comment = Some(comment_builder);
        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn mut_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn mutation(mut self, mutation: BlockMutation) -> Self {
        self.mutation = Some(mutation);
        self
    }

    fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
        let BlockNormalBuilder {
            opcode,
            comment,
            inputs,
            fields,
            shadow,
            mutation,
            x,
            y,
        } = self;
        // let mut inputs_b: HashMap<String, BlockInput> = HashMap::default();
        // for (key, input) in inputs {
        //     inputs_b.insert(key, input.build(comment_buff, final_stack, &my_uid));
        // }
        let inputs: HashMap<String, BlockInput> = inputs
            .into_iter()
            .map(|(key, input)| {
                (
                    key,
                    input.build(my_uid, comment_buff, final_stack, target_context),
                )
            })
            .collect();
        let fields: HashMap<String, BlockField> = fields
            .into_iter()
            .map(|(key, field)| (key, field.build(target_context)))
            .collect();
        let comment = match comment {
            Some(comment) => {
                let comment_uid = Uid::generate();
                let mut comment = comment.build();
                comment.block_id = Some(my_uid.clone().into_inner());
                comment_buff.insert(comment_uid.clone(), comment);
                Some(comment_uid.into_inner())
            }
            None => None,
        };
        let block_b = BlockNormal {
            opcode,
            comment,
            next: None,
            parent: None,
            inputs: StringHashMap(inputs),
            fields: StringHashMap(fields),
            shadow,
            top_level: false,
            mutation,
            x: x.map(|x| x.into()),
            y: y.map(|y| y.into()),
        };
        block_b
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    NoRef,
    #[default]
    NoRefMaybe,
    Broadcast,
    SpriteVariable,
    GlobalVariable,
    SpriteList,
    GlobalList,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockFieldBuilder {
    value: String,
    kind: FieldKind,
}

impl BlockFieldBuilder {
    pub fn new_with_kind(value: String, kind: FieldKind) -> BlockFieldBuilder {
        BlockFieldBuilder { value, kind }
    }

    pub fn new(value: String) -> BlockFieldBuilder {
        BlockFieldBuilder {
            value,
            kind: FieldKind::NoRefMaybe,
        }
    }

    pub fn build(self, target_context: &TargetContext) -> BlockField {
        let BlockFieldBuilder { value, kind } = self;
        let value = value.into();
        let sb_sbity::value::Value::Text(ref value_str) = value else {
            unreachable!("why the hell the `not text` would be here")
        };
        let id = match kind {
            FieldKind::NoRef => return BlockField::NoId { value },
            FieldKind::NoRefMaybe => return BlockField::WithId { value, id: None },

            FieldKind::Broadcast => target_context.all_broadcasts,
            FieldKind::SpriteVariable => target_context.this_sprite_vars,
            FieldKind::GlobalVariable => target_context.global_vars,
            FieldKind::SpriteList => target_context.this_sprite_lists,
            FieldKind::GlobalList => target_context.global_lists,
        }
        .get(value_str)
        .map(|uid| uid.clone())
        .unwrap_or_else(|| Uid::new("__unknown__"));
        BlockField::WithId {
            value,
            id: Some(id.into_inner()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarListFrom {
    Global,
    Sprite,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockVarListBuilder {
    kind: ListOrVariable,
    from: VarListFrom,
    name: String,
    x: f64,
    y: f64,
    comment: Option<CommentBuilder>,
}

impl BlockVarListBuilder {
    pub fn global<Name: Into<String>>(
        var_or_list: ListOrVariable,
        name: Name,
    ) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: var_or_list,
            name: name.into(),
            from: VarListFrom::Global,
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn sprite<Name: Into<String>>(
        var_or_list: ListOrVariable,
        name: Name,
    ) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: var_or_list,
            from: VarListFrom::Sprite,
            name: name.into(),
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn comment(mut self, comment: CommentBuilder) -> BlockVarListBuilder {
        self.comment = Some(comment);
        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn mut_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> BlockVarListReporterTop {
        let BlockVarListBuilder {
            kind,
            from,
            name,
            x,
            y,
            comment,
        } = self;
        let varlist_id = match (&kind, from) {
            (ListOrVariable::Variable, VarListFrom::Global) => target_context.global_vars,
            (ListOrVariable::Variable, VarListFrom::Sprite) => target_context.this_sprite_vars,
            (ListOrVariable::List, VarListFrom::Global) => target_context.global_lists,
            (ListOrVariable::List, VarListFrom::Sprite) => target_context.this_sprite_lists,
        }
        .get(&name)
        .map(|uid| uid.clone())
        .unwrap_or(Uid::new("__unknown__"));
        if let Some(comment) = comment {
            let comment_uid = Uid::generate();
            let mut comment = comment.build();
            comment.block_id = Some(my_uid.clone().into_inner());
            comment_buff.insert(comment_uid.clone(), comment);
        }
        let block_varlist_b = BlockVarListReporterTop {
            kind,
            name,
            id: varlist_id.into_inner(),
            x: x.into(),
            y: y.into(),
        };
        block_varlist_b
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBuilder {
    Normal(BlockNormalBuilder),
    VarList(BlockVarListBuilder),
}

impl BlockBuilder {
    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> Block {
        match self {
            BlockBuilder::Normal(n) => {
                let b = n.build(my_uid, comment_buff, final_stack, target_context);
                Block::Normal(b)
            }
            BlockBuilder::VarList(vl) => {
                let b = vl.build(my_uid, comment_buff, target_context);
                Block::VarList(b)
            }
        }
    }
}
