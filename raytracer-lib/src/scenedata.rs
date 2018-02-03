use lib::object::{Analytical, Raymarchable};
use lib::material::Material;
use std::rc::Rc;

pub struct ObjectData {
    pub object: Rc<Raymarchable>,
    pub bound: Option<Rc<Analytical>>,
    pub material: Rc<Material>,
}

impl Clone for ObjectData {
    fn clone(&self) -> Self {
        return Self {
            object: Rc::clone(&self.object),
            material: Rc::clone(&self.material),
            bound: match self.bound {
                Some(ref v) => Some(Rc::clone(v)),
                None => None,
            },
        };
    }
}
