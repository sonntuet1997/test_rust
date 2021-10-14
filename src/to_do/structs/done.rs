use super::base::Base;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;
pub struct Done {
    pub super_struct: Base,
}
impl Done {
    pub fn new(input_title: &str) -> Done {
        let base = Base::new(input_title, "doneeee");
        return Done { super_struct: base };
    }
}
impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
