// This file was autogenerated from Opc.Ua.NodeSet2.Part14.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::{convert::TryFrom, str::FromStr};

#[allow(unused_imports)]
use crate::types::{service_types::Argument, *};

#[allow(unused_imports)]
use crate::address_space::{types::*, EventNotifier};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_object_1(address_space);
    add_object_2(address_space);
    add_datatype_3(address_space);
}

fn add_object_1(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 14801);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 14532),
                &ReferenceTypeId::HasEncoding,
                ReferenceDirection::Inverse,
            ),
            (
                &NodeId::new(0, 14826),
                &ReferenceTypeId::HasDescription,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 76),
                &ReferenceTypeId::HasTypeDefinition,
                ReferenceDirection::Forward,
            ),
        ]),
    );
}

fn add_object_2(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 14845);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    let _ = address_space.insert(
        node,
        Some(&[
            (
                &NodeId::new(0, 14532),
                &ReferenceTypeId::HasEncoding,
                ReferenceDirection::Inverse,
            ),
            (
                &NodeId::new(0, 14870),
                &ReferenceTypeId::HasDescription,
                ReferenceDirection::Forward,
            ),
            (
                &NodeId::new(0, 76),
                &ReferenceTypeId::HasTypeDefinition,
                ReferenceDirection::Forward,
            ),
        ]),
    );
}

fn add_datatype_3(address_space: &mut AddressSpace) {
    // DataType
    let name = "EnumField";
    let node_id = NodeId::new(0, 14532);
    let node = DataType::new(&node_id, name, name, false);
    let _ = address_space.insert(
        node,
        Some(&[(
            &NodeId::new(0, 7594),
            &ReferenceTypeId::HasSubtype,
            ReferenceDirection::Inverse,
        )]),
    );
}
