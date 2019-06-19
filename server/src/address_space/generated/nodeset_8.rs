// This file was autogenerated from Opc.Ua.NodeSet2.Part8.xml by tools/schema/gen_address_space.js
// DO NOT EDIT THIS FILE

#[allow(unused_imports)]
use std::str::FromStr;

#[allow(unused_imports)]
use opcua_types::{
    node_id::NodeId,
    data_value::DataValue,
    variant::Variant, 
    extension_object::ExtensionObject, 
    string::UAString,
    basic_types::LocalizedText,
    service_types::{
        Argument
    },
    node_ids::*
};
#[allow(unused_imports)]
use crate::address_space::{EventNotifier, types::*};

#[allow(unused_variables)]
pub fn populate_address_space(address_space: &mut AddressSpace) {
    add_object_1(address_space);
    add_object_2(address_space);
    add_object_3(address_space);
    add_object_4(address_space);
    add_object_5(address_space);
    add_object_6(address_space);
    add_object_7(address_space);
    add_object_8(address_space);
    add_object_9(address_space);
    add_object_10(address_space);
    add_object_11(address_space);
    add_object_12(address_space);
    add_datatype_13(address_space);
    add_datatype_14(address_space);
    add_datatype_15(address_space);
    add_datatype_16(address_space);
    add_datatype_17(address_space);
    add_datatype_18(address_space);
    add_datatype_19(address_space);
    add_variable_20(address_space);
    add_variable_21(address_space);
    add_variable_22(address_space);
    add_variable_23(address_space);
    add_variable_24(address_space);
    add_variable_25(address_space);
    add_variable_26(address_space);
    add_variable_27(address_space);
    add_variable_28(address_space);
    add_variable_29(address_space);
    add_variable_30(address_space);
    add_variable_31(address_space);
    add_variable_32(address_space);
    add_variable_33(address_space);
    add_variable_34(address_space);
    add_variable_35(address_space);
    add_variable_36(address_space);
    add_variable_37(address_space);
    add_variable_38(address_space);
    add_variable_39(address_space);
    add_variable_40(address_space);
    add_variable_41(address_space);
    add_variable_42(address_space);
    add_variable_43(address_space);
    add_variabletype_44(address_space);
    add_variabletype_45(address_space);
    add_variabletype_46(address_space);
    add_variabletype_47(address_space);
    add_variabletype_48(address_space);
    add_variabletype_49(address_space);
    add_variabletype_50(address_space);
    add_variabletype_51(address_space);
    add_variabletype_52(address_space);
    add_variabletype_53(address_space);
    add_variabletype_54(address_space);
    add_variabletype_55(address_space);
}

fn add_object_1(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 885);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 884), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8873), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_2(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 888);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 887), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8876), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_3(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 12173);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12171), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12175), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_4(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 12174);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12172), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12178), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_5(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 12081);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12079), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12083), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_6(address_space: &mut AddressSpace) {
    // Object
    let name = "Default XML";
    let node_id = NodeId::new(0, 12082);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12080), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12086), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_7(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 886);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 884), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8238), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_8(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 889);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 887), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 8241), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_9(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 12181);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12171), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12183), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_10(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 12182);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12172), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12186), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_11(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 12089);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12079), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12091), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_object_12(address_space: &mut AddressSpace) {
    // Object
    let name = "Default Binary";
    let node_id = NodeId::new(0, 12090);
    let node = Object::new(&node_id, name, name, EventNotifier::empty());
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12080), ReferenceTypeId::HasEncoding, ReferenceDirection::Inverse),
        (&NodeId::new(0, 12094), ReferenceTypeId::HasDescription, ReferenceDirection::Forward),
        (&NodeId::new(0, 76), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
    ]));
}

fn add_datatype_13(address_space: &mut AddressSpace) {
    // DataType
    let name = "Range";
    let node_id = NodeId::new(0, 884);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_14(address_space: &mut AddressSpace) {
    // DataType
    let name = "EUInformation";
    let node_id = NodeId::new(0, 887);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_15(address_space: &mut AddressSpace) {
    // DataType
    let name = "AxisScaleEnumeration";
    let node_id = NodeId::new(0, 12077);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12078), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 29), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_16(address_space: &mut AddressSpace) {
    // DataType
    let name = "ComplexNumberType";
    let node_id = NodeId::new(0, 12171);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_17(address_space: &mut AddressSpace) {
    // DataType
    let name = "DoubleComplexNumberType";
    let node_id = NodeId::new(0, 12172);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_18(address_space: &mut AddressSpace) {
    // DataType
    let name = "AxisInformation";
    let node_id = NodeId::new(0, 12079);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_datatype_19(address_space: &mut AddressSpace) {
    // DataType
    let name = "XVType";
    let node_id = NodeId::new(0, 12080);
    let node = DataType::new(&node_id, name, name, false);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 22), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_20(address_space: &mut AddressSpace) {
    // Variable
    let name = "Definition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2366);
    let mut node = Variable::new_data_value(&node_id, name, name, DataTypeId::String, value);
    node.set_description(LocalizedText::from("A vendor-specific, human readable string that specifies how the value of this DataItem is calculated."));
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_21(address_space: &mut AddressSpace) {
    // Variable
    let name = "ValuePrecision";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2367);
    let mut node = Variable::new_data_value(&node_id, name, name, DataTypeId::Double, value);
    node.set_description(LocalizedText::from("The maximum precision that the server can maintain for the item based on restrictions in the target environment."));
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_22(address_space: &mut AddressSpace) {
    // Variable
    let name = "InstrumentRange";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2370);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(884u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_23(address_space: &mut AddressSpace) {
    // Variable
    let name = "EURange";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2369);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(884u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_24(address_space: &mut AddressSpace) {
    // Variable
    let name = "EngineeringUnits";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2371);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(887u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2368), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2368), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_25(address_space: &mut AddressSpace) {
    // Variable
    let name = "FalseState";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2374);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2373), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2373), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_26(address_space: &mut AddressSpace) {
    // Variable
    let name = "TrueState";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2375);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2373), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2373), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_27(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 2377);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2376), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 2376), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_28(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumValues";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11241);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(7594u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11238), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11238), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_29(address_space: &mut AddressSpace) {
    // Variable
    let name = "ValueAsText";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 11461);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11238), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 11238), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_30(address_space: &mut AddressSpace) {
    // Variable
    let name = "InstrumentRange";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12024);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(884u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 80), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_31(address_space: &mut AddressSpace) {
    // Variable
    let name = "EURange";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12025);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(884u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_32(address_space: &mut AddressSpace) {
    // Variable
    let name = "EngineeringUnits";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12026);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(887u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_33(address_space: &mut AddressSpace) {
    // Variable
    let name = "Title";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12027);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_34(address_space: &mut AddressSpace) {
    // Variable
    let name = "AxisScaleType";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12028);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12077u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12021), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_35(address_space: &mut AddressSpace) {
    // Variable
    let name = "XAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12037);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12029), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12029), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_36(address_space: &mut AddressSpace) {
    // Variable
    let name = "XAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12046);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12038), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12038), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_37(address_space: &mut AddressSpace) {
    // Variable
    let name = "XAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12055);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12047), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12047), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_38(address_space: &mut AddressSpace) {
    // Variable
    let name = "YAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12056);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12047), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12047), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_39(address_space: &mut AddressSpace) {
    // Variable
    let name = "XAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12065);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_40(address_space: &mut AddressSpace) {
    // Variable
    let name = "YAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12066);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_41(address_space: &mut AddressSpace) {
    // Variable
    let name = "ZAxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12067);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12057), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12057), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_42(address_space: &mut AddressSpace) {
    // Variable
    let name = "AxisDefinition";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12076);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::from_u32(12079u32).unwrap(), value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12068), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12068), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variable_43(address_space: &mut AddressSpace) {
    // Variable
    let name = "EnumStrings";
    let value = Variant::Empty;
    let node_id = NodeId::new(0, 12078);
    let node = Variable::new_data_value(&node_id, name, name, DataTypeId::LocalizedText, value);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12077), ReferenceTypeId::Organizes, ReferenceDirection::Inverse),
        (&NodeId::new(0, 68), ReferenceTypeId::HasTypeDefinition, ReferenceDirection::Forward),
        (&NodeId::new(0, 78), ReferenceTypeId::HasModellingRule, ReferenceDirection::Forward),
        (&NodeId::new(0, 12077), ReferenceTypeId::HasProperty, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_44(address_space: &mut AddressSpace) {
    // VariableType
    let name = "DataItemType";
    let node_id = NodeId::new(0, 2365);
    let mut node = VariableType::new(&node_id, name, name, NodeId::null(), false, -2);
    node.set_description(LocalizedText::from("A variable that contains live automation data."));
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2366), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2367), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 63), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_45(address_space: &mut AddressSpace) {
    // VariableType
    let name = "AnalogItemType";
    let node_id = NodeId::new(0, 2368);
    let node = VariableType::new(&node_id, name, name, NodeId::new(0, 26), false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2370), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2369), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2371), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_46(address_space: &mut AddressSpace) {
    // VariableType
    let name = "DiscreteItemType";
    let node_id = NodeId::new(0, 2372);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), true, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_47(address_space: &mut AddressSpace) {
    // VariableType
    let name = "TwoStateDiscreteType";
    let node_id = NodeId::new(0, 2373);
    let node = VariableType::new(&node_id, name, name, NodeId::new(0, 1), false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2374), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2375), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_48(address_space: &mut AddressSpace) {
    // VariableType
    let name = "MultiStateDiscreteType";
    let node_id = NodeId::new(0, 2376);
    let node = VariableType::new(&node_id, name, name, NodeId::new(0, 28), false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 2377), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_49(address_space: &mut AddressSpace) {
    // VariableType
    let name = "MultiStateValueDiscreteType";
    let node_id = NodeId::new(0, 11238);
    let node = VariableType::new(&node_id, name, name, NodeId::new(0, 26), false, -2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 11241), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 11461), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2372), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_50(address_space: &mut AddressSpace) {
    // VariableType
    let name = "ArrayItemType";
    let node_id = NodeId::new(0, 12021);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), true, 0);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12024), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12025), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12026), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12027), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12028), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 2365), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_51(address_space: &mut AddressSpace) {
    // VariableType
    let name = "YArrayItemType";
    let node_id = NodeId::new(0, 12029);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), false, 1);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12037), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_52(address_space: &mut AddressSpace) {
    // VariableType
    let name = "XYArrayItemType";
    let node_id = NodeId::new(0, 12038);
    let node = VariableType::new(&node_id, name, name, NodeId::new(0, 12080), false, 1);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12046), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_53(address_space: &mut AddressSpace) {
    // VariableType
    let name = "ImageItemType";
    let node_id = NodeId::new(0, 12047);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), false, 2);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12055), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12056), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_54(address_space: &mut AddressSpace) {
    // VariableType
    let name = "CubeItemType";
    let node_id = NodeId::new(0, 12057);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), false, 3);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12065), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12066), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12067), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

fn add_variabletype_55(address_space: &mut AddressSpace) {
    // VariableType
    let name = "NDimensionArrayItemType";
    let node_id = NodeId::new(0, 12068);
    let node = VariableType::new(&node_id, name, name, NodeId::null(), false, 0);
    address_space.insert(node, Some(&[
        (&NodeId::new(0, 12076), ReferenceTypeId::HasProperty, ReferenceDirection::Forward),
        (&NodeId::new(0, 12021), ReferenceTypeId::HasSubtype, ReferenceDirection::Inverse),
    ]));
}

