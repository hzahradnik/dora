use num_traits::cast::FromPrimitive;

use crate::bytecode::{
    BytecodeFunction, BytecodeOffset, BytecodeOpcode, ConstPoolIdx, OperandWidth, Register,
};
use crate::vm::{ClassDefId, FieldId, GlobalId, TupleId};

pub fn read<T: BytecodeVisitor>(data: &[u8], visitor: &mut T) {
    BytecodeReader::new(data, visitor).read();
}

struct BytecodeReader<'a, T: BytecodeVisitor> {
    data: &'a [u8],
    pos: usize,
    visitor: &'a mut T,
}

impl<'a, T> BytecodeReader<'a, T>
where
    T: BytecodeVisitor,
{
    fn new(data: &'a [u8], visitor: &'a mut T) -> BytecodeReader<'a, T> {
        BytecodeReader {
            data: data,
            pos: 0,
            visitor: visitor,
        }
    }

    fn read(&mut self) {
        while self.pos < self.data.len() {
            let start = self.pos;
            self.visitor
                .visit_instruction(BytecodeOffset(self.pos as u32));
            let width = self.read_operand_width();
            let opcode = self.read_opcode();
            self.read_instruction(width, opcode);
            let end = self.pos;

            debug_assert_eq!(
                end - start,
                opcode.size(width) as usize,
                "bug in BytecodeOpcode::size() with {:?} and width {:?}",
                opcode,
                width
            );
        }
    }

    fn read_instruction(&mut self, width: OperandWidth, opcode: BytecodeOpcode) {
        match opcode {
            BytecodeOpcode::Wide => unreachable!(),

            BytecodeOpcode::AddInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_add_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::AddInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_add_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::AddFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_add_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::AddFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_add_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::SubInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sub_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::SubInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sub_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::SubFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sub_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::SubFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sub_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::NegInt32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_neg_int32(dest, src);
            }
            BytecodeOpcode::NegInt64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_neg_int64(dest, src);
            }
            BytecodeOpcode::NegFloat32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_neg_float32(dest, src);
            }
            BytecodeOpcode::NegFloat64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_neg_float64(dest, src);
            }
            BytecodeOpcode::MulInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mul_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::MulInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mul_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::MulFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mul_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::MulFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mul_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::DivInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_div_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::DivInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_div_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::DivFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_div_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::DivFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_div_float64(dest, lhs, rhs);
            }

            BytecodeOpcode::ModInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mod_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::ModInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_mod_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::AndInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_and_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::AndInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_and_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::OrInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_or_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::OrInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_or_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::XorInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_xor_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::XorInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_xor_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::NotBool => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_not_bool(dest, src);
            }
            BytecodeOpcode::NotInt32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_not_int32(dest, src);
            }
            BytecodeOpcode::NotInt64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_not_int64(dest, src);
            }

            BytecodeOpcode::ShlInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_shl_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::ShrInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_shr_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::SarInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sar_int32(dest, lhs, rhs);
            }

            BytecodeOpcode::ShlInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_shl_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::ShrInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_shr_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::SarInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_sar_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::RolInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_rol_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::RorInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_ror_int32(dest, lhs, rhs);
            }

            BytecodeOpcode::RolInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_rol_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::RorInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_ror_int64(dest, lhs, rhs);
            }

            BytecodeOpcode::ExtendUInt8ToChar => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_extend_byte_to_char(dest, src);
            }
            BytecodeOpcode::ExtendUInt8ToInt32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_extend_byte_to_int32(dest, src);
            }
            BytecodeOpcode::ExtendUInt8ToInt64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_extend_byte_to_int64(dest, src);
            }
            BytecodeOpcode::ExtendInt32ToInt64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_extend_int32_to_int64(dest, src);
            }
            BytecodeOpcode::ExtendCharToInt64 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_extend_char_to_int64(dest, src);
            }
            BytecodeOpcode::CastCharToInt32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_char_to_int32(dest, src);
            }
            BytecodeOpcode::CastInt32ToUInt8 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_int32_to_uint8(dest, src);
            }
            BytecodeOpcode::CastInt32ToChar => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_int32_to_char(dest, src);
            }
            BytecodeOpcode::CastInt64ToUInt8 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_int64_to_uint8(dest, src);
            }
            BytecodeOpcode::CastInt64ToChar => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_int64_to_char(dest, src);
            }
            BytecodeOpcode::CastInt64ToInt32 => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_cast_int64_to_int32(dest, src);
            }

            BytecodeOpcode::InstanceOf => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                let cls_id = self.read_const_pool_idx(width);
                self.visitor.visit_instance_of(dest, src, cls_id);
            }
            BytecodeOpcode::CheckedCast => {
                let src = self.read_register(width);
                let cls_id = self.read_const_pool_idx(width);
                self.visitor.visit_checked_cast(src, cls_id);
            }

            BytecodeOpcode::Mov => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                self.visitor.visit_mov(dest, src);
            }

            BytecodeOpcode::LoadTupleElement => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                let tuple = self.read_tuple(width);
                let element = self.read_index(width);
                self.visitor
                    .visit_load_tuple_element(dest, src, tuple, element);
            }

            BytecodeOpcode::LoadEnumElement => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                let element = self.read_index(width);
                self.visitor
                    .visit_load_enum_element(dest, src, idx, element);
            }

            BytecodeOpcode::LoadEnumVariant => {
                let dest = self.read_register(width);
                let src = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_load_enum_variant(dest, src, idx);
            }

            BytecodeOpcode::LoadStructField => {
                let dest = self.read_register(width);
                let obj = self.read_register(width);
                let field = self.read_const_pool_idx(width);
                self.visitor.visit_load_struct_field(dest, obj, field);
            }

            BytecodeOpcode::LoadField => {
                let dest = self.read_register(width);
                let obj = self.read_register(width);
                let field = self.read_const_pool_idx(width);
                self.visitor.visit_load_field(dest, obj, field);
            }

            BytecodeOpcode::StoreField => {
                let src = self.read_register(width);
                let obj = self.read_register(width);
                let field = self.read_const_pool_idx(width);
                self.visitor.visit_store_field(src, obj, field);
            }

            BytecodeOpcode::LoadGlobal => {
                let dest = self.read_register(width);
                let glob = self.read_global(width);
                self.visitor.visit_load_global(dest, glob);
            }

            BytecodeOpcode::StoreGlobal => {
                let src = self.read_register(width);
                let glob = self.read_global(width);
                self.visitor.visit_store_global(src, glob);
            }

            BytecodeOpcode::PushRegister => {
                let src = self.read_register(width);
                self.visitor.visit_push_register(src);
            }

            BytecodeOpcode::ConstTrue => {
                let dest = self.read_register(width);
                self.visitor.visit_const_true(dest);
            }
            BytecodeOpcode::ConstFalse => {
                let dest = self.read_register(width);
                self.visitor.visit_const_false(dest);
            }
            BytecodeOpcode::ConstZeroUInt8 => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_uint8(dest);
            }
            BytecodeOpcode::ConstZeroChar => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_char(dest);
            }
            BytecodeOpcode::ConstZeroInt32 => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_int32(dest);
            }
            BytecodeOpcode::ConstZeroInt64 => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_int64(dest);
            }
            BytecodeOpcode::ConstZeroFloat32 => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_float32(dest);
            }
            BytecodeOpcode::ConstZeroFloat64 => {
                let dest = self.read_register(width);
                self.visitor.visit_const_zero_float64(dest);
            }
            BytecodeOpcode::ConstChar => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_char(dest, idx);
            }
            BytecodeOpcode::ConstUInt8 => {
                let dest = self.read_register(width);
                let value = self.read_byte();
                self.visitor.visit_const_uint8(dest, value as u8);
            }
            BytecodeOpcode::ConstInt32 => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_int32(dest, idx);
            }
            BytecodeOpcode::ConstInt64 => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_int64(dest, idx);
            }
            BytecodeOpcode::ConstFloat32 => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_float32(dest, idx);
            }
            BytecodeOpcode::ConstFloat64 => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_float64(dest, idx);
            }
            BytecodeOpcode::ConstString => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_const_string(dest, idx);
            }

            BytecodeOpcode::TestIdentity => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_identity(dest, lhs, rhs);
            }

            BytecodeOpcode::TestEqBool => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_bool(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeBool => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_bool(dest, lhs, rhs);
            }
            BytecodeOpcode::TestEqUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeUInt8 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_uint8(dest, lhs, rhs);
            }
            BytecodeOpcode::TestEqChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeChar => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_char(dest, lhs, rhs);
            }
            BytecodeOpcode::TestEqEnum => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_enum(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeEnum => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_enum(dest, lhs, rhs);
            }
            BytecodeOpcode::TestEqInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeInt64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_int64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestEqInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_int32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeInt32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_int32(dest, lhs, rhs);
            }

            BytecodeOpcode::TestEqFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_float32(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeFloat32 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_float32(dest, lhs, rhs);
            }

            BytecodeOpcode::TestEqFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_eq_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestNeFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ne_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGtFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_gt_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestGeFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_ge_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLtFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_lt_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::TestLeFloat64 => {
                let dest = self.read_register(width);
                let lhs = self.read_register(width);
                let rhs = self.read_register(width);
                self.visitor.visit_test_le_float64(dest, lhs, rhs);
            }
            BytecodeOpcode::Assert => {
                let value = self.read_register(width);
                self.visitor.visit_assert(value);
            }

            BytecodeOpcode::JumpLoop => {
                let offset = self.read_offset(width);
                self.visitor.visit_jump_loop(offset);
            }
            BytecodeOpcode::LoopStart => {
                self.visitor.visit_loop_start();
            }
            BytecodeOpcode::JumpIfFalse => {
                let opnd = self.read_register(width);
                let offset = self.read_offset(width);
                self.visitor.visit_jump_if_false(opnd, offset);
            }
            BytecodeOpcode::JumpIfFalseConst => {
                let opnd = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_jump_if_false_const(opnd, idx);
            }
            BytecodeOpcode::JumpIfTrue => {
                let opnd = self.read_register(width);
                let offset = self.read_offset(width);
                self.visitor.visit_jump_if_true(opnd, offset);
            }
            BytecodeOpcode::JumpIfTrueConst => {
                let opnd = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_jump_if_true_const(opnd, idx);
            }
            BytecodeOpcode::Jump => {
                let offset = self.read_offset(width);
                self.visitor.visit_jump(offset);
            }
            BytecodeOpcode::JumpConst => {
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_jump_const(idx);
            }

            BytecodeOpcode::InvokeDirectVoid => {
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_direct_void(fct);
            }
            BytecodeOpcode::InvokeDirect => {
                let dest = self.read_register(width);
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_direct(dest, fct);
            }

            BytecodeOpcode::InvokeVirtualVoid => {
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_virtual_void(fct);
            }
            BytecodeOpcode::InvokeVirtual => {
                let dest = self.read_register(width);
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_virtual(dest, fct);
            }

            BytecodeOpcode::InvokeStaticVoid => {
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_static_void(fct);
            }
            BytecodeOpcode::InvokeStatic => {
                let dest = self.read_register(width);
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_static(dest, fct);
            }

            BytecodeOpcode::InvokeGenericStaticVoid => {
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_generic_static_void(fct);
            }
            BytecodeOpcode::InvokeGenericStatic => {
                let dest = self.read_register(width);
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_generic_static(dest, fct);
            }

            BytecodeOpcode::InvokeGenericDirectVoid => {
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_generic_direct_void(fct);
            }
            BytecodeOpcode::InvokeGenericDirect => {
                let dest = self.read_register(width);
                let fct = self.read_const_pool_idx(width);
                self.visitor.visit_invoke_generic_direct(dest, fct);
            }

            BytecodeOpcode::NewObject => {
                let dest = self.read_register(width);
                let cls = self.read_const_pool_idx(width);
                self.visitor.visit_new_object(dest, cls);
            }
            BytecodeOpcode::NewArray => {
                let dest = self.read_register(width);
                let cls = self.read_const_pool_idx(width);
                let length = self.read_register(width);
                self.visitor.visit_new_array(dest, cls, length);
            }
            BytecodeOpcode::NewTuple => {
                let dest = self.read_register(width);
                let tuple = self.read_tuple(width);
                self.visitor.visit_new_tuple(dest, tuple);
            }
            BytecodeOpcode::NewEnum => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_new_enum(dest, idx);
            }
            BytecodeOpcode::NewStruct => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                self.visitor.visit_new_struct(dest, idx);
            }
            BytecodeOpcode::NewTraitObject => {
                let dest = self.read_register(width);
                let idx = self.read_const_pool_idx(width);
                let src = self.read_register(width);
                self.visitor.visit_new_trait_object(dest, idx, src);
            }

            BytecodeOpcode::NilCheck => {
                let obj = self.read_register(width);
                self.visitor.visit_nil_check(obj);
            }

            BytecodeOpcode::ArrayLength => {
                let dest = self.read_register(width);
                let array = self.read_register(width);
                self.visitor.visit_array_length(dest, array);
            }
            BytecodeOpcode::ArrayBoundCheck => {
                let array = self.read_register(width);
                let index = self.read_register(width);
                self.visitor.visit_array_bound_check(array, index);
            }

            BytecodeOpcode::LoadArray => {
                let dest = self.read_register(width);
                let array = self.read_register(width);
                let index = self.read_register(width);
                self.visitor.visit_load_array(dest, array, index);
            }

            BytecodeOpcode::StoreArray => {
                let src = self.read_register(width);
                let array = self.read_register(width);
                let index = self.read_register(width);
                self.visitor.visit_store_array(src, array, index);
            }

            BytecodeOpcode::RetVoid => {
                self.visitor.visit_ret_void();
            }
            BytecodeOpcode::Ret => {
                let opnd = self.read_register(width);
                self.visitor.visit_ret(opnd);
            }
        }
    }

    fn read_register(&mut self, width: OperandWidth) -> Register {
        Register(self.read_index(width) as usize)
    }

    fn read_class(&mut self, width: OperandWidth) -> ClassDefId {
        (self.read_index(width) as usize).into()
    }

    fn read_field(&mut self, width: OperandWidth) -> FieldId {
        (self.read_index(width) as usize).into()
    }

    fn read_tuple(&mut self, width: OperandWidth) -> TupleId {
        self.read_index(width).into()
    }

    fn read_global(&mut self, width: OperandWidth) -> GlobalId {
        self.read_index(width).into()
    }

    fn read_opcode(&mut self) -> BytecodeOpcode {
        let opcode = self.read_byte();
        FromPrimitive::from_u32(opcode).expect("illegal opcode")
    }

    fn read_const_pool_idx(&mut self, wide: OperandWidth) -> ConstPoolIdx {
        (self.read_index(wide) as usize).into()
    }

    fn read_offset(&mut self, wide: OperandWidth) -> u32 {
        self.read_index(wide)
    }

    fn read_index(&mut self, width: OperandWidth) -> u32 {
        match width {
            OperandWidth::Normal => self.read_byte(),
            OperandWidth::Wide => self.read_wide(),
        }
    }

    fn read_operand_width(&mut self) -> OperandWidth {
        if self.data[self.pos] as u32 == BytecodeOpcode::Wide as u32 {
            self.pos += 1;
            OperandWidth::Wide
        } else {
            OperandWidth::Normal
        }
    }

    fn read_byte(&mut self) -> u32 {
        let value = self.data[self.pos];
        self.pos += 1;
        value as u32
    }

    fn read_wide(&mut self) -> u32 {
        let v1 = self.read_byte();
        let v2 = self.read_byte();
        let v3 = self.read_byte();
        let v4 = self.read_byte();

        (v4 << 24) | (v3 << 16) | (v2 << 8) | v1
    }
}

pub trait BytecodeVisitor {
    fn visit_instruction(&mut self, _offset: BytecodeOffset) {}

    fn visit_add_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_add_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_add_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_add_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_sub_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_sub_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_sub_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_sub_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_neg_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_neg_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_neg_float32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_neg_float64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }

    fn visit_mul_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_mul_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_mul_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_mul_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_div_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_div_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_div_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_div_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_mod_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_mod_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_and_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_and_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_or_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_or_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_xor_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_xor_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_not_bool(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_not_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_not_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }

    fn visit_shl_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_shr_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_sar_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_shl_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_shr_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_sar_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_rol_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_ror_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_rol_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_ror_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_extend_byte_to_char(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_extend_byte_to_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_extend_byte_to_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_extend_int32_to_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_extend_char_to_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_char_to_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_int32_to_uint8(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_int32_to_char(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_int64_to_uint8(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_int64_to_char(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_cast_int64_to_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }

    fn visit_truncate_float32_to_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_truncate_float32_to_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_truncate_float64_to_int32(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }
    fn visit_truncate_float64_to_int64(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }

    fn visit_instance_of(&mut self, _dest: Register, _src: Register, _cls_id: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_checked_cast(&mut self, _src: Register, _cls_id: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_mov(&mut self, _dest: Register, _src: Register) {
        unimplemented!();
    }

    fn visit_load_tuple_element(
        &mut self,
        _dest: Register,
        _src: Register,
        _tuple_id: TupleId,
        _element: u32,
    ) {
        unimplemented!();
    }

    fn visit_load_enum_element(
        &mut self,
        _dest: Register,
        _src: Register,
        _idx: ConstPoolIdx,
        _element: u32,
    ) {
        unimplemented!();
    }

    fn visit_load_enum_variant(&mut self, _dest: Register, _src: Register, _idx: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_load_struct_field(&mut self, _dest: Register, _obj: Register, _field: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_load_field(&mut self, _dest: Register, _obj: Register, _field: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_store_field(&mut self, _src: Register, _obj: Register, _field: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_load_global(&mut self, _dest: Register, _glob: GlobalId) {
        unimplemented!();
    }

    fn visit_store_global(&mut self, _src: Register, _glob: GlobalId) {
        unimplemented!();
    }

    fn visit_push_register(&mut self, _src: Register) {
        unimplemented!();
    }

    fn visit_const_true(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_false(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_uint8(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_char(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_int32(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_int64(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_float32(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_zero_float64(&mut self, _dest: Register) {
        unimplemented!();
    }
    fn visit_const_char(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_const_uint8(&mut self, _dest: Register, _value: u8) {
        unimplemented!();
    }
    fn visit_const_int32(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_const_int64(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_const_float32(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_const_float64(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_const_string(&mut self, _dest: Register, _value: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_test_identity(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_test_eq_bool(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_bool(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_eq_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_uint8(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_eq_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_char(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_eq_enum(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_enum(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_test_eq_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_int32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_test_eq_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_int64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_test_eq_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_float32(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }

    fn visit_test_eq_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ne_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_gt_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_ge_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_lt_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_test_le_float64(&mut self, _dest: Register, _lhs: Register, _rhs: Register) {
        unimplemented!();
    }
    fn visit_assert(&mut self, _value: Register) {
        unimplemented!();
    }

    fn visit_jump_if_false(&mut self, _opnd: Register, _offset: u32) {
        unimplemented!();
    }
    fn visit_jump_if_false_const(&mut self, _opnd: Register, _idx: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_jump_if_true(&mut self, _opnd: Register, _offset: u32) {
        unimplemented!();
    }
    fn visit_jump_if_true_const(&mut self, _opnd: Register, _idx: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_jump_loop(&mut self, _offset: u32) {
        unimplemented!();
    }
    fn visit_loop_start(&mut self) {
        unimplemented!();
    }
    fn visit_jump(&mut self, _offset: u32) {
        unimplemented!();
    }
    fn visit_jump_const(&mut self, _idx: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_invoke_direct_void(&mut self, _fct: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_invoke_direct(&mut self, _dest: Register, _fct: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_invoke_virtual_void(&mut self, _fct: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_invoke_virtual(&mut self, _dest: Register, _fct: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_invoke_static_void(&mut self, _fct: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_invoke_static(&mut self, _dest: Register, _fct: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_invoke_generic_static_void(&mut self, _fct: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_invoke_generic_static(&mut self, _dest: Register, _fct: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_invoke_generic_direct_void(&mut self, _fct: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_invoke_generic_direct(&mut self, _dest: Register, _fct: ConstPoolIdx) {
        unimplemented!();
    }

    fn visit_new_object(&mut self, _dest: Register, _cls: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_new_array(&mut self, _dest: Register, _cls: ConstPoolIdx, _length: Register) {
        unimplemented!();
    }
    fn visit_new_tuple(&mut self, _dest: Register, _tuple: TupleId) {
        unimplemented!();
    }
    fn visit_new_enum(&mut self, _dest: Register, _idx: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_new_struct(&mut self, _dest: Register, _idx: ConstPoolIdx) {
        unimplemented!();
    }
    fn visit_new_trait_object(&mut self, _dest: Register, _idx: ConstPoolIdx, _src: Register) {
        unimplemented!();
    }

    fn visit_nil_check(&mut self, _obj: Register) {
        unimplemented!();
    }

    fn visit_array_length(&mut self, _dest: Register, _arr: Register) {
        unimplemented!();
    }
    fn visit_array_bound_check(&mut self, _arr: Register, _idx: Register) {
        unimplemented!();
    }

    fn visit_load_array(&mut self, _dest: Register, _arr: Register, _idx: Register) {
        unimplemented!();
    }

    fn visit_store_array(&mut self, _src: Register, _arr: Register, _idx: Register) {
        unimplemented!();
    }

    fn visit_ret_void(&mut self) {
        unimplemented!();
    }
    fn visit_ret(&mut self, _opnd: Register) {
        unimplemented!();
    }
}

struct BytecodeIterator<'a> {
    fct: &'a BytecodeFunction,
    pos: usize,
}

impl<'a> BytecodeIterator<'a> {
    fn new(fct: &'a BytecodeFunction) -> BytecodeIterator<'a> {
        BytecodeIterator { fct, pos: 0 }
    }
}
