use parking_lot::RwLock;

use std::collections::hash_map::HashMap;
use std::convert::TryInto;
use std::ops::Index;
use std::sync::Arc;

use dora_parser::ast;
use dora_parser::interner::Name;
use dora_parser::lexer::position::Position;

use crate::semck::specialize::replace_type_param;
use crate::ty::{SourceType, SourceTypeArray};
use crate::utils::GrowableVec;
use crate::vm::{
    accessible_from, extension_matches, impl_matches, namespace_path, Candidate, ClassDefId,
    ExtensionId, FileId, ImplId, NamespaceId, TypeParam, TypeParamId, VM,
};

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
    pub file_id: FileId,
    pub namespace_id: NamespaceId,
    pub ast: Arc<ast::Enum>,
    pub pos: Position,
    pub name: Name,
    pub is_pub: bool,
    pub type_params: Vec<TypeParam>,
    pub variants: Vec<EnumVariant>,
    pub name_to_value: HashMap<Name, u32>,
    pub impls: Vec<ImplId>,
    pub extensions: Vec<ExtensionId>,
    pub specializations: RwLock<HashMap<SourceTypeArray, EnumDefId>>,
    pub simple_enumeration: bool,
}

impl EnumData {
    pub fn type_param(&self, id: TypeParamId) -> &TypeParam {
        &self.type_params[id.to_usize()]
    }

    pub fn name(&self, vm: &VM) -> String {
        namespace_path(vm, self.namespace_id, self.name)
    }

    pub fn name_with_params(&self, vm: &VM, type_list: &SourceTypeArray) -> String {
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
    pub id: usize,
    pub name: Name,
    pub types: Vec<SourceType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct EnumDefId(u32);

impl From<usize> for EnumDefId {
    fn from(data: usize) -> EnumDefId {
        EnumDefId(data as u32)
    }
}

impl GrowableVec<EnumDef> {
    pub fn idx(&self, index: EnumDefId) -> Arc<EnumDef> {
        self.idx_usize(index.0 as usize)
    }
}

#[derive(Debug)]
pub struct EnumDef {
    pub id: EnumDefId,
    pub enum_id: EnumId,
    pub type_params: SourceTypeArray,
    pub layout: EnumLayout,
    pub variants: RwLock<Vec<Option<ClassDefId>>>,
}

impl EnumDef {
    pub fn field_id(&self, xenum: &EnumData, variant_id: usize, element: u32) -> u32 {
        let variant = &xenum.variants[variant_id];
        let mut units = 0;

        for ty in &variant.types[0..element as usize] {
            if ty.is_unit() {
                units += 1;
            }
        }

        1 + element - units
    }
}

#[derive(Copy, Clone, Debug)]
pub enum EnumLayout {
    Int,
    Ptr,
    Tagged,
}

#[derive(Debug)]
pub struct EnumDefVariant {
    pub types: Vec<SourceType>,
}

pub fn find_methods_in_enum(
    vm: &VM,
    object_type: SourceType,
    type_param_defs: &[TypeParam],
    name: Name,
    is_static: bool,
) -> Vec<Candidate> {
    let enum_id = object_type.enum_id().unwrap();
    let xenum = vm.enums[enum_id].read();

    for &extension_id in &xenum.extensions {
        if !extension_matches(vm, object_type.clone(), type_param_defs, extension_id) {
            continue;
        }

        let extension = vm.extensions[extension_id].read();

        let table = if is_static {
            &extension.static_names
        } else {
            &extension.instance_names
        };

        if let Some(&fct_id) = table.get(&name) {
            let ext_ty = extension.ty.clone();
            let type_params = object_type.type_params(vm);
            let ext_ty = replace_type_param(vm, ext_ty, &type_params, None);
            return vec![Candidate {
                object_type: ext_ty,
                fct_id,
            }];
        }
    }

    let mut candidates = Vec::new();

    for &impl_id in &xenum.impls {
        if !impl_matches(vm, object_type.clone(), type_param_defs, impl_id) {
            continue;
        }

        let ximpl = vm.impls[impl_id].read();

        for &method in &ximpl.methods {
            let method = vm.fcts.idx(method);
            let method = method.read();

            if method.name == name && method.is_static == is_static {
                let impl_ty = ximpl.ty.clone();
                let type_params = object_type.type_params(vm);
                let impl_ty = replace_type_param(vm, impl_ty, &type_params, None);
                candidates.push(Candidate {
                    object_type: impl_ty,
                    fct_id: method.id,
                });
            }
        }
    }

    candidates
}

pub fn enum_accessible_from(vm: &VM, enum_id: EnumId, namespace_id: NamespaceId) -> bool {
    let xenum = vm.enums[enum_id].read();

    accessible_from(vm, xenum.namespace_id, xenum.is_pub, namespace_id)
}
