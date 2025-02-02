use std::rc::Rc;

use fe_analyzer::namespace::items as analyzer_items;
use smol_str::SmolStr;

use crate::{
    db::MirDb,
    ir::{self, function::Linkage, FunctionSignature, TypeId},
    lower::function::{lower_func_body, lower_func_signature},
};

pub fn mir_lowered_func_signature(
    db: &dyn MirDb,
    analyzer_func: analyzer_items::FunctionId,
) -> ir::FunctionId {
    lower_func_signature(db, analyzer_func)
}

pub fn mir_lowered_func_body(db: &dyn MirDb, func: ir::FunctionId) -> Rc<ir::FunctionBody> {
    lower_func_body(db, func)
}

impl ir::FunctionId {
    pub fn signature(self, db: &dyn MirDb) -> Rc<FunctionSignature> {
        db.lookup_mir_intern_function(self)
    }

    pub fn return_type(self, db: &dyn MirDb) -> Option<TypeId> {
        self.signature(db).return_type
    }

    pub fn linkage(self, db: &dyn MirDb) -> Linkage {
        self.signature(db).linkage
    }

    pub fn analyzer_func(self, db: &dyn MirDb) -> analyzer_items::FunctionId {
        self.signature(db).analyzer_func_id
    }

    pub fn body(self, db: &dyn MirDb) -> Rc<ir::FunctionBody> {
        db.mir_lowered_func_body(self)
    }

    pub fn module(self, db: &dyn MirDb) -> analyzer_items::ModuleId {
        let analyzer_func = self.analyzer_func(db);
        analyzer_func.module(db.upcast())
    }

    pub fn is_contract_init(self, db: &dyn MirDb) -> bool {
        self.analyzer_func(db).is_constructor(db.upcast())
    }

    pub fn name(&self, db: &dyn MirDb) -> SmolStr {
        let analyzer_func = self.analyzer_func(db);
        analyzer_func.name(db.upcast())
    }

    /// Returns `class_name::fn_name` if a function is a method else `fn_name`.
    pub fn name_with_class(self, db: &dyn MirDb) -> SmolStr {
        let analyzer_func = self.analyzer_func(db);
        let func_name = analyzer_func.name(db.upcast());

        if let Some(class) = analyzer_func.class(db.upcast()) {
            let class_name = class.name(db.upcast());
            format!("{}::{}", class_name, func_name).into()
        } else {
            func_name
        }
    }

    pub fn returns_aggregate(self, db: &dyn MirDb) -> bool {
        self.return_type(db)
            .map(|ty| ty.is_aggregate(db))
            .unwrap_or_default()
    }
}
