use lib::light::Light;
use lib::object::Raymarchable;
use lib::material::Material;
use lib::{Colour, LightData, ObjectData, Scene};

use std::vec::Vec;
use std::sync::Arc;

pub struct SceneBuilder {
    objects: Vec<ObjectData>,
    lights: Vec<LightData>,
    background: Colour,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            background: Colour::zero(),
        }
    }

    pub fn add_object<O, M>(mut self, obj: O, mat: M) -> Self
    where
        O: Raymarchable + Sized + 'static,
        M: Material + Sized + 'static,
    {
        self.objects.push(ObjectData {
            object: Arc::new(obj),
            material: Arc::new(mat),
            bound: None,
        });

        self
    }
    pub fn add_light<L>(mut self, light: L) -> Self
    where
        L: Light + 'static,
    {
        self.lights.push(LightData::new(Arc::new(light)));

        self
    }
    pub fn background(mut self, colour: Colour) -> Self {
        self.background = colour;
        self
    }

    pub fn unwrap(self) -> Scene {
        Scene {
            background: self.background,
            lights: self.lights,
            objects: self.objects,
        }
    }
}
