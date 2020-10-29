use crate::error::msg::SemError;
use crate::sym::{SymTables, TermSym, TypeSym};
use crate::vm::*;

use dora_parser::ast::visit::*;
use dora_parser::ast::Expr::*;
use dora_parser::ast::Stmt::*;
use dora_parser::ast::*;
use dora_parser::interner::Name;
use dora_parser::lexer::position::Position;

use crate::semck::globaldef::report_term_shadow;
use crate::sym::TermSym::{
    SymClassConstructor, SymClassConstructorAndModule, SymConst, SymFct, SymGlobal, SymModule,
    SymNamespace, SymStructConstructor, SymStructConstructorAndModule, SymVar,
};
use crate::sym::TypeSym::{SymClass, SymEnum, SymStruct, SymTypeParam};
use crate::ty::SourceType;

pub fn check(vm: &VM) {
    for fct in vm.fcts.iter() {
        let fct = fct.read();

        if !fct.is_src() {
            continue;
        }

        let src = fct.src();
        let mut src = src.write();

        let mut nameck = NameCheck {
            vm,
            fct: &fct,
            src: &mut src,
            ast: &fct.ast,
            sym: SymTables::current(vm, fct.namespace_id),
        };

        nameck.check();
    }
}

struct NameCheck<'a> {
    vm: &'a VM,
    fct: &'a Fct,
    src: &'a mut FctSrc,
    ast: &'a Function,
    sym: SymTables,
}

impl<'a> NameCheck<'a> {
    fn check(&mut self) {
        self.sym.push_level();

        if self.fct.has_self() {
            // add hidden this parameter for ctors and methods
            self.add_hidden_parameter_self();
        }

        let cls_type_params_count = if let FctParent::Class(cls_id) = self.fct.parent {
            let cls = self.vm.classes.idx(cls_id);
            let cls = cls.read();

            for (tpid, tp) in cls.type_params.iter().enumerate() {
                self.sym.insert_type(tp.name, SymTypeParam(tpid.into()));
            }

            cls.type_params.len()
        } else {
            0
        };

        if let Some(ref type_params) = self.fct.ast.type_params {
            for (tpid, tp) in type_params.iter().enumerate() {
                self.sym
                    .insert_type(tp.name, SymTypeParam((cls_type_params_count + tpid).into()));
            }
        }

        for p in &self.ast.params {
            self.visit_param(p);
        }

        {
            let block = self.ast.block();

            for stmt in &block.stmts {
                self.visit_stmt(stmt);
            }

            if let Some(ref value) = block.expr {
                self.visit_expr(value);
            }
        }

        self.sym.pop_level();
    }

    pub fn add_hidden_parameter_self(&mut self) {
        let ty = match self.fct.parent {
            FctParent::Class(cls_id) => {
                let cls = self.vm.classes.idx(cls_id);
                let cls = cls.read();

                cls.ty
            }

            FctParent::Impl(impl_id) => {
                let ximpl = self.vm.impls[impl_id].read();
                let cls = self.vm.classes.idx(ximpl.cls_id(self.vm));
                let cls = cls.read();

                cls.ty
            }

            FctParent::Extension(extension_id) => {
                let extension = self.vm.extensions[extension_id].read();
                extension.ty
            }

            _ => unreachable!(),
        };

        let ast_id = self.fct.ast.id;
        let name = self.vm.interner.intern("self");

        let var = Var {
            id: VarId(0),
            name,
            ty,
            reassignable: false,
            node_id: ast_id,
        };

        self.src.vars.push(var);
    }

    pub fn add_var(&mut self, mut var: Var, pos: Position) -> VarId {
        let name = var.name;
        let var_id = VarId(self.src.vars.len());

        var.id = var_id;

        match self.sym.insert_term(name, SymVar(var_id)) {
            Some(SymVar(_)) | None => {}
            Some(sym) => report_term_shadow(self.vm, name, self.fct.file, pos, sym),
        }
        self.src.vars.push(var);

        var_id
    }

    fn check_stmt_let(&mut self, let_decl: &StmtLetType) {
        if let Some(ref expr) = let_decl.expr {
            self.visit_expr(expr);
        }

        self.check_stmt_let_pattern(&let_decl.pattern, let_decl.reassignable);
    }

    fn check_stmt_let_pattern(&mut self, pattern: &LetPattern, reassignable: bool) {
        match pattern {
            LetPattern::Ident(ref ident) => {
                let var_ctxt = Var {
                    id: VarId(0),
                    name: ident.name,
                    reassignable: reassignable || ident.mutable,
                    ty: SourceType::Unit,
                    node_id: ident.id,
                };

                let var_id = self.add_var(var_ctxt, ident.pos);
                self.src.map_vars.insert(ident.id, var_id)
            }

            LetPattern::Underscore(_) => {
                // no variable to declare
            }

            LetPattern::Tuple(ref tuple) => {
                for sub in &tuple.parts {
                    self.check_stmt_let_pattern(sub, reassignable);
                }
            }
        }
    }

    fn check_stmt_for(&mut self, fl: &StmtForType) {
        self.visit_expr(&fl.expr);

        self.sym.push_level();

        self.check_stmt_let_pattern(&fl.pattern, false);

        self.visit_stmt(&fl.block);
        self.sym.pop_level();
    }

    fn check_expr_ident(&mut self, ident: &ExprIdentType) {
        let sym_term = self.sym.get_term(ident.name);
        let sym_type = self.sym.get_type(ident.name);
        self.find_sym(sym_term, sym_type, ident.name, ident.id, ident.pos);
    }

    fn find_sym(
        &mut self,
        sym_term: Option<TermSym>,
        sym_type: Option<TypeSym>,
        name: Name,
        node_id: NodeId,
        node_pos: Position,
    ) {
        match (sym_term, sym_type) {
            (Some(SymVar(id)), None) => {
                self.src.map_idents.insert(node_id, IdentType::Var(id));
            }

            (Some(SymGlobal(id)), None) => {
                self.src.map_idents.insert(node_id, IdentType::Global(id));
            }

            (Some(SymConst(id)), None) => {
                self.src.map_idents.insert(node_id, IdentType::Const(id));
            }

            (Some(SymFct(id)), None) => {
                self.src.map_idents.insert(node_id, IdentType::Fct(id));
            }

            (Some(SymModule(id)), None) => {
                self.src.map_idents.insert(node_id, IdentType::Module(id));
            }

            (None, Some(SymStruct(id))) => {
                self.src.map_idents.insert(node_id, IdentType::Struct(id));
            }

            (None, Some(SymClass(id))) => {
                self.src.map_idents.insert(node_id, IdentType::Class(id));
            }

            (None, Some(SymTypeParam(id))) => {
                let ty = SourceType::TypeParam(id);
                self.src
                    .map_idents
                    .insert(node_id, IdentType::TypeParam(ty))
            }

            (None, Some(SymEnum(id))) => self.src.map_idents.insert(node_id, IdentType::Enum(id)),

            (Some(SymModule(module_id)), Some(SymClass(class_id)))
            | (Some(SymClassConstructorAndModule(_, module_id)), Some(SymClass(class_id))) => self
                .src
                .map_idents
                .insert(node_id, IdentType::ClassAndModule(class_id, module_id)),

            (Some(SymClassConstructor(id)), _) => {
                self.src.map_idents.insert(node_id, IdentType::Class(id))
            }

            (Some(SymModule(module_id)), Some(SymStruct(struct_id)))
            | (Some(SymStructConstructorAndModule(_, module_id)), Some(SymStruct(struct_id))) => {
                self.src
                    .map_idents
                    .insert(node_id, IdentType::StructAndModule(struct_id, module_id))
            }

            (Some(SymStructConstructor(id)), _) => {
                self.src.map_idents.insert(node_id, IdentType::Struct(id))
            }

            (Some(SymNamespace(id)), _) => self
                .src
                .map_idents
                .insert(node_id, IdentType::Namespace(id)),

            (None, None) => {
                let name = self.vm.interner.str(name).to_string();
                report(
                    self.vm,
                    self.fct.file,
                    node_pos,
                    SemError::UnknownIdentifier(name),
                );
            }

            (term_sym, type_sym) => unreachable!(format!("{:?} {:?}", term_sym, type_sym)),
        }
    }

    fn check_expr_path(&mut self, path: &ExprPathType) {
        self.visit_expr(&path.lhs);
        // do not check right hand site of path

        let lhs_ident_type = self.src.map_idents.get(path.lhs.id()).cloned();

        if let Some(lhs_ident_type) = lhs_ident_type {
            match lhs_ident_type {
                IdentType::Namespace(namespace_id) => {
                    if let Some(rhs_ident) = path.rhs.to_ident() {
                        let namespace = &self.vm.namespaces[namespace_id.to_usize()];
                        let table = namespace.table.read();
                        let sym_term = table.get_term(rhs_ident.name);
                        let sym_type = table.get_type(rhs_ident.name);
                        self.find_sym(sym_term, sym_type, rhs_ident.name, path.id, path.pos);
                    }
                }

                _ => {}
            }
        }
    }

    fn check_expr_dot(&mut self, dot: &ExprDotType) {
        self.visit_expr(&dot.lhs);
        // do not check right hand site of dot
    }

    fn check_expr_block(&mut self, block: &ExprBlockType) {
        self.sym.push_level();

        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }

        if let Some(ref expr) = block.expr {
            self.visit_expr(expr);
        }

        self.sym.pop_level();
    }
}

impl<'a> Visitor for NameCheck<'a> {
    fn visit_param(&mut self, p: &Param) {
        let var_ctxt = Var {
            id: VarId(0),
            name: p.name,
            reassignable: false,
            ty: SourceType::Unit,
            node_id: p.id,
        };

        // params are only allowed to replace functions, vars cannot be replaced
        let term_sym = self.sym.get_term(p.name);
        match term_sym {
            Some(SymFct(_)) | None => {
                let var_id = self.add_var(var_ctxt, p.pos);
                self.src.map_vars.insert(p.id, var_id)
            }
            Some(conflict_sym) => {
                report_term_shadow(self.vm, p.name, self.fct.file, p.pos, conflict_sym)
            }
        }
    }

    fn visit_stmt(&mut self, s: &Stmt) {
        match *s {
            StmtLet(ref stmt) => self.check_stmt_let(stmt),
            StmtFor(ref stmt) => self.check_stmt_for(stmt),

            // no need to handle rest of statements
            _ => visit::walk_stmt(self, s),
        }
    }

    fn visit_expr(&mut self, e: &Expr) {
        match e {
            &ExprIdent(ref ident) => self.check_expr_ident(ident),
            &ExprPath(ref path) => self.check_expr_path(path),
            &ExprDot(ref dot) => self.check_expr_dot(dot),
            &ExprBlock(ref block) => self.check_expr_block(block),

            // no need to handle rest of expressions
            _ => visit::walk_expr(self, e),
        }
    }
}

fn report(vm: &VM, file: FileId, pos: Position, msg: SemError) {
    vm.diag.lock().report(file, pos, msg);
}

fn str(vm: &VM, name: Name) -> String {
    vm.interner.str(name).to_string()
}

#[cfg(test)]
mod tests {
    use crate::error::msg::SemError;
    use crate::error::msg::SemError::ShadowClassConstructor;
    use crate::semck::tests::*;

    #[test]
    fn multiple_functions() {
        ok("fun f() {}\nfun g() {}");
    }

    #[test]
    fn redefine_function() {
        err(
            "fun f() {}\nfun f() {}",
            pos(2, 1),
            SemError::ShadowFunction("f".into()),
        );
    }

    #[test]
    fn shadow_type_with_function() {
        err(
            "fun Int32() {}",
            pos(1, 1),
            SemError::ShadowClassConstructor("Int32".into()),
        );
    }

    #[test]
    fn shadow_type_with_param() {
        err(
            "fun test(Bool: String) {}",
            pos(1, 10),
            ShadowClassConstructor("Bool".into()),
        );
    }

    #[test]
    fn shadow_type_with_var() {
        ok("fun test() { let String = 3; }");
    }

    #[test]
    fn shadow_function() {
        ok("fun f() { let f = 1; }");
        err(
            "fun f() { let f = 1; f(); }",
            pos(1, 23),
            SemError::UnknownMethod("Int32".into(), "get".into(), Vec::new()),
        );
    }

    #[test]
    fn shadow_var() {
        ok("fun f() { let f = 1; let f = 2; }");
    }

    #[test]
    fn shadow_param() {
        err(
            "fun f(a: Int32, b: Int32, a: String) {}",
            pos(1, 27),
            SemError::ShadowParam("a".into()),
        );
    }

    #[test]
    fn multiple_params() {
        ok("fun f(a: Int32, b: Int32, c:String) {}");
    }

    #[test]
    fn undefined_variable() {
        err(
            "fun f() { let b = a; }",
            pos(1, 19),
            SemError::UnknownIdentifier("a".into()),
        );
        err(
            "fun f() { a; }",
            pos(1, 11),
            SemError::UnknownIdentifier("a".into()),
        );
    }

    #[test]
    fn undefined_function() {
        err(
            "fun f() { foo(); }",
            pos(1, 11),
            SemError::UnknownIdentifier("foo".into()),
        );
    }

    #[test]
    fn recursive_function_call() {
        ok("fun f() { f(); }");
    }

    #[test]
    fn function_call() {
        ok("fun a() {}\nfun b() { a(); }");

        // non-forward definition of functions
        ok("fun a() { b(); }\nfun b() {}");
    }

    #[test]
    fn variable_outside_of_scope() {
        err(
            "fun f(): Int32 { { let a = 1; } return a; }",
            pos(1, 40),
            SemError::UnknownIdentifier("a".into()),
        );

        ok("fun f(): Int32 { let a = 1; { let a = 2; } return a; }");
    }

    #[test]
    fn const_value() {
        ok("const one: Int32 = 1;
            fun f(): Int32 { return one; }");
    }

    #[test]
    fn for_var() {
        ok("fun f() { for i in range(0, 4) { i; } }");
    }

    #[test]
    #[ignore]
    fn namespace_fct_call() {
        ok("
            fun f() { foo::g(); }
            namespace foo { fun g() {} }
        ");
    }
}
