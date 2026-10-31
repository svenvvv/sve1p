use crate::utils::modlib::ModlibFuncs;
use crate::utils::userfuncs::BUILTIN_USERFUNCS;
use crate::{dto::e1pmodule::E1pModule, visitors::luawriter::NodeVariant::*};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Error, Write},
};

use crate::dto::ast::{Node, NodeVariant, NodeXValue};

macro_rules! expect_count {
    ($collection:expr, $count:expr) => {{
        if $collection.len() != $count {
            panic!(
                "Expected {} children, got {}:\n{:?}",
                $count,
                $collection.len(),
                $collection
            );
        }
    }};
}

macro_rules! expect {
    ($a:expr, $b:expr) => {{
        if $a != $b {
            panic!("Expected {:?},  got {:?}", $a, $b);
        }
    }};
}

macro_rules! expect_node {
    ($child:expr, $($type:ident),+) => {{
        match &$child.variant {
            $( $type(data) )|* => data,
            _ => {
                let expected = [$(stringify!($type)),+].join(" or ");
                panic!("Expected {:?}, got {:?}", expected, $child.variant);
            }
        }
    }};
}

macro_rules! get_str {
    ($data:expr) => {{
        $data
            .string
            .as_ref()
            .expect("Expected string to be present")
    }};
}

#[derive(Clone)]
enum ModuleScope {
    MAIN,
    MOD,
}

pub struct LuaWriter {
    is_main: bool,
    in_phrase: bool,
    active_scope: ModuleScope,
    hoist_expressions: Vec<String>,
    main_phrases: Vec<u32>,
    module_phrases: Vec<u32>,
    main_renamed_phrases_by_address: HashMap<u64, String>,
    module_renamed_phrases_by_address: HashMap<u64, String>,
    main_object_names: HashMap<usize, String>,
    module_object_names: HashMap<usize, String>,
    has_pushed_object_ref: bool,
    current_function: Option<(String, String)>,
}

impl LuaWriter {
    pub fn write(file: File, module: &mut E1pModule) {
        let mut writer = BufWriter::new(file);

        write!(writer, "require \"modlib\"\n").unwrap();
        write!(writer, "\n").unwrap();
        write!(writer, "ModuleName = \"{}\"\n", module.generated.name.0).unwrap();
        write!(writer, "\n").unwrap();

        // write!(writer, "ModulePhrases[ModuleName] = {{").unwrap();
        // for (i, offset) in module.phrases.indices.iter().enumerate() {
        //     if i % 5 == 0 {
        //         write!(writer, "\n  ").unwrap();
        //     }
        //     write!(writer, "Phrase_{}, ", offset).unwrap();
        // }
        // write!(writer, "\n}}\n").unwrap();

        let mut s = LuaWriter {
            is_main: module.is_main,
            active_scope: if module.is_main {
                ModuleScope::MAIN
            } else {
                ModuleScope::MOD
            },
            in_phrase: false,
            hoist_expressions: Vec::new(),
            main_phrases: module.phrases.indices.clone(),
            module_phrases: module.phrases.indices.clone(),
            main_renamed_phrases_by_address: HashMap::new(),
            module_renamed_phrases_by_address: HashMap::new(),
            main_object_names: HashMap::new(),
            module_object_names: HashMap::new(),
            has_pushed_object_ref: false,
            current_function: None,
        };

        s.write_internal(&mut writer, module)
            .expect("Failed to write");
    }

    fn write_internal<T: Write>(
        &mut self,
        writer: &mut T,
        module: &mut E1pModule,
    ) -> Result<(), Error> {
        let (ctor, dtor) = module.find_ctor();
        self.write_ctor_dtor(writer, ctor)?;

        // let node = expect_node!(module.get_root_seq_mut(), Seq, CtorPhrase, DtorPhrase);
        for child in module.generated.root_nodes.iter() {
            match &child.variant {
                Phrase(_) | CtorPhrase(_) | DtorPhrase(_) => self.write_node(writer, child, 1)?,
                Seq(_) => self.write_node(writer, child, 1)?,
                _ => {
                    println!("unexpected in toplevel {}", child.variant.get_name());
                    write!(writer, "-- ")?;
                    self.write_node(writer, child, 1)?;
                }
            }
        }
        Ok(())
    }
}

fn get_addsub_op(x: Option<&NodeXValue>) -> char {
    match x.expect("Expected X for addsub") {
        NodeXValue::Normal(x) => match x {
            0 => '+',
            1 => '-',
            2 => '*',
            3 => '/',
            _ => panic!("Unexpected op: {}", x),
        },
        NodeXValue::Offset(_) => panic!("Unexpected offset"),
    }
}

impl LuaWriter {
    fn indent_newline<T: Write>(&mut self, writer: &mut T, depth: usize) -> Result<(), Error> {
        write!(writer, "\n")?;
        if !self.in_phrase {
            write!(writer, "--")?;
        }
        write!(writer, "{}", " ".repeat(2 * depth))?;
        Ok(())
    }

    fn get_phrase_name(&self, offset: u64, scope: ModuleScope) -> String {
        match scope {
            ModuleScope::MAIN => {
                if let Some(new_name) = self.main_renamed_phrases_by_address.get(&offset) {
                    new_name.clone()
                } else {
                    format!("MainPhrase_{}", offset)
                }
            }
            ModuleScope::MOD => {
                if let Some(new_name) = self.module_renamed_phrases_by_address.get(&offset) {
                    new_name.clone()
                } else {
                    format!("Phrase_{}", offset)
                }
            }
        }
    }

    fn write_children<T: Write>(
        &mut self,
        writer: &mut T,
        nodes: &Vec<Node>,
        depth: usize,
        joinch: &str,
    ) -> Result<(), Error> {
        for (i, child) in nodes.iter().enumerate() {
            self.write_node(writer, child, depth)?;
            if i + 1 != nodes.len() {
                if joinch == "\n" {
                    self.indent_newline(writer, depth)?;
                } else {
                    write!(writer, "{}", joinch)?;
                }
            }
        }

        Ok(())
    }

    fn write_call_args<T: Write>(
        &mut self,
        writer: &mut T,
        children: &Vec<Node>,
    ) -> Result<(), Error> {
        for (i, child) in children.iter().enumerate() {
            let data = expect_node!(&child, Assign);
            expect_count!(data.children, 2);
            let _arg_dst = expect_node!(&data.children[0], ObjRef, Flag, StrFlag);
            let arg_src = &data.children[1];

            self.write_node(writer, arg_src, 0)?;
            if i + 1 != children.len() {
                write!(writer, ", ")?;
            }
        }
        Ok(())
    }

    fn write_switch_case_expr<T: Write>(
        &mut self,
        writer: &mut T,
        case_data: &Node,
        cond_var: &String,
    ) -> Result<(), Error> {
        match &case_data.variant {
            Range(range) => {
                expect_count!(range.children, 2);
                write!(writer, "(")?;
                self.write_node(writer, &range.children[0], 0)?;
                write!(writer, " <= {} and {} <= ", cond_var, cond_var)?;
                self.write_node(writer, &range.children[1], 0)?;
                write!(writer, ")")?
            }
            ConstInt(_) | ConstStr(_) | Flag(_) | GetObjAttr(_) | Object(_) => {
                write!(writer, "(")?;
                write!(writer, "{} == ", cond_var)?;
                self.write_node(writer, case_data, 0)?;
                write!(writer, ")")?;
            }
            _ => panic!(
                "Unsupported expression in switch case expr {}:\n{:?}",
                case_data.variant.get_name(),
                case_data.variant
            ),
        };
        Ok(())
    }

    fn write_function<T: Write>(
        &mut self,
        writer: &mut T,
        name: &String,
        arguments: &Vec<Node>,
        children: &Vec<Node>,
    ) -> Result<(), Error> {
        self.indent_newline(writer, 0)?;

        // if name == "AlvaresGuard:OnAttacked" {
        //     println!("DUMP {}", name);
        //     for c in children.iter() {
        //         println!("{:?}", c);
        //     }
        //     println!("END {}", name);
        // }

        let mut args_buf = BufWriter::new(Vec::new());
        self.write_children(&mut args_buf, arguments, 1, ", ")?;
        let args_str =
            String::from_utf8(args_buf.into_inner().expect("Failed to get args str")).unwrap();

        self.current_function = Some((name.clone(), args_str.clone()));

        write!(writer, "function {}({})", name, args_str)?;
        self.indent_newline(writer, 1)?;

        self.write_children(writer, children, 1, "\n")?;

        self.indent_newline(writer, 0)?;
        write!(writer, "end\n\n")?;

        self.current_function = None;

        Ok(())
    }

    fn write_node<T: Write>(
        &mut self,
        writer: &mut T,
        node: &Node,
        depth: usize,
    ) -> Result<(), Error> {
        match &node.variant {
            CtorPhrase(data) | DtorPhrase(data) => {
                expect_count!(data.children, 1);

                let body = expect_node!(data.children[0], Seq);

                self.in_phrase = true;

                let name = match &node.variant {
                    CtorPhrase(_) => "InitializeModule".to_string(),
                    DtorPhrase(_) => "ModuleErrorHandler".to_string(),
                    _ => panic!("what"),
                };
                self.write_function(writer, &name, &Vec::new(), &body.children)?;

                self.in_phrase = false;
            }

            NodeVariant::Phrase(data) => {
                let name = self.get_phrase_name(node.opcode.pos, self.active_scope.clone());

                let nchildren = data.children.len();
                if nchildren == 2 {
                    // Function definition
                    let body = expect_node!(data.children[0], Seq);
                    let arguments = expect_node!(data.children[1], Seq);

                    self.in_phrase = true;
                    self.write_function(writer, &name, &arguments.children, &body.children)?;
                    self.in_phrase = false;
                } else if nchildren == 0 {
                    // Function reference
                    write!(writer, "{}", name)?;
                } else {
                    panic!("Unexpected phrase node children count: {}", nchildren);
                }
            }
            NodeVariant::Seq(data) => {
                for child in data.children.iter() {
                    self.write_node(writer, child, depth)?;
                }
            }
            NodeVariant::Checklist(_) => {
                write!(writer, "-- TODO: checklist")?;
                self.indent_newline(writer, depth)?;
            }

            NodeVariant::Inherit(data) => {
                expect_count!(data.children, 2);
                write!(writer, "InheritTable(")?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }

            NodeVariant::SpeakPhrase(data) => {
                expect_count!(data.children, 1);
                write!(writer, "talk(")?;
                self.write_node(writer, data.children.first().unwrap(), depth)?;
                write!(writer, ")")?;
            }
            NodeVariant::SetUserPhrase(data) => {
                expect_count!(data.children, 2);
                write!(writer, "userch(")?;
                self.write_node(writer, data.children.first().unwrap(), depth)?;
                write!(writer, ", ")?;
                self.write_node(writer, data.children.last().unwrap(), depth)?;
                write!(writer, ")")?;
            }
            NodeVariant::AskChoice(data) => {
                expect_count!(data.children, 0);
                write!(writer, "askch()")?;
            }
            NodeVariant::Wait(data) => {
                expect_count!(data.children, 0);
                write!(writer, "wait()")?;
            }

            NodeVariant::UserFunc(data) => {
                expect_count!(data.children, 1);
                let args = expect_node!(&data.children[0], Seq);

                let opcode = match data.x.as_ref().unwrap() {
                    NodeXValue::Normal(x) => *x as u16,
                    NodeXValue::Offset(_) => panic!("Unexpected offset userfunc"),
                };
                let uf_def = match BUILTIN_USERFUNCS.get(&opcode) {
                    Some(uf) => uf,
                    None => panic!("Unknown userfunc by opcode {}", opcode),
                };

                if args.children.len() != uf_def.argument_types.len() {
                    panic!(
                        "Unexpected amount of arguments for userfunc {}, expected {}, got {}",
                        uf_def.name,
                        uf_def.argument_types.len(),
                        args.children.len()
                    );
                }

                write!(writer, "UserFuncs.{}(", uf_def.name)?;
                if args.children.len() > 0 {
                    self.write_children(writer, &args.children, depth, ", ")?;
                }
                write!(writer, ")")?;
            }
            NodeVariant::Call(data) => {
                expect_count!(data.children, 1);
                let args = expect_node!(&data.children[0], Seq);

                match data.x.as_ref().unwrap() {
                    NodeXValue::Normal(x) => {
                        let idx: usize = x.abs() as usize;
                        if let Some(offset) = self.main_phrases.get(idx) {
                            let name = self.get_phrase_name((*offset).into(), ModuleScope::MAIN);
                            write!(writer, "{}(", name)?;
                        } else if !self.is_main {
                            write!(writer, "{}({})", ModlibFuncs::ExecutePhrase, idx)?;
                        } else {
                            panic!("Missing main phrase by idx: {}\n{:?}", idx, args);
                        }
                    }
                    NodeXValue::Offset(x) => {
                        let idx: usize = x.abs() as usize;
                        if let Some(offset) = self.module_phrases.get(idx) {
                            let name = self.get_phrase_name((*offset).into(), ModuleScope::MOD);
                            write!(writer, "{}(", name)?;
                        } else {
                            panic!("Missing module phrase by idx: {}\n{:?}", idx, args);
                        }
                    }
                };
                self.write_call_args(writer, &args.children)?;

                write!(writer, ")")?;
            }

            NodeVariant::OnEvent(data) => {
                // TODO unimplemented
                // TODO x value

                write!(writer, "{}(", ModlibFuncs::RegisterEventHandler)?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }

            NodeVariant::OnTimerEvent(data) => {
                let mut bufwriter = BufWriter::new(Vec::new());

                let events = &data.children[0];
                let callback = &data.children[1];
                let unk = &data.children[2];

                write!(bufwriter, "local function event_callback_{}()", 123)?;
                self.indent_newline(&mut bufwriter, depth + 1)?;

                self.write_node(&mut bufwriter, callback, depth + 1)?;

                self.indent_newline(&mut bufwriter, depth)?;
                write!(bufwriter, "end")?;
                self.indent_newline(&mut bufwriter, depth)?;

                // if data.children.len() != 3 {
                //     panic!(
                //         "Unexpected children count\nEvents: {:?}\nSecond: {:?}\nThird: {:?}",
                //         events, &data.children[1], &data.children[2]
                //     );
                // }
                if data.children.len() < 3 && data.children.len() < 5 {
                    panic!("Unexpected children count: {}", data.children.len());
                }

                let timers = &data.children[0];
                match &timers.variant {
                    Add(data) => {}
                    If(data) => {}
                    Or(data) => {}
                    And(data) => {}
                    ThrowEvent(data) => {}
                    OnEvent(data) => {}
                    _ => panic!("Unsupported timer event: {:?}", timers.variant),
                }

                write!(writer, "{}(", ModlibFuncs::RegisterTimerHandler)?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
                // (Timer $-92
                //   (Add
                //     (ThrowEvent "T2")
                //     (Mul
                //       (ConstInt 24)
                //       (Flag $-1)))
                //   (SetObjAttr "HluxGone"
                //     (Object $2)
                //     (ConstInt 1))
                //   (ConstInt 0)))
                // (Timer -20
                //   (Add
                //     (ThrowEvent "T1")
                //     (ConstInt 5))
                //   (Call 14
                //     ...
                //   (ConstInt 0)
                //   (ConstInt 5)
                //   (ThrowEvent
                //     (Concat
                //       (Concat
                //         (ConstStr "\"")
                //         (Concat
                //           (ConstStr "Character ")
                //           (Concat
                //             (GetObjAttr "ININame"
                //               (Object 601))
                //             (ConstStr " left sector"))))
                //       (ConstStr "\"")))))
            }
            NodeVariant::Event(data) => {
                expect_count!(data.children, 1);
                expect_node!(&data.children[0], ThrowEvent);

                self.write_node(writer, &data.children[0], depth)?;
            }
            NodeVariant::ThrowEvent(data) => {
                if data.children.len() == 0 {
                    expect!(data.x.is_none(), true);
                    write!(writer, "throw(\"{}\")", get_str!(data))?;
                } else {
                    expect_count!(data.children, 1);
                    expect!(data.x.is_none(), true);
                    expect!(data.string.is_none(), true);

                    write!(writer, "throw(")?;
                    self.write_node(writer, &data.children[0], depth)?;
                    write!(writer, ")")?;
                }
            }
            NodeVariant::EventDelete(data) => {
                expect_count!(data.children, 1);
                expect!(data.x.is_none(), true);
                expect!(data.string.is_none(), true);

                write!(writer, "{}(", ModlibFuncs::DeleteEventHandler)?;
                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ")")?;
            }

            NodeVariant::Assign(data) => {
                expect_count!(data.children, 2);
                self.write_node(writer, data.children.first().unwrap(), depth)?;
                write!(writer, " = ")?;
                self.write_node(writer, data.children.last().unwrap(), depth)?;
            }
            NodeVariant::Concat(data) => {
                self.write_children(writer, &data.children, depth, " .. ")?;
            }
            NodeVariant::RetVal(data) => {
                expect_count!(data.children, 1);
                write!(writer, "return ")?;
                self.write_node(writer, &data.children[0], depth)?;
            }
            NodeVariant::Return(data) => {
                expect_count!(data.children, 0);
                write!(writer, "return")?;
            }
            NodeVariant::Solve(data) => {
                expect_count!(data.children, 1);
                let weights = expect_node!(&data.children[0], Seq);

                // X ignored, no idea what it does

                write!(writer, "GetWeightedRandom(")?;
                self.write_children(writer, &weights.children, depth, ", ")?;
                write!(writer, ")")?;
            }

            NodeVariant::Object(data) => {
                if data.x.is_none() {
                    panic!("Expected x child for Object");
                }
                let txt: String = match data.x.as_ref().unwrap() {
                    NodeXValue::Normal(x) => {
                        let name_maybe = self.main_object_names.get(&(*x as usize));
                        if self.is_main {
                            name_maybe
                                .expect(
                                    format!("Expected object name for main object {}", x).as_str(),
                                )
                                .clone()
                        } else {
                            name_maybe.map_or_else(|| format!("Objects[{}]", x), |v| v.clone())
                        }
                    }
                    NodeXValue::Offset(x) => {
                        if self.is_main {
                            panic!("Shouldn't access module objects in main module");
                        } else {
                            self.module_object_names
                                .get(&(*x as usize))
                                .expect("Missing module object name")
                                .clone()
                        }
                    }
                };
                write!(writer, "{}", txt)?;
            }
            NodeVariant::List(data) => {
                if data.x.is_none() {
                    panic!("Expected x child for List");
                }
                let txt = match data.x.as_ref().unwrap() {
                    NodeXValue::Normal(x) => format!("Lists[{}]", x),
                    NodeXValue::Offset(x) => format!("ModuleLists[ModuleName][{}]", x),
                };
                write!(writer, "{}", txt)?;
            }
            // NodeVariant::ListItem(data) => {
            //     expect_count!(data.children, 2);
            //     write!(writer, "table.insert(")?;
            //     self.write_children(writer, &data.children, depth, ", ")?;
            //     write!(writer, ")")?;
            // }
            NodeVariant::ListItem(data) | NodeVariant::ListAppend(data) => {
                write!(writer, "table.insert(")?;
                // List
                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ", ")?;
                if data.children.len() == 3 {
                    // Index
                    self.write_node(writer, &data.children[2], depth)?;
                    write!(writer, ", ")?;
                }
                // Element
                self.write_node(writer, &data.children[1], depth)?;
                write!(writer, ")")?;
            }
            NodeVariant::ListIsType(data) => {
                expect_count!(data.children, 1);
                let type_id = match data.x.as_ref().unwrap() {
                    NodeXValue::Normal(x) => match x {
                        61 => "table",
                        _ => panic!("Unexpected type ID: {}", x),
                    },
                    NodeXValue::Offset(x) => panic!("Unexpected offset x for ListIsType: {}", x),
                };

                write!(writer, "(type(")?;
                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ") == {})", type_id)?;
            }
            NodeVariant::ListRef(data) => {
                expect_count!(data.children, 1);
                self.write_node(writer, &data.children[0], depth)?;
            }
            NodeVariant::ListCount(data) => {
                expect_count!(data.children, 1);
                write!(writer, "#")?;
                self.write_node(writer, &data.children[0], depth)?;
            }
            NodeVariant::ListFind(data) => {
                expect_count!(data.children, 2);
                write!(writer, "ListFindIndex(")?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }
            NodeVariant::ListEraseItem(data) => {
                write!(writer, "table.remove(")?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }
            NodeVariant::ListClear(data) => {
                expect_count!(data.children, 1);
                write!(writer, "{}(", ModlibFuncs::ClearList)?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }

            NodeVariant::ObjRef(data) => {
                expect_count!(data.children, 0);
                let txt = match data.x.as_ref().expect("Expected x for ObjRef") {
                    NodeXValue::Normal(x) => {
                        if *x == 0 {
                            "self".to_string()
                        } else {
                            format!("ObjectRef_{}", x.abs())
                        }
                    }
                    NodeXValue::Offset(x) => format!("ObjectRefsOff_{}", x.abs()),
                };
                write!(writer, "{}", txt)?;
            }
            NodeVariant::PushObjectRef(data) => {
                expect_count!(data.children, 1);
                let obj = &data.children[0];
                expect_node!(obj, Object, ObjRef, GetObjAttr);
                self.write_node(writer, obj, depth)?;

                self.has_pushed_object_ref = true;
            }
            NodeVariant::PushMember(data) => {
                // expect_count!(data.children, 1);
                // 0 -- "callm"
                // 1 -- "callm once", not really sure what's the exact difference (does callm call multiple times??)
                // expect!(map_x(data.x.as_ref()), 0);
                let args = expect_node!(&data.children[0], Seq);

                if !self.has_pushed_object_ref {
                    write!(writer, "TODO_SUPERCLASS")?;
                }

                let name = get_str!(data);
                if name.0 != "~ind" {
                    // Call method by string name
                    write!(writer, ":{}", get_str!(data))?;
                } else {
                    // Call method by runtime name
                    if data.children.len() != 2 {
                        panic!("Call by string, weird child count");
                    }

                    let dyn_name = &data.children[1];

                    write!(writer, "[")?;
                    self.write_node(writer, dyn_name, depth)?;
                    write!(writer, "]")?;
                }

                // Write function call args
                write!(writer, "(")?;
                self.write_children(writer, &args.children, depth, ", ")?;
                write!(writer, ")")?;
            }
            NodeVariant::PopObjectRef(data) => {
                expect_count!(data.children, 0);
                expect!(data.x.is_none(), true);
                expect!(data.string.is_none(), true);
                self.has_pushed_object_ref = false;
            }

            NodeVariant::ListGet(data) => {
                expect_count!(data.children, 2);
                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, "[")?;
                self.write_node(writer, &data.children[1], depth)?;
                write!(writer, "]")?;
            }
            NodeVariant::DeleteAttr(data) | NodeVariant::ObjectDelete(data) => {
                expect_count!(data.children, 1);
                expect_node!(&data.children[0], Object, ObjRef, GetObjAttr);

                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ".{} = nil", get_str!(data))?;
            }
            NodeVariant::CopyObjectMethod(data) => {
                expect_count!(data.children, 3);
                expect_node!(&data.children[0], ObjRef, Object, GetObjAttr, List);
                expect_node!(&data.children[1], ObjRef, Object, GetObjAttr, List);
                let b = expect_node!(&data.children[2], Print);

                let a_method = get_str!(data);
                let b_method = get_str!(b);

                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ".{} = ", a_method)?;
                self.write_node(writer, &data.children[1], depth)?;
                write!(writer, ".{}", b_method)?;
            }

            NewObjAttr(data) | SetObjAttr(data) | NewObjectPhrase(data) => {
                println!("attr {:?}", node);
                expect_count!(data.children, 2);
                expect_node!(&data.children[0], Object, ObjRef, GetObjAttr);

                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ".{} = ", get_str!(data))?;
                self.write_node(writer, &data.children[1], depth)?;
            }
            NodeVariant::HasAttr(data) | NodeVariant::HasMethod(data) => {
                expect_count!(data.children, 1);

                let obj = &data.children[0];
                expect_node!(obj, ObjRef, Object, GetObjAttr);

                write!(writer, "(")?;
                self.write_node(writer, obj, depth)?;
                write!(writer, ".{} ~= nil)", get_str!(data))?;
            }
            NodeVariant::GetObjAttr(data) => {
                expect_count!(data.children, 1);

                let obj = &data.children[0];
                expect_node!(obj, Object, ObjRef, GetObjAttr);

                self.write_node(writer, obj, depth)?;
                write!(writer, ".{}", get_str!(data))?;
            }
            // NodeVariant::SetObjAttr(data) => {
            //     expect_count!(data.children, 2);

            //     let obj = &data.children[0];
            //     expect_node!(obj, Object, ObjRef, GetObjAttr);
            //     self.write_node(writer, obj, depth)?;

            //     write!(writer, ".{} = ", get_str!(data))?;

            //     let val = &data.children[1];
            //     self.write_node(writer, val, depth)?;
            // }
            NodeVariant::StrFlag(data) => {
                expect_count!(data.children, 0);
                let txt = match data.x.as_ref().expect("Expected x for StrFlag") {
                    NodeXValue::Normal(x) => format!("StringFlag_{}", x.abs()),
                    NodeXValue::Offset(x) => format!("StringFlagOff_{}", x.abs()),
                };
                write!(writer, "{}", txt)?;
            }

            NodeVariant::Break(data) => {
                expect_count!(data.children, 0);
                write!(writer, "return")?;
            }
            NodeVariant::If(data) => {
                let count = data.children.len();
                if count != 3 && count != 2 {
                    panic!("Unexpected children count for if: {:?}", data.children);
                }

                let condition = &data.children[0];
                let if_expr = expect_node!(data.children[1], Seq);
                let mut else_expr = None;

                if count > 2 {
                    let else_node = expect_node!(data.children[2], Seq);
                    if else_node.children.len() > 0 {
                        else_expr = Some(else_node);
                    }
                }

                // Transpile the expression into a temp buffer so that we can hoist out condition variables
                let mut bufwriter = BufWriter::new(Vec::new());

                write!(&mut bufwriter, "if ")?;
                self.write_node(&mut bufwriter, condition, depth)?;
                if self.hoist_expressions.len() > 0 {
                    write!(writer, "{}", self.hoist_expressions.pop().unwrap())?;
                    self.indent_newline(writer, depth)?;
                }
                write!(
                    writer,
                    "{}",
                    String::from_utf8(bufwriter.into_inner().unwrap()).unwrap()
                )?;

                write!(writer, " then")?;
                self.indent_newline(writer, depth + 1)?;
                self.write_children(writer, &if_expr.children, depth + 1, "\n")?;

                self.indent_newline(writer, depth)?;

                if let Some(else_node) = else_expr {
                    write!(writer, "else")?;
                    self.indent_newline(writer, depth + 1)?;
                    self.write_children(writer, &else_node.children, depth + 1, "\n")?;
                }

                self.indent_newline(writer, depth)?;
                write!(writer, "end")?;
            }
            NodeVariant::Switch(data) => {
                expect_count!(data.children, 3);

                let condition = &data.children[0];
                let cases = expect_node!(data.children[1], Seq);
                let else_expr = expect_node!(data.children[2], Seq);

                let switch_var = format!("switchCond{}", node.opcode.pos);
                write!(writer, "local {} = ", switch_var)?;
                self.write_node(writer, condition, depth)?;
                self.indent_newline(writer, depth)?;

                for (i, case_node) in cases.children.iter().enumerate() {
                    let case = expect_node!(case_node, Case);
                    expect_count!(case.children, 2);
                    let case_expr = expect_node!(case.children[0], Seq);
                    let case_then = expect_node!(case.children[1], Seq);

                    if i == 0 {
                        write!(writer, "if ")?;
                    } else {
                        self.indent_newline(writer, depth)?;
                        write!(writer, "elseif ")?;
                    }
                    for (i, child) in case_expr.children.iter().enumerate() {
                        self.write_switch_case_expr(writer, &child, &switch_var)?;
                        if i + 1 != case_expr.children.len() {
                            write!(writer, " or ")?;
                        }
                    }
                    write!(writer, " then")?;

                    self.indent_newline(writer, depth + 1)?;
                    self.write_children(writer, &case_then.children, depth + 1, "\n")?;
                }

                if else_expr.children.len() > 0 {
                    self.indent_newline(writer, depth)?;
                    write!(writer, "else")?;

                    self.indent_newline(writer, depth + 1)?;
                    self.write_children(writer, &else_expr.children, depth + 1, "\n")?;
                }

                self.indent_newline(writer, depth)?;
                write!(writer, "end")?;
            }
            NodeVariant::In(data) => {
                expect_count!(data.children, 2);

                let expr = &data.children[0];
                let list = expect_node!(&data.children[1], Seq);

                let mut bufwriter = BufWriter::new(Vec::new());
                let condition_name = format!("inCondition{}", node.opcode.pos);
                write!(&mut bufwriter, "local {} = ", condition_name)?;
                self.write_node(&mut bufwriter, expr, 0)?;

                write!(writer, "(")?;
                for (i, child) in list.children.iter().enumerate() {
                    self.write_switch_case_expr(writer, child, &condition_name)?;
                    // self.write_node(writer, expr, depth)?;
                    // write!(writer, " == ")?;
                    // expect_node_type!(child, ConstInt, ConstStr, Range);
                    // self.write_node(writer, child, depth)?;
                    if i + 1 != list.children.len() {
                        write!(writer, " or ")?;
                    }
                }
                write!(writer, ")")?;

                let to_hoist_bytes = bufwriter.into_inner().unwrap();
                if to_hoist_bytes.len() > 0 {
                    let to_add = String::from_utf8(to_hoist_bytes).unwrap();
                    self.hoist_expressions.push(to_add);
                }
            }
            While(data) => {
                // (While 1         <-- x indicates whether has step
                //   (Flag -1)      <-- local (conditional on x)
                //   (ConstInt 1)   <-- local init (conditional on x)
                //   (Not           <-- loop condition
                //      ...
                //   (Seq           <-- local step (conditional on x)
                //      ...
                //   (Seq           <-- loop body
                //      ...
                //

                let has_local_index = match data.x.as_ref().expect("Expected x") {
                    NodeXValue::Normal(x) => *x == 1,
                    NodeXValue::Offset(_) => panic!("Unexpected offset x"),
                };

                let mut local_index_info = None;
                let mut step_info = None;
                let condition;
                let body;

                if has_local_index {
                    expect_count!(data.children, 5);
                    let local = &data.children[0];
                    expect_node!(local, Flag);

                    let local_init = &data.children[1];

                    condition = &data.children[2];
                    step_info = Some(&data.children[3]);
                    body = expect_node!(&data.children[4], Seq);
                    local_index_info = Some((local, local_init));
                } else {
                    expect_count!(data.children, 3);
                    condition = &data.children[0];
                    let step_init = expect_node!(&data.children[1], Seq);

                    if step_init.children.len() > 0 {
                        expect_count!(step_init.children, 1);
                        step_info = Some(&step_init.children[0]);
                    }

                    body = expect_node!(&data.children[2], Seq);
                }

                if let Some((local, local_init)) = local_index_info {
                    write!(writer, "-- While init")?;
                    self.indent_newline(writer, depth)?;
                    self.write_node(writer, local, depth)?;
                    write!(writer, " = ")?;
                    self.write_node(writer, local_init, depth)?;
                    self.indent_newline(writer, depth)?;
                }

                write!(writer, "while ")?;
                self.write_node(writer, condition, depth)?;
                write!(writer, " do")?;

                self.indent_newline(writer, depth + 1)?;
                self.write_children(writer, &body.children, depth + 1, "\n")?;

                if let Some(step) = step_info {
                    self.indent_newline(writer, depth + 1)?;
                    self.write_node(writer, step, depth + 1)?;
                    // self.write_node(writer, local, depth + 1)?;
                    // write!(writer, " = ")?;
                    // self.write_node(writer, local, depth + 1)?;
                    // write!(writer, " + {}", step.value)?;
                }

                self.indent_newline(writer, depth)?;
                write!(writer, "end")?;
                self.indent_newline(writer, depth)?;
            }
            BreakNow(data) => {
                expect_count!(data.children, 0);
                write!(writer, "break")?;
            }

            NodeVariant::ConstStr(data) => {
                write!(
                    writer,
                    "\"{}\"",
                    data.value.0.replace("\\", "\\\\").replace("\"", "\\\"")
                )?;
            }
            NodeVariant::ConstInt(data) => {
                write!(writer, "{}", data.value)?;
            }

            Flag(data) => {
                expect_count!(data.children, 0);
                match data.x.as_ref().expect("Expected x") {
                    NodeXValue::Normal(x) => {
                        write!(writer, "Local_{}", x.abs())?;
                    }
                    NodeXValue::Offset(x) => {
                        write!(writer, "LocalOff_{}", x.abs())?;
                    }
                };
            }
            FlagIndex(data) => {
                // (FlagIndex
                //   (FlagArr <id> <start>)  <-- the flag array
                //   <index>)
                // e.g. GLChangeCount[CurrentGameLevel] where GLChangeCount is flag array 93
                expect_count!(data.children, 2);
                let arr = expect_node!(&data.children[0], FlagArr);
                write!(writer, "Local_{}[", arr.x.abs())?;
                self.write_node(writer, &data.children[1], depth)?;
                write!(writer, "]")?;
            }
            AssignAddSub(data) => {
                expect_count!(data.children, 2);
                let flag = &data.children[0];
                expect_node!(flag, Flag, StrFlag);
                let value = &data.children[1];

                let op = get_addsub_op(data.x.as_ref());
                // match data.x.as_ref().expect("Expected X for addsub") {
                //     NodeXValue::Normal(x) => match x {
                //         0 => '+',
                //         1 => '-',
                //         _ => panic!("Unexpected op"),
                //     },
                //     NodeXValue::Offset(_) => panic!("Unexpected offset"),
                // };

                self.write_node(writer, flag, depth)?;
                write!(writer, " = ")?;
                self.write_node(writer, flag, depth)?;
                write!(writer, " {} (", op)?;
                self.write_node(writer, value, depth)?;
                write!(writer, ")")?;
            }
            PostfixAddSub(data) => {
                expect_count!(data.children, 2);
                let flag = &data.children[0];
                expect_node!(flag, ObjRef, Object, GetObjAttr);
                let value = &data.children[1];

                let member = get_str!(data);
                let op = get_addsub_op(data.x.as_ref());

                self.write_node(writer, flag, depth)?;
                write!(writer, ".{} = ", member)?;
                self.write_node(writer, flag, depth)?;
                write!(writer, ".{} {} (", member, op)?;
                self.write_node(writer, value, depth)?;
                write!(writer, ")")?;
            }

            Mul(data) | Div(data) | Add(data) | Sub(data) | And(data) | Or(data) | Eq(data)
            | Gt(data) | Lt(data) => {
                expect_count!(data.children, 2);

                let op = match &node.variant {
                    Mul(_) => "*",
                    Div(_) => "/",
                    Add(_) => "+",
                    Sub(_) => "-",
                    And(_) => "and",
                    Or(_) => "or",
                    Eq(_) => "==",
                    Gt(_) => ">=",
                    Lt(_) => "<=",
                    _ => panic!("Expected simple math operator: {:?}", node),
                };

                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, " {} ", op)?;
                self.write_node(writer, &data.children[1], depth)?;
            }
            NodeVariant::Not(data) => {
                expect_count!(data.children, 1);
                write!(writer, "not (")?;
                self.write_node(writer, &data.children[0], depth)?;
                write!(writer, ")")?;
            }

            NodeVariant::ListReplace(data) => {
                expect_count!(data.children, 3);
                write!(writer, "ListReplace(")?;
                self.write_children(writer, &data.children, depth, ", ")?;
                write!(writer, ")")?;
            }

            NodeVariant::True(data) => {
                expect_count!(data.children, 0);
                write!(writer, "true")?;
            }
            NodeVariant::False(data) => {
                expect_count!(data.children, 0);
                write!(writer, "false")?;
            }

            Repeat(data) => {
                expect_count!(data.children, 0);
                let (func, args) = self
                    .current_function
                    .as_ref()
                    .expect("No current function for repeat");
                write!(writer, "{}({})", func, args)?;
            }
            GetStringById(data) => {
                expect_count!(data.children, 0);
                let id = match data.x.as_ref().expect("Missing ID") {
                    NodeXValue::Normal(x) => x,
                    NodeXValue::Offset(_) => panic!("Unexpected ID"),
                };
                write!(writer, "{}({})", ModlibFuncs::GetStringById, id)?;
            }
            ClearGlobPhrase(data) => {
                expect_count!(data.children, 0);
                write!(writer, "{}()", ModlibFuncs::ClearGlobPhrase)?;
            }
            Op07(data) => {
                expect_count!(data.children, 0);
                let id = match data.x.as_ref().expect("Missing ID") {
                    NodeXValue::Normal(x) => x,
                    NodeXValue::Offset(_) => panic!("Unexpected ID"),
                };
                write!(writer, "Op07({})", id)?;
            }

            _ => {
                panic!(
                    "Unimplemented {}:\n{:?}",
                    node.variant.get_name(),
                    node.variant
                )
            }
        }
        Ok(())
    }

    fn get_object_index(&mut self, x: Option<&NodeXValue>) -> usize {
        match x.expect("Expected object index") {
            NodeXValue::Normal(v) => *v as usize,
            NodeXValue::Offset(v) => *v as usize,
        }
    }

    fn collect_object_names(&mut self, ctor_children: &Vec<Node>) -> Result<(), Error> {
        for child in ctor_children {
            match &child.variant {
                NewObjAttr(data) => {
                    expect_count!(data.children, 2);

                    let obj_data = expect_node!(&data.children[0], Object);
                    let obj_index = self.get_object_index(obj_data.x.as_ref());
                    let key = get_str!(data).0.clone();

                    if key != "DISPLAYNAME" {
                        continue;
                    }

                    let value = &expect_node!(&data.children[1], ConstStr).value.0;
                    let object_name_map = if self.is_main {
                        &mut self.main_object_names
                    } else {
                        &mut self.module_object_names
                    };
                    object_name_map.insert(obj_index, value.clone());
                }
                _ => continue,
            }
        }
        Ok(())
    }

    fn write_ctor_dtor<T: Write>(&mut self, writer: &mut T, node: &Node) -> Result<(), Error> {
        let ctor = expect_node!(node, CtorPhrase);
        expect_count!(&ctor.children, 1);

        let mut objects: HashMap<usize, HashMap<String, String>> = HashMap::new();

        let seq = expect_node!(&ctor.children[0], Seq);

        self.collect_object_names(&seq.children)?;

        for child in seq.children.iter() {
            match &child.variant {
                Assign(_) => {}
                NewObjAttr(data) | SetObjAttr(data) | NewObjectPhrase(data) => {
                    expect_count!(data.children, 2);

                    let obj_data = expect_node!(&data.children[0], Object);
                    // let obj_index = match obj_data.x.as_ref().expect("Expected object index") {
                    //     NodeXValue::Normal(v) => {
                    //         if !self.is_main {
                    //             panic!("Expected module object index")
                    //         }
                    //         *v as usize
                    //     }
                    //     NodeXValue::Offset(v) => {
                    //         if self.is_main {
                    //             panic!("Expected main object index")
                    //         }
                    //         *v as usize
                    //     }
                    // };
                    let obj_index = self.get_object_index(obj_data.x.as_ref());

                    if !objects.contains_key(&obj_index) {
                        objects.insert(obj_index, HashMap::new());
                    }
                    let obj = objects
                        .get_mut(&obj_index)
                        .expect("Somehow still missing object");

                    let key = get_str!(data).0.clone();
                    if key == "DISPLAYNAME" {
                        continue;
                    }

                    let value_node = &data.children[1];
                    let value = match &value_node.variant {
                        ConstStr(strd) => format!("\"{}\"", strd.value.0),
                        ConstInt(intd) => intd.value.to_string(),
                        ListRef(_) => "{}".to_string(),
                        GetStringById(data) => {
                            let id = match data.x.as_ref().expect("Missing value for GetStringById")
                            {
                                NodeXValue::Normal(x) => x,
                                NodeXValue::Offset(_) => {
                                    panic!("Unexpected offset x for GetStringById")
                                }
                            };
                            format!("{}({})", ModlibFuncs::GetStringById, id)
                        }
                        Object(data) => {
                            let idx = self.get_object_index(data.x.as_ref());
                            if self.is_main {
                                self.main_object_names
                                    .get(&idx)
                                    .expect("Missing object name")
                                    .clone()
                            } else {
                                self.module_object_names
                                    .get(&idx)
                                    .expect("Missing object name")
                                    .clone()
                            }
                        }
                        UserFunc(_) => {
                            let mut writer = BufWriter::new(Vec::new());
                            self.write_node(&mut writer, value_node, 1)?;
                            let buf = writer.into_inner().expect("Failed to get inner buf");
                            String::from_utf8(buf).expect("msg")
                        }
                        Seq(_) => {
                            let mut str_writer = BufWriter::new(Vec::new());
                            self.write_node(&mut str_writer, value_node, 1)?;

                            let buf = str_writer.into_inner().expect("Failed to get inner buf");
                            String::from_utf8(buf).expect("msg")
                        }
                        Phrase(_) => {
                            let phrase_index = match data.x.as_ref().expect("Expected phrase index")
                            {
                                NodeXValue::Normal(v) => {
                                    if !self.is_main {
                                        panic!("Expected main phrase index");
                                    }
                                    *v as usize
                                }
                                NodeXValue::Offset(v) => {
                                    if self.is_main {
                                        panic!("Expected module phrase index");
                                    }
                                    *v as usize
                                }
                            };

                            let (object_name_map, phrase_map, phrase_offsets) = if self.is_main {
                                (
                                    &self.main_object_names,
                                    &mut self.main_renamed_phrases_by_address,
                                    &self.main_phrases,
                                )
                            } else {
                                (
                                    &self.module_object_names,
                                    &mut self.module_renamed_phrases_by_address,
                                    &self.module_phrases,
                                )
                            };
                            let phrase_off = phrase_offsets[phrase_index];
                            if let Some(object_name) = object_name_map.get(&obj_index) {
                                phrase_map
                                    .insert(phrase_off as u64, format!("{}:{}", object_name, key));
                            } else {
                                phrase_map.insert(phrase_off as u64, key.clone());
                            }

                            // println!("Phrase_{} renamed {}", phrase_off, key);
                            // I think Lua class methods don't have to be wired like this?
                            continue;
                        }
                        OnTimerEvent(_) => {
                            println!("Skipping timer obj attr");
                            "\"timer_skipped_TODO\"".to_string()
                        }
                        _ => panic!(
                            "Unexpected NewObjAttr variant for key {}: {}\n{:?}",
                            key,
                            data.children[1].variant.get_name(),
                            value_node
                        ),
                    };

                    // println!("{}.{} = {}", obj_index, key, value);
                    obj.insert(key, value);
                }
                Inherit(_) => {
                    println!("TODO: skipping inherit in ctor");
                }
                Seq(_) => {
                    println!("TODO: skipping seq in ctor");
                }
                OnTimerEvent(_) => {
                    println!("TODO: skipping timer in ctor");
                }
                Call(_) => {
                    println!("TODO: skipping call in ctor");
                }
                ListItem(_) => {
                    println!("TODO: skipping listitem in ctor");
                }
                DeleteAttr(_) => {
                    println!("TODO: skipping deleteattr in ctor");
                }
                UserFunc(_) => {
                    println!("TODO: skipping userfunc in ctor");
                }
                Event(_) => {
                    println!("TODO: skipping event in ctor");
                }
                If(_) => {
                    println!("TODO: skipping if in ctor");
                }
                EventDelete(_) => {
                    println!("TODO: skipping EventDelete in ctor");
                }
                // NewObjectPhrase(data) => {
                //     expect_count!(data.children, 2);
                //     expect_node!(&data.children[0], Object);
                //     expect_node!(&data.children[1], Phrase);

                //     let index = match data.x.as_ref().unwrap() {
                //         NodeXValue::Normal(_) => panic!("Unexpected"),
                //         NodeXValue::Offset(v) => *v as usize,
                //     };
                //     let phrase_off = self.phrases[index];

                //     // self.write_node(writer, &data.children[0], depth)?;
                //     // write!(writer, ".{} = Phrase_{}", get_str!(data), phrase_off)?;
                // }
                _ => panic!(
                    "Unexpected node in ctor: {}\n{:?}",
                    child.variant.get_name(),
                    child.variant
                ),
            }
        }

        self.write_object_classes(writer, &objects)?;

        Ok(())
    }

    fn write_object_classes<T: Write>(
        &mut self,
        writer: &mut T,
        objects: &HashMap<usize, HashMap<String, String>>,
    ) -> Result<(), Error> {
        for (obj_index, map) in objects.iter() {
            let obj_names = if self.is_main {
                &self.main_object_names
            } else {
                &self.module_object_names
            };
            let obj_name = obj_names
                .get(&obj_index)
                .map_or(format!("Object[{}]", obj_index), |n| n.clone());
            // .expect(format!("Expected object {} to have DISPLAYNAME", obj_index).as_str());

            write!(writer, "{} = {{\n", obj_name)?;
            for (i, (k, v)) in map.iter().enumerate() {
                write!(writer, "  {} = {}", k, v)?;
                if i + 1 != map.len() {
                    write!(writer, ",")?;
                }
                write!(writer, "\n")?;
            }
            write!(writer, "}}\n")?;
        }

        Ok(())
    }
}

// CtorPhrase(data) | DtorPhrase(data) => {
//     expect_count!(data.children, 1);
//     let body = expect_node!(data.children[0], Seq);
//
//     self.in_phrase = true;
//
//     let name = match &node.variant {
//         CtorPhrase(_) => format!("Ctor_{}", node.opcode.pos),
//         DtorPhrase(_) => format!("Dtor_{}", node.opcode.pos),
//         _ => panic!("Unexpected"),
//     };
//     self.write_function(&name, &body.children)?;
//
//     self.in_phrase = false;
// }
