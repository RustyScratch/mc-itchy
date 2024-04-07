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

use crate::{
    build_context::TargetContext,
    comment::CommentBuilder,
    custom_block::{CustomBlockBuilder, CustomFuncCallBuilder},
    stack::StackBuilder,
    uid::Uid,
};

#[derive(Debug, Clone, PartialEq)]
pub enum StackOrValue {
    Value(BlockInputValue),
    Stack(StackBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputBuilder {
    pub shadow: ShadowInputType,
    pub values: Vec<Option<StackOrValue>>,
}

impl BlockInputBuilder {
    pub fn new() -> BlockInputBuilder {
        BlockInputBuilder {
            shadow: ShadowInputType::NoShadow,
            values: vec![],
        }
    }

    pub fn set_shadow(&mut self, shadow: ShadowInputType) -> &mut Self {
        self.shadow = shadow;
        self
    }

    pub fn add_input(&mut self, input: Option<StackOrValue>) -> &mut Self {
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
        let mut b = BlockInputBuilder::new();
        b.set_shadow(ShadowInputType::Shadow)
            .add_input(Some(StackOrValue::Value(value)));
        b
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::NoShadow)
    ///     .input(Some(StackOrValue::Stack(stack)))
    /// ```
    pub fn stack(stack: StackBuilder) -> Self {
        let mut b = BlockInputBuilder::new();
        b.set_shadow(ShadowInputType::NoShadow)
            .add_input(Some(StackOrValue::Stack(stack)));
        b
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::Shadow)
    ///     .input(Some(StackOrValue::Stack(value)))
    /// ```
    pub fn shadow_stack(stack: StackBuilder) -> Self {
        let mut b = BlockInputBuilder::new();
        b.set_shadow(ShadowInputType::Shadow)
            .add_input(Some(StackOrValue::Stack(stack)));
        b
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::ShadowObscured)
    ///     .input(Some(StackOrValue::Stack(stack)))
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn stack_with_value_obscured(stack: StackBuilder, value: BlockInputValue) -> Self {
        let mut b = BlockInputBuilder::new();
        b.set_shadow(ShadowInputType::ShadowObscured)
            .add_input(Some(StackOrValue::Stack(stack)))
            .add_input(Some(StackOrValue::Value(value)));
        b
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
                            let Block::VarList(vl) = s_builded.remove(&first_block_uid).unwrap()
                            else {
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

impl Default for BlockInputBuilder {
    fn default() -> Self {
        BlockInputBuilder {
            shadow: ShadowInputType::NoShadow,
            values: vec![],
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
        &mut self,
        key: K,
        block_input_builder: BlockInputBuilder,
    ) -> &mut Self {
        self.inputs.insert(key.into(), block_input_builder);
        self
    }

    pub fn add_field<S: Into<String>>(
        &mut self,
        key: S,
        block_field_builder: BlockFieldBuilder,
    ) -> &mut Self {
        self.fields.insert(key.into(), block_field_builder);
        self
    }

    pub fn set_opcode(&mut self, opcode: OpCode) -> &mut Self {
        self.opcode = opcode;
        self
    }

    pub fn set_comment(&mut self, comment: Option<CommentBuilder>) -> &mut Self {
        self.comment = comment;
        self
    }

    pub fn set_inputs(&mut self, inputs: HashMap<String, BlockInputBuilder>) -> &mut Self {
        self.inputs = inputs;
        self
    }

    pub fn set_fields(&mut self, fields: HashMap<String, BlockFieldBuilder>) -> &mut Self {
        self.fields = fields;
        self
    }

    pub fn set_mutation(&mut self, mutation: BlockMutation) -> &mut Self {
        self.mutation = Some(mutation);
        self
    }

    pub fn set_shadow(&mut self, is_shadow: bool) -> &mut Self {
        self.shadow = is_shadow;
        self
    }

    pub fn set_x(&mut self, x: Option<f64>) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: Option<f64>) -> &mut Self {
        self.y = y;
        self
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub(crate) fn build(
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

        BlockNormal {
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
        }
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
    pub value: String,
    pub kind: FieldKind,
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

    pub fn set_value(&mut self, value: String) -> &mut Self {
        self.value = value;
        self
    }
    pub fn set_kind(&mut self, kind: FieldKind) -> &mut Self {
        self.kind = kind;
        self
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
        .cloned()
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
    pub kind: ListOrVariable,
    pub from: VarListFrom,
    pub name: String,
    pub x: f64,
    pub y: f64,
    pub comment: Option<CommentBuilder>,
}

impl BlockVarListBuilder {
    pub fn global_var<S: Into<String>>(name: S) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: ListOrVariable::Variable,
            name: name.into(),
            from: VarListFrom::Global,
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn global_list<S: Into<String>>(name: S) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: ListOrVariable::List,
            name: name.into(),
            from: VarListFrom::Global,
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn sprite_var<S: Into<String>>(name: S) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: ListOrVariable::Variable,
            from: VarListFrom::Sprite,
            name: name.into(),
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn sprite_list<S: Into<String>>(name: S) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: ListOrVariable::List,
            from: VarListFrom::Sprite,
            name: name.into(),
            x: 0.,
            y: 0.,
            comment: None,
        }
    }

    pub fn set_kind(&mut self, kind: ListOrVariable) -> &mut Self {
        self.kind = kind;
        self
    }

    pub fn set_from(&mut self, from: VarListFrom) -> &mut Self {
        self.from = from;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
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

    pub fn set_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn set_comment(&mut self, comment: Option<CommentBuilder>) -> &mut Self {
        self.comment = comment;
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
        .cloned()
        .unwrap_or(Uid::new("__unknown__"));
        if let Some(comment) = comment {
            let comment_uid = Uid::generate();
            let mut comment = comment.build();
            comment.block_id = Some(my_uid.clone().into_inner());
            comment_buff.insert(comment_uid, comment);
        }

        BlockVarListReporterTop {
            kind,
            name,
            id: varlist_id.into_inner(),
            x: x.into(),
            y: y.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBuilder {
    Normal(BlockNormalBuilder),
    CustomBlock(CustomBlockBuilder),
    CustomBlockCall(CustomFuncCallBuilder),
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
            BlockBuilder::CustomBlock(f) => {
                let b = f.build(my_uid, comment_buff, final_stack, target_context);
                Block::Normal(b)
            }
            BlockBuilder::VarList(vl) => {
                let b = vl.build(my_uid, comment_buff, target_context);
                Block::VarList(b)
            }
            BlockBuilder::CustomBlockCall(fc) => {
                let b = fc.build(my_uid, comment_buff, final_stack, target_context);
                Block::Normal(b)
            }
        }
    }

    fn new_inputs_height(
        &self,
        data: &crate::stack::BlockHeightData,
        inputs: &HashMap<String, BlockInputBuilder>,
    ) -> f64 {
        let mut heights = Vec::new();

        for (code, input) in inputs {
            if !code.starts_with("SUBSTACK") {
                for value in &input.values {
                    if let Some(StackOrValue::Stack(stack)) = value {
                        heights.push(
                            stack.calc_block_height(data, true) + data.input_block_nest_height,
                        );
                    }
                    if let Some(StackOrValue::Value(_)) = value {
                        heights.push(data.input_block_height);
                    }
                }
            }
        }

        heights
            .into_iter()
            .fold(0., |acc, h| if acc > h { acc } else { h })
    }

    pub fn calc_block_height(&self, data: &crate::stack::BlockHeightData, is_input: bool) -> f64 {
        match self {
            BlockBuilder::Normal(n) => {
                if !is_input {
                    let flag_opcodes = |opcode: &OpCode| -> bool {
                        if opcode == "control_start_as_clone" {
                            return true;
                        }
                        if opcode.starts_with("event_when") {
                            return true;
                        }
                        return false;
                    };

                    if flag_opcodes(&n.opcode) {
                        return data.event_block_height;
                    }
                }

                let new_inputs_height = self.new_inputs_height(data, &n.inputs);

                if is_input {
                    return new_inputs_height;
                } else {
                    let mut base_height = if new_inputs_height > data.block_height {
                        new_inputs_height
                    } else {
                        data.block_height
                    };
                    for (code, input) in &n.inputs {
                        if code.starts_with("SUBSTACK") {
                            base_height += data.block_nest_height;
                            for value in &input.values {
                                if let Some(StackOrValue::Stack(stack)) = value {
                                    base_height += stack.calc_block_height(data, false);
                                }
                            }
                        }
                    }
                    return base_height;
                };
            }
            BlockBuilder::CustomBlock(_) => data.custom_block_height,
            BlockBuilder::CustomBlockCall(c) => {
                let new_inputs_height =
                    self.new_inputs_height(data, &c.args.clone().into_iter().collect());

                return if new_inputs_height > data.block_height {
                    new_inputs_height
                } else {
                    data.block_height
                };
            }
            BlockBuilder::VarList(_) => data.input_block_height,
        }
    }
}
