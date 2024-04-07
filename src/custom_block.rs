use std::collections::HashMap;

use sb_sbity::{
    block::{Block, BlockInputValue, BlockMutation, BlockMutationEnum, BlockNormal},
    comment::Comment,
    value::ValueWithBool,
};

use crate::{
    build_context::TargetContext,
    prelude::{
        BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder, CommentBuilder, StandardOpCode,
        Uid,
    },
    stack::StackBuilder,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomBlockBuilder {
    name: String,
    x: Option<f64>,
    y: Option<f64>,
    comment: Option<CommentBuilder>,
}

impl CustomBlockBuilder {
    pub fn new<S: Into<String>>(name: S) -> CustomBlockBuilder {
        CustomBlockBuilder {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn set_comment(&mut self, comment: CommentBuilder) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = Some(x);
        self
    }

    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn set_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
        let CustomBlockBuilder {
            comment,
            name,
            x,
            y,
        } = self;

        let ty = target_context
            .custom_blocks
            .iter()
            .find(|ty| ty.name() == name)
            .expect("CustomBlock not found")
            .clone();

        // let (params_id, custom_func_ty) =
        //     generate_func_input_block(my_uid.clone(), &args, final_stack, warp);

        let mut define_prototype = BlockNormalBuilder::new(StandardOpCode::procedures_prototype);
        for (arg_id, ty) in ty.vars() {
            define_prototype.add_input(
                arg_id.into_inner(),
                BlockInputBuilder::shadow_stack(StackBuilder::start({
                    let mut b = BlockNormalBuilder::new(match &ty {
                        CustomBlockInputType::Text(_) => unreachable!(),
                        CustomBlockInputType::StringOrNumber(_) => {
                            StandardOpCode::argument_reporter_string_number
                        }
                        CustomBlockInputType::Boolean(_) => {
                            StandardOpCode::argument_reporter_boolean
                        }
                    });
                    b.add_field(
                        "VALUE",
                        BlockFieldBuilder::new_with_kind(
                            ty.name(),
                            crate::prelude::FieldKind::NoRefMaybe,
                        ),
                    );
                    b.set_shadow(true);
                    b
                })),
            );
        }
        define_prototype.set_mutation(BlockMutation {
            tag_name: "mutation".into(),
            children: vec![],
            mutation_enum: ty.define_mutation(),
        });
        define_prototype.set_shadow(true);

        let mut define_block = BlockNormalBuilder::new(StandardOpCode::procedures_definition);
        define_block.add_input(
            "custom_block",
            BlockInputBuilder::shadow_stack(StackBuilder::start(define_prototype)),
        );

        define_block.set_pos(x, y);

        define_block.set_comment(comment);

        let block = define_block.build(my_uid, comment_buff, final_stack, target_context);

        block
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CustomBlockInputType {
    Text(String),
    StringOrNumber(String),
    Boolean(String),
}

impl CustomBlockInputType {
    pub fn name(&self) -> String {
        match self {
            CustomBlockInputType::Text(name) => name.clone(),
            CustomBlockInputType::StringOrNumber(name) => name.clone(),
            CustomBlockInputType::Boolean(name) => name.clone(),
        }
    }
}

// pub type CustomBlockTy = (BlockMutationEnum, HashMap<String, Uid>);
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomBlockTy {
    ty: Vec<(Option<String>, CustomBlockInputType)>,
    warp: bool,
}

impl CustomBlockTy {
    pub fn new(args: Vec<CustomBlockInputType>, warp: bool) -> CustomBlockTy {
        let ty = args
            .iter()
            .map(|arg| {
                let name = match arg {
                    CustomBlockInputType::Text(_) => None,
                    _ => Some(Uid::generate().into_inner()),
                };
                (name, arg.clone())
            })
            .collect::<Vec<_>>();
        CustomBlockTy { ty, warp }
    }

    pub fn name(&self) -> String {
        match (&self.ty).into_iter().next() {
            Some((_, CustomBlockInputType::Text(name))) => name.clone(),
            _ => "".into(),
        }
    }

    pub fn call_mutation(&self) -> BlockMutationEnum {
        BlockMutationEnum::ProceduresCall {
            proccode: self.proccode(),
            argumentids: self
                .argumentids()
                .iter()
                .map(|i| i.clone().into_inner())
                .collect(),
            warp: Some(self.warp),
        }
    }

    pub fn define_mutation(&self) -> BlockMutationEnum {
        BlockMutationEnum::ProceduresPrototype {
            proccode: self.proccode(),
            argumentids: self
                .argumentids()
                .iter()
                .map(|i| i.clone().into_inner())
                .collect(),
            argumentnames: self.argumentnames(),
            argumentdefaults: self.argumentdefaults(),
            warp: Some(self.warp),
        }
    }

    pub fn proccode(&self) -> String {
        let mut proccode: Option<String> = None;
        for (_, ty) in self.ty.iter() {
            let ty = match ty {
                CustomBlockInputType::Text(name) => name.clone(),
                CustomBlockInputType::StringOrNumber(_) => "%s".into(),
                CustomBlockInputType::Boolean(_) => "%b".into(),
            };
            match &mut proccode {
                Some(proccode) => {
                    proccode.push_str(&format!(" {ty}"));
                }
                None => {
                    proccode = Some(ty);
                }
            }
        }
        proccode.unwrap_or_default()
    }

    pub fn vars(&self) -> Vec<(Uid, CustomBlockInputType)> {
        self.ty
            .iter()
            .filter_map(|(name, ty)| match name {
                Some(name) => Some((Uid::new(name.clone()), ty.clone())),
                None => None,
            })
            .collect::<Vec<_>>()
    }

    pub fn argumentids(&self) -> Vec<Uid> {
        self.vars()
            .iter()
            .map(|(i, _)| i.clone())
            .collect::<Vec<_>>()
    }

    pub fn argumentnames(&self) -> Vec<String> {
        self.vars()
            .iter()
            .map(|(_, i)| i.name())
            .collect::<Vec<_>>()
    }

    pub fn argumentdefaults(&self) -> Vec<ValueWithBool> {
        self.vars()
            .iter()
            .map(|(_, i)| match i {
                CustomBlockInputType::Text(_) => unreachable!(),
                CustomBlockInputType::StringOrNumber(_) => ValueWithBool::Text("".into()),
                CustomBlockInputType::Boolean(_) => ValueWithBool::Bool(false),
            })
            .collect::<Vec<_>>()
    }

    pub fn warp(&self) -> bool {
        self.warp
    }
}

// pub fn generate_func_input_block(
//     parent: Uid,
//     params: &Vec<CustomBlockInputType>,
//     stack: &mut HashMap<Uid, Block>,
//     warp: bool,
// ) -> (Uid, CustomBlockTy) {
//     let this_block_id = Uid::generate();

//     let mut inputs = StringHashMap::default();

//     let mut argumentids = Vec::new();
//     let mut argumentnames = Vec::new();
//     let mut argumentdefaults = Vec::new();

//     // let mut proccode: Option<String> = None;
//     let mut proccode: Option<String> = Some("".into());

//     let mut custom_func_ty = HashMap::new();

//     for ty in params {
//         let wrapper_id = Uid::generate();
//         match ty {
//             CustomBlockInputType::Text(name) => {
//                 custom_func_ty.insert(name.clone(), wrapper_id.clone());
//                 match &mut proccode {
//                     Some(proccode) => {
//                         proccode.push_str(&format!(" {name}"));
//                     }
//                     None => {
//                         proccode = Some(name.clone());
//                     }
//                 }
//             }
//             CustomBlockInputType::StringOrNumber(name) => {
//                 custom_func_ty.insert(name.clone(), wrapper_id.clone());
//                 let (id, block, ty, default) = generate_func_input_block_var_string_number(
//                     this_block_id.clone(),
//                     name.clone(),
//                 );
//                 inputs.0.insert(
//                     wrapper_id.clone().into_inner(),
//                     BlockInput {
//                         shadow: ShadowInputType::Shadow,
//                         inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
//                     },
//                 );
//                 argumentids.push(wrapper_id.into_inner());
//                 argumentnames.push(name.clone());
//                 argumentdefaults.push(default);
//                 stack.insert(id, block);
//                 match &mut proccode {
//                     Some(proccode) => {
//                         proccode.push_str(&format!(" {ty}"));
//                     }
//                     None => {
//                         proccode = Some(ty);
//                     }
//                 }
//             }
//             CustomBlockInputType::Boolean(name) => {
//                 custom_func_ty.insert(name.clone(), wrapper_id.clone());
//                 let (id, block, ty, default) = generate_func_input_block_var_boolean(
//                     this_block_id.clone().into_inner(),
//                     name.clone(),
//                 );
//                 inputs.0.insert(
//                     wrapper_id.clone().into_inner(),
//                     BlockInput {
//                         shadow: ShadowInputType::Shadow,
//                         inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
//                     },
//                 );
//                 argumentids.push(wrapper_id.into_inner());
//                 argumentnames.push(name.clone());
//                 argumentdefaults.push(default);
//                 stack.insert(id, block);
//                 match &mut proccode {
//                     Some(proccode) => {
//                         proccode.push_str(&format!(" {ty}"));
//                     }
//                     None => {
//                         proccode = Some(ty);
//                     }
//                 }
//             }
//         }
//     }

//     let block = BlockNormal {
//         opcode: "procedures_prototype".into(),
//         next: None,
//         parent: Some(parent.into_inner()),
//         shadow: true,
//         top_level: false,
//         x: None,
//         y: None,
//         inputs: inputs,
//         fields: StringHashMap::default(),
//         mutation: Some(BlockMutation {
//             tag_name: "mutation".into(),
//             children: vec![],
//             mutation_enum: ProceduresPrototype {
//                 proccode: proccode.clone().unwrap(),
//                 argumentids: argumentids.clone(),
//                 argumentnames,
//                 argumentdefaults,
//                 warp: Some(warp),
//             },
//         }),
//         comment: None,
//     };
//     stack.insert(this_block_id.clone(), Block::Normal(block));
//     (
//         this_block_id,
//         (
//             BlockMutationEnum::ProceduresCall {
//                 proccode: proccode.unwrap(),
//                 argumentids,
//                 warp: Some(warp),
//             },
//             custom_func_ty,
//         ),
//     )
// }

// pub fn generate_func_input_block_var_boolean(
//     parent: String,
//     name: String,
// ) -> (Uid, Block, String, ValueWithBool) {
//     let this_block_id = Uid::generate();

//     let mut fields = StringHashMap::default();

//     fields.0.insert(
//         "VALUE".into(),
//         WithId {
//             id: None,
//             value: Value::Text(name.clone()),
//         },
//     );

//     let block = BlockNormal {
//         opcode: "argument_reporter_boolean".into(),
//         next: None,
//         parent: Some(parent),
//         shadow: true,
//         top_level: false,
//         x: None,
//         y: None,
//         inputs: StringHashMap::default(),
//         fields: fields,
//         mutation: None,
//         comment: None,
//     };
//     (
//         this_block_id,
//         Block::Normal(block),
//         "%b".into(),
//         ValueWithBool::Bool(false),
//     )
// }

// pub fn generate_func_input_block_var_string_number(
//     parent: Uid,
//     name: String,
// ) -> (Uid, Block, String, ValueWithBool) {
//     let this_block_id = Uid::generate();

//     let mut fields = StringHashMap::default();

//     fields.0.insert(
//         "VALUE".into(),
//         WithId {
//             id: None,
//             value: Value::Text(name.clone()),
//         },
//     );

//     let block = BlockNormal {
//         opcode: "argument_reporter_string_number".into(),
//         next: None,
//         parent: Some(parent.into_inner()),
//         shadow: true,
//         top_level: false,
//         x: None,
//         y: None,
//         inputs: StringHashMap::default(),
//         fields: fields,
//         mutation: None,
//         comment: None,
//     };
//     (
//         this_block_id,
//         Block::Normal(block),
//         "%s".into(),
//         ValueWithBool::Text("".into()),
//     )
// }

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomFuncCallBuilder {
    name: String,
    comment: Option<CommentBuilder>,
    pub(crate) args: Vec<(String, BlockInputBuilder)>,
    x: Option<f64>,
    y: Option<f64>,
}

impl CustomFuncCallBuilder {
    pub fn new() -> CustomFuncCallBuilder {
        CustomFuncCallBuilder {
            ..Default::default()
        }
    }

    pub fn add_input<K: Into<String>>(&mut self, key: K, input: BlockInputBuilder) -> &mut Self {
        self.args.push((key.into(), input));
        self
    }

    pub fn set_comment(&mut self, comment: CommentBuilder) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_args(&mut self, args: Vec<(String, BlockInputBuilder)>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
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

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
        let CustomFuncCallBuilder {
            name,
            comment,
            args,
            x,
            y,
        } = self;

        let ty = target_context
            .custom_blocks
            .iter()
            .find(|ty| ty.name() == name)
            .expect("Custom function not found")
            .clone();

        let mut call_block = BlockNormalBuilder::new(StandardOpCode::procedures_call);
        // for (key, input) in args {
        //     let arg_position = ty.argumentnames().iter().position(|i| i == &key).unwrap();
        //     let arg_id = ty.argumentids()[arg_position].clone();

        //     call_block.add_input(arg_id.into_inner(), input);
        // }
        for (id, name) in ty.vars() {
            let name = name.name();
            let arg = args.iter().find(|(key, _)| key == &name);
            let arg = match arg {
                Some((_, arg)) => arg.clone(),
                None => {
                    let defaults = ty.argumentdefaults();
                    let pos = ty.argumentnames().iter().position(|i| i == &name).unwrap();
                    let default = defaults[pos].clone();

                    BlockInputBuilder::value(match default {
                        ValueWithBool::Text(value) => BlockInputValue::String {
                            value: value.into(),
                        },
                        ValueWithBool::Number(value) => BlockInputValue::Number {
                            value: value.into(),
                        },
                        ValueWithBool::Bool(_) => continue,
                    })
                }
            };
            call_block.add_input(id.into_inner(), arg);
        }

        call_block.set_pos(x, y);

        call_block.set_comment(comment);

        call_block.set_mutation(BlockMutation {
            tag_name: "mutation".into(),
            children: vec![],
            mutation_enum: ty.call_mutation(),
        });

        let block = call_block.build(my_uid, comment_buff, final_stack, target_context);

        block
    }
}
