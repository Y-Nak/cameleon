#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::missing_errors_doc
)]

pub mod elem_type;
pub mod formula;
pub mod parser;
pub mod store;

pub use boolean::BooleanNode;
pub use category::CategoryNode;
pub use command::CommandNode;
pub use converter::ConverterNode;
pub use enumeration::{EnumEntryNode, EnumerationNode};
pub use float::FloatNode;
pub use float_reg::FloatRegNode;
pub use int_converter::IntConverterNode;
pub use int_reg::IntRegNode;
pub use int_swiss_knife::IntSwissKnifeNode;
pub use integer::IntegerNode;
pub use masked_int_reg::MaskedIntRegNode;
pub use node::Node;
pub use node_base::NodeBase;
pub use port::PortNode;
pub use register::RegisterNode;
pub use register_base::RegisterBase;
pub use register_description::RegisterDescription;
pub use string::StringNode;
pub use string_reg::StringRegNode;
pub use swiss_knife::SwissKnifeNode;

mod boolean;
mod category;
mod command;
mod converter;
mod enumeration;
mod float;
mod float_reg;
mod int_converter;
mod int_reg;
mod int_swiss_knife;
mod integer;
mod masked_int_reg;
mod node;
mod node_base;
mod port;
mod register;
mod register_base;
mod register_description;
mod string;
mod string_reg;
mod swiss_knife;

pub trait Device {
    type Error: std::error::Error;

    fn read_mem(&mut self, address: u64, buf: &mut [u8]) -> Result<(), Self::Error>;

    fn write_mem(&mut self, address: u64, data: &[u8]) -> Result<(), Self::Error>;
}
