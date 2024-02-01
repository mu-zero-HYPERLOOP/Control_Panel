use can_config_rs::config::{MessageId, MessageRef, SignalType, Type};
use serde::{ser::SerializeMap, Serialize};

use super::can_adapter::timestamped::Timestamped;
use bitvec::{
    prelude::{BitOrder, Lsb0},
    store::BitStore,
    vec::BitVec,
};

#[derive(Clone)]
pub struct Frame {
    config: MessageRef,
    data: u64,
    value: FrameValue,
}

pub type TFrame = Timestamped<Frame>;

#[derive(Debug, Clone)]
pub enum Value {
    UnsignedValue(u64),
    SignedValue(i64),
    RealValue(f64),
    StructValue(Vec<Attribute>),
    EnumValue(String),
}

impl Value {
    /// if the value is of type StructValue then the attribute value of the
    pub fn attribute(&self, name: &str) -> Option<&Value> {
        match &self {
            Value::StructValue(attribs) => attribs
                .iter()
                .find(|attrib| attrib.name == name)
                .map(|attrib| &attrib.value),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct FrameValue {
    attributes: Vec<Attribute>,
}

impl FrameValue {
    pub fn new(attributes : Vec<Attribute>) -> Self{
        Self {
            attributes
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Attribute {
    name: String,
    value: Value,
}

impl Attribute {
    pub fn new(name: &str, value: Value) -> Self {
        Self {
            name: name.to_owned(),
            value,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn value(&self) -> &Value {
        &self.value
    }
}

impl Frame {
    pub fn new(config: MessageRef, data: u64, value: FrameValue) -> Self {
        Self {
            config,
            data,
            value,
        }
    }

    pub fn id(&self) -> &MessageId {
        self.config.id()
    }
    pub fn ide(&self) -> bool {
        self.config.id().ide()
    }
    pub fn data(&self) -> u64 {
        self.data
    }
    pub fn rtr(&self) -> bool {
        // TODO am i even required still
        return false;
    }
    pub fn description(&self) -> Option<&str> {
        self.config.description()
    }
    pub fn name(&self) -> &str {
        self.config.name()
    }
    pub fn dlc(&self) -> u8 {
        self.config.dlc()
    }
    pub fn attribute(&self, name: &str) -> Option<&Value> {
        self.value
            .attributes
            .iter()
            .find(|attrib| attrib.name == name)
            .map(|attrib| &attrib.value)
    }
    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.value.attributes
    }
}

impl Value {
    pub fn get_as_bin<S>(&self, ty: &Type) -> (Vec<S>, u8)
    where
        S: BitStore,
    {
        let mut bit_vec: BitVec<S, Lsb0> = BitVec::new();

        fn continue_get_as_bin<S, O>(value: &Value, ty: &Type, bit_vec: &mut BitVec<S, O>)
        where
            S: BitStore,
            O: BitOrder,
        {
            match (value, ty) {
                (Value::UnsignedValue(val), Type::Primitive(SignalType::UnsignedInt { size })) => {
                    for i in 0..*size {
                        let bit_int = (*val >> i) & 0x1;
                        bit_vec.push(if bit_int == 0 { false } else { true });
                    }
                }
                (Value::SignedValue(val), Type::Primitive(SignalType::SignedInt { size })) => {
                    for i in 0..*size {
                        let bit_int = (*val >> i) & 0x1;
                        bit_vec.push(if bit_int == 0 { false } else { true });
                    }
                }
                (
                    Value::RealValue(val),
                    Type::Primitive(SignalType::Decimal {
                        size,
                        offset,
                        scale,
                    }),
                ) => {
                    let base_float = (val - offset) / scale;
                    let base_bits = base_float.round() as i64;
                    // just in case floating point errors fuck us.
                    // not sure if actually needed.
                    let base_bits: u64 = if base_bits < 0 {
                        0
                    } else if (base_bits >> size) > 0 {
                        0xffff_ffff_ffff_ffff
                    } else {
                        base_bits as u64
                    };
                    for i in 0..*size {
                        let bit_int = (base_bits >> i) & 0x1;
                        bit_vec.push(if bit_int == 0 { false } else { true });
                    }
                }
                (
                    Value::StructValue(value_attributes),
                    Type::Struct {
                        name: _,
                        description: _,
                        attribs: struct_attributes,
                        visibility: _,
                    },
                ) => {
                    for (attrib, (_, attrib_ty)) in value_attributes.iter().zip(struct_attributes) {
                        continue_get_as_bin(attrib.value(), attrib_ty, bit_vec);
                    }
                    // for (nested_val, nested_type) in val
                    //     .attributes()
                    //     .iter()
                    //     .zip(attribs.iter().map(|attr| attr.1.clone()))
                    // {
                    //     continue_get_as_bin(nested_val.value(), nested_type.as_ref(), bit_vec)
                    // }
                }
                (
                    Value::EnumValue(variant_name),
                    Type::Enum {
                        name: _,
                        description: _,
                        size,
                        entries,
                        visibility: _,
                    },
                ) => {
                    if let Some(enum_val) = entries.iter().find(|(name, _)| name == variant_name) {
                        let bit_val = enum_val.1;
                        for i in 0..*size {
                            let bit_int = (bit_val >> i) & 0x1;
                            bit_vec.push(if bit_int == 0 { false } else { true });
                        }
                    } else {
                        panic!("enum variant name not known!");
                    };
                }
                _ => panic!("TypeValue and config::Type did not match!"),
            };
        }

        continue_get_as_bin(self, ty, &mut bit_vec);
        let num_bytes = (bit_vec.len() + 7) / 8;
        let last_fill: u8 = (num_bytes % std::mem::size_of::<S>()) as u8;

        bit_vec.set_uninitialized(false);
        let vec_t = bit_vec.into_vec();
        return (vec_t, last_fill);
    }
}

impl Serialize for Frame {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("id", &self.id().as_u32())?;
        map.serialize_entry("ide", &self.ide())?;
        map.serialize_entry("rtr", &self.rtr())?;
        map.serialize_entry("dlc", &self.dlc())?;
        map.serialize_entry("name", self.name())?;
        match self.description() {
            Some(desc) => map.serialize_entry("description", desc)?,
            None => (),
        };
        map.serialize_entry("attributes", self.attributes());
        map.serialize_entry("data", &self.data());
        map.end()
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match &self {
            Value::UnsignedValue(value) => serializer.serialize_u64(*value),
            Value::SignedValue(value) => serializer.serialize_i64(*value),
            Value::RealValue(value) => serializer.serialize_f64(*value),
            Value::StructValue(attributes) => {
                let mut map = serializer.serialize_map(Some(attributes.len()))?;
                for attrib in attributes {
                    map.serialize_entry(attrib.name(), attrib.value())?;
                }
                map.end()
            }
            Value::EnumValue(value) => serializer.serialize_str(value),
        }
    }
}
