use parking_lot::RwLock;

use std::collections::hash_map::HashMap;
use std::convert::TryInto;
use std::ops::Index;
use std::sync::Arc;

use dora_parser::interner::Name;
use dora_parser::lexer::position::Position;

use crate::ty::{BuiltinType, TypeList, TypeListId};
use crate::utils::GrowableVec;
use crate::vm::{ExtensionId, FctId, FileId, TypeParam, VM};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumId(u32);

impl From<usize> for EnumId {
    fn from(data: usize) -> EnumId {
        EnumId(data.try_into().unwrap())
    }
}

impl Index<EnumId> for Vec<RwLock<EnumData>> {
    type Output = RwLock<EnumData>;

    fn index(&self, index: EnumId) -> &RwLock<EnumData> {
        &self[index.0 as usize]
    }
}

#[derive(Debug)]
pub struct EnumData {
    pub id: EnumId,
    pub file: FileId,
    pub pos: Position,
    pub name: Name,
    pub type_params: Vec<TypeParam>,
    pub variants: Vec<EnumVariant>,
    pub name_to_value: HashMap<Name, u32>,
    pub extensions: Vec<ExtensionId>,
    pub specializations: RwLock<HashMap<TypeList, EnumDefId>>,
}

impl EnumData {
    pub fn type_param(&self, id: TypeListId) -> &TypeParam {
        &self.type_params[id.to_usize()]
    }

    pub fn name_with_params(&self, vm: &VM, type_list: &TypeList) -> String {
        let name = vm.interner.str(self.name);

        if type_list.len() > 0 {
            let type_list = type_list
                .iter()
                .map(|p| p.name_enum(vm, self))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{}[{}]", name, type_list)
        } else {
            name.to_string()
        }
    }
}

#[derive(Debug)]
pub struct EnumVariant {
    pub name: Name,
    pub types: Vec<BuiltinType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumDefId(u32);

impl From<usize> for EnumDefId {
    fn from(data: usize) -> EnumDefId {
        EnumDefId(data as u32)
    }
}

impl<'ast> GrowableVec<RwLock<EnumDef>> {
    pub fn idx(&self, index: EnumDefId) -> Arc<RwLock<EnumDef>> {
        self.idx_usize(index.0 as usize)
    }
}

#[derive(Debug)]
pub struct EnumDef {
    pub id: EnumDefId,
    pub enum_id: EnumId,
    pub type_params: TypeList,
    pub layout: EnumLayout,
}

#[derive(Debug)]
pub enum EnumLayout {
    Int,
    Ptr,
    Tagged,
}

#[derive(Debug)]
pub struct EnumDefVariant {
    pub types: Vec<BuiltinType>,
}

pub fn find_methods_in_enum(
    vm: &VM,
    object_type: BuiltinType,
    name: Name,
    is_static: bool,
) -> Vec<(BuiltinType, FctId)> {
    let enum_id = object_type.enum_id().unwrap();
    let xenum = vm.enums[enum_id].read();

    for &extension_id in &xenum.extensions {
        let extension = vm.extensions[extension_id].read();

        if extension.ty.type_params(vm) != object_type.type_params(vm) {
            continue;
        }

        let table = if is_static {
            &extension.static_names
        } else {
            &extension.instance_names
        };

        if let Some(&fct_id) = table.get(&name) {
            return vec![(extension.ty, fct_id)];
        }
    }

    Vec::new()
}
