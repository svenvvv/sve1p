use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{Error, Read, Seek, Write};
use std::rc::Rc;

use binrw::{BinRead, BinResult, BinWrite, Endian, PosValue};

use crate::utils::e1pstring::E1pString;

#[macro_export]
macro_rules! define_nodes {
    (
        $( $variant:ident ( $data_type:ty ) = $opcode:expr ),* $(,)?
    ) => {
        #[derive(BinRead, BinWrite, Debug)]
        #[bw(import(ctx: WriterContext))]
        pub enum NodeVariant {
            $(
                #[brw(magic = $opcode)]
                $variant(
                    #[bw(args(ctx.clone()))]
                    $data_type
                ),
            )*
        }

        impl NodeVariant {
            pub fn get_name(&self) -> &str {
                match self {
                    $(
                        NodeVariant::$variant(_) => stringify!($variant),
                    )*
                }
            }
        }

        impl NodeVariant {
            pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
                match self {
                    $(
                        NodeVariant::$variant(data) => data.visit(visitor),
                    )*
                }
            }
        }

    };
}

define_nodes! {
    Exit(AmalgamationData)              = 0x00u16,
    Block(AmalgamationData)             = 0x01u16,

    // E5
    Op02(AmalgamationData)              = 0x02u16,

    Flag(AmalgamationData)              = 0x03u16,
    FlagArr(UnknownData)                = 0x04u16,
    StrFlag(AmalgamationData)           = 0x05u16,
    Op06(AmalgamationData)              = 0x06u16,
    Op07(AmalgamationData)              = 0x07u16,
    Checklist(AmalgamationData)         = 0x08u16,
    Phrase(AmalgamationData)            = 0x09u16,
    CtorPhrase(AmalgamationData)        = 0x0au16,
    DtorPhrase(AmalgamationData)        = 0x0bu16,
    If(AmalgamationData)                = 0x0cu16,
    Switch(AmalgamationData)            = 0x0du16,
    Case(AmalgamationData)              = 0x0eu16,
    Return(AmalgamationData)            = 0x0fu16,
    RetVal(AmalgamationData)            = 0x10u16,
    Repeat(AmalgamationData)            = 0x11u16,

    // 7.62 Reload
    Op12(AmalgamationData)              = 0x12u16,

    // E5
    Op14(AmalgamationData)              = 0x14u16,

    Print(AmalgamationData)             = 0x15u16,
    SpeakPhrase(AmalgamationData)       = 0x16u16,
    SetUserPhrase(AmalgamationData)     = 0x17u16,
    AskChoice(AmalgamationData)         = 0x18u16,
    Solve(AmalgamationData)             = 0x19u16,
    Assign(AmalgamationData)            = 0x1au16,
    Call(AmalgamationData)              = 0x1bu16,
    Wait(AmalgamationData)              = 0x1cu16,

    ConstInt(InlineIntData)             = 0x1eu16,
    Mul(AmalgamationData)               = 0x1fu16,
    Div(AmalgamationData)               = 0x20u16,
    Add(AmalgamationData)               = 0x21u16,
    Sub(AmalgamationData)               = 0x22u16,
    ConstStr(InlineStringData)          = 0x23u16,
    Concat(AmalgamationData)            = 0x24u16,
    False(AmalgamationData)             = 0x25u16,
    True(AmalgamationData)              = 0x26u16,
    And(AmalgamationData)               = 0x27u16,
    Or(AmalgamationData)                = 0x28u16,
    Not(AmalgamationData)               = 0x29u16,
    Eq(AmalgamationData)                = 0x2au16,
    Gt(AmalgamationData)                = 0x2bu16,
    Lt(AmalgamationData)                = 0x2cu16,
    In(AmalgamationData)                = 0x2du16,
    Range(AmalgamationData)             = 0x2eu16,
    Seq(AmalgamationData)               = 0x2fu16,
    FlagIndex(AmalgamationData)         = 0x30u16,
    UserFunc(AmalgamationData)          = 0x31u16,
    While(AmalgamationData)             = 0x33u16,
    Op34(UnknownData)                   = 0x34u16,
    Op35(UnknownData)                   = 0x35u16,
    Break(AmalgamationData)             = 0x36u16,
    End(AmalgamationData)               = 0x37u16,

    // E5
    Op3a(UnknownData)                   = 0x3au16,

    ClearGlobChoice(AmalgamationData)   = 0x3bu16,
    ClearGlobPhrase(AmalgamationData)   = 0x3cu16,
    Object(AmalgamationData)            = 0x3du16,
    NewObjAttr(AmalgamationData)        = 0x3eu16,
    SetObjAttr(AmalgamationData)        = 0x3fu16,

    GetObjAttr(AmalgamationData)        = 0x40u16,
    HasAttr(AmalgamationData)           = 0x41u16,
    DeleteAttr(AmalgamationData)        = 0x42u16,

    Event(AmalgamationData)             = 0x46u16,
    ThrowEvent(AmalgamationData)        = 0x47u16,
    OnTimerEvent(AmalgamationData)      = 0x48u16,

    List(AmalgamationData)              = 0x4au16,
    ListReplace(AmalgamationData)       = 0x4bu16,

    ListClearHash(AmalgamationData)     = 0x4du16,
    ListAppend(AmalgamationData)        = 0x4eu16,
    ListItem(AmalgamationData)          = 0x4fu16,
    ListEraseItem(AmalgamationData)     = 0x50u16,
    ListRef(AmalgamationData)           = 0x51u16,
    ListIsType(AmalgamationData)        = 0x53u16,
    ListGet(AmalgamationData)           = 0x54u16,
    ListFind(AmalgamationData)          = 0x55u16,
    ListGetNext(AmalgamationData)       = 0x56u16,
    ListGetHash(AmalgamationData)       = 0x57u16,
    NewObjectPhrase(AmalgamationData)   = 0x58u16,
    ObjectDelete(AmalgamationData)      = 0x59u16,
    PushMember(AmalgamationData)        = 0x5au16,
    ListCount(AmalgamationData)         = 0x5fu16,
    ListClear(AmalgamationData)         = 0x60u16,

    // E5
    Op61(AmalgamationData)              = 0x61u16,
    Op62(AmalgamationData)              = 0x62u16,
    Op63(AmalgamationData)              = 0x63u16,
    Op64(AmalgamationData)              = 0x64u16,

    ObjRef(AmalgamationData)            = 0x65u16,
    CopyObjectMethod(AmalgamationData)  = 0x66u16,
    PushObjectRef(AmalgamationData)     = 0x67u16,
    PopObjectRef(AmalgamationData)      = 0x68u16,
    EventDelete(AmalgamationData)       = 0x69u16,
    GetStringById(AmalgamationData)     = 0x6au16,
    Inherit(AmalgamationData)           = 0x6bu16,
    BreakNow(AmalgamationData)          = 0x6cu16,

    OnEvent(AmalgamationData)           = 0x6eu16,
    AssignAddSub(AmalgamationData)      = 0x6fu16,
    PostfixAddSub(AmalgamationData)     = 0x70u16,

    HasMethod(AmalgamationData)         = 0x72u16,

    // E5
    Op7f(AmalgamationData)              = 0x7fu16,
}

#[derive(Clone)]
pub struct WriterContext {
    /// Addresses to track relocations of
    pub track_relocation: Rc<HashSet<u32>>,
    /// Key old address (from track_relocations), value new address
    pub relocatations: Rc<RefCell<HashMap<u32, u32>>>,
}

impl WriterContext {
    pub fn new(addresses: HashSet<u32>) -> WriterContext {
        WriterContext {
            track_relocation: Rc::new(addresses),
            relocatations: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn empty() -> WriterContext {
        WriterContext {
            track_relocation: Rc::new(HashSet::new()),
            relocatations: Rc::new(RefCell::new(HashMap::new())),
        }
    }
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct Node {
    /// This field is skipped and only used to track the original address for relocations
    #[br(restore_position)]
    #[bw(args(ctx.clone()), write_with = track_relocations)]
    pub opcode: PosValue<u16>,
    #[bw(args(ctx.clone()))]
    pub variant: NodeVariant,
}

fn track_relocations<W: Write + Seek>(
    field: &PosValue<u16>,
    writer: &mut W,
    _endian: Endian,
    args: (WriterContext,),
) -> BinResult<()> {
    let ctx = args.0;
    let old_off: u32 = field.pos.try_into().expect("Failed to downcast offset");
    let new_off: u32 = writer
        .stream_position()?
        .try_into()
        .expect("Failed to downcast offset");

    if ctx.track_relocation.contains(&old_off) {
        let mut rel = ctx.relocatations.borrow_mut();
        rel.insert(old_off, new_off);
    }

    Ok(())
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct InlineStringData {
    pub value: E1pString<u16, true, true>,
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct InlineIntData {
    pub value: i32,
}

#[derive(Debug)]
pub enum NodeXValue {
    Normal(i16),
    Offset(i16),
}

impl fmt::Display for NodeXValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            NodeXValue::Normal(v) => v,
            NodeXValue::Offset(v) => v,
        };
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub struct AmalgamationData {
    pub x: Option<NodeXValue>,
    pub string: Option<E1pString<u16>>,
    pub children: Vec<Node>,
    // TODO: this doesn't match for the root children,
    // Either e1pc writes junk or we're parsing nodes wrong somewhere...
    #[allow(dead_code)]
    original_children_count: usize,
}

#[derive(BinRead, BinWrite, Debug)]
#[bw(import(ctx: WriterContext))]
pub struct UnknownData {
    // If y is 0xffff then: x += module_modifier_index
    pub x: i16,
    pub y: u16,
}

impl BinRead for AmalgamationData {
    type Args<'a> = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        endian: Endian,
        (): Self::Args<'_>,
    ) -> BinResult<Self> {
        let mut x = None;
        let mut string = None;

        // let offset = reader.stream_position()?;
        let mut tag = u16::read_options(reader, endian, ())?;
        if tag == 0xffff {
            let raw_x = i16::read_options(reader, endian, ())?;

            tag = u16::read_options(reader, endian, ())?;
            if tag == 0xffff {
                // I think this tag is to indicate some x-value transformation
                x = Some(NodeXValue::Offset(raw_x));
                tag = u16::read_options(reader, endian, ())?;
            } else {
                x = Some(NodeXValue::Normal(raw_x));
            }
        }

        if tag == 0xfffe {
            string = Some(E1pString::read_options(reader, endian, ())?);
            tag = u16::read_options(reader, endian, ())?;
        }

        let children_count = tag.into();
        let mut children = Vec::with_capacity(children_count);
        for _ in 0..tag {
            match Node::read_options(reader, endian, ()) {
                Ok(node) => {
                    children.push(node);
                }
                Err(e) => {
                    // Weird fucking format... No idea why they even write child count if it's not always valid.
                    if e.is_eof() {
                        // println!(
                        //     "Got EOF before finishing all {} children: {}",
                        //     children_count, offset
                        // );
                        break;
                    }
                    return Err(e);
                }
            };
        }

        Ok(AmalgamationData {
            x: x,
            string: string,
            children: children,
            original_children_count: children_count,
        })
    }
}

impl BinWrite for AmalgamationData {
    type Args<'a> = (WriterContext,);

    fn write_options<W: std::io::Write + Seek>(
        &self,
        writer: &mut W,
        endian: Endian,
        args: Self::Args<'_>,
    ) -> BinResult<()> {
        if self.x.is_some() {
            0xffff_u16.write_options(writer, endian, ())?;

            match self.x.as_ref().unwrap() {
                NodeXValue::Normal(x) => {
                    x.write_options(writer, endian, ())?;
                }
                NodeXValue::Offset(x) => {
                    x.write_options(writer, endian, ())?;
                    0xffff_u16.write_options(writer, endian, ())?;
                }
            }
        }

        if self.string.is_some() {
            0xfffe_u16.write_options(writer, endian, ())?;
            self.string.write_options(writer, endian, ())?;
        }

        let file_children_count: u16 = self
            .original_children_count
            .try_into()
            .expect("Failed to fit children count into u16");
        let children_count: u16 = self
            .children
            .len()
            .try_into()
            .expect("Failed to fit children count into u16");
        // If we have original children count then use that so I can use checksums to check for correct round-trip
        if file_children_count > children_count {
            file_children_count.write_options(writer, endian, ())?;
        } else {
            children_count.write_options(writer, endian, ())?;
        }
        for c in self.children.iter() {
            c.write_options(writer, endian, args.clone())?;
        }

        Ok(())
    }
}

pub trait NodeVisitor {
    fn visit_node(&mut self, _node: &Node) -> Result<(), Error> {
        Ok(())
    }

    fn visit_amalgamation(&mut self, data: &mut AmalgamationData) -> Result<(), Error>
    where
        Self: Sized,
    {
        for c in data.children.iter_mut() {
            c.visit(self)?;
        }
        Ok(())
    }

    fn visit_inline_int(&mut self, _data: &mut InlineIntData) -> Result<(), Error> {
        Ok(())
    }

    fn visit_inline_string(&mut self, _data: &mut InlineStringData) -> Result<(), Error> {
        Ok(())
    }

    fn visit_unknown(&mut self, _data: &mut UnknownData) -> Result<(), Error> {
        Ok(())
    }
}

impl Node {
    pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_node(&self)?;
        self.variant.visit(visitor)
    }
}

impl AmalgamationData {
    pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_amalgamation(self)
    }
}

impl InlineIntData {
    pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_inline_int(self)
    }
}

impl InlineStringData {
    pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_inline_string(self)
    }
}

impl UnknownData {
    pub fn visit<V: NodeVisitor>(&mut self, visitor: &mut V) -> Result<(), Error> {
        visitor.visit_unknown(self)
    }
}
