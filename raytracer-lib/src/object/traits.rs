use vec::*;
use ray::*;
use std::iter::Iterator;
use cacheable::Cacheable;
use std::rc::Rc;

use erased_serde::Serialize;

fn yxx(v: Vec2d) -> Vec3d {
    return v.yxx();
}
fn xyx(v: Vec2d) -> Vec3d {
    return v.xyx();
}
fn xxy(v: Vec2d) -> Vec3d {
    return v.xxy();
}

pub fn normal_finite_difference<T>(me: &T, point: Vec3d) -> Vec3d
where
    T: ?Sized + Raymarchable,
{
    let eps = vec2(0.0, me.epsilon(point));
    // Normal approximation using gradient method

    return vec3(
        me.distance(point + yxx(eps)) - me.distance(point - yxx(eps)),
        me.distance(point + xyx(eps)) - me.distance(point - xyx(eps)),
        me.distance(point + xxy(eps)) - me.distance(point - xxy(eps)),
    ).normalize();
}

pub trait Raymarchable: Serialize + Sync + Send {
    fn epsilon(&self, _point: Vec3d) -> f64 {
        0.000001
    }

    fn normal_at(&self, point: Vec3d, _direction: Vec3d) -> Vec3d {
        normal_finite_difference(self, point)
    }

    fn distance(&self, point: Vec3d) -> f64;
}

pub trait Analytical: Raymarchable {
    // Indicates that the object can find intersections analytically
    fn analytical(&self) -> bool {
        return false;
    }
    // Finds the intersection analytically
    fn intersect(&self, _ray: &Ray) -> Option<Vec3d> {
        unimplemented!();
    }
}

pub trait IFS: Raymarchable {
    fn points(&self, point: Vec3d) -> Box<Iterator<Item = Vec3d>>;
}

impl<T> Cacheable<Rc<Raymarchable>> for T
where
    T: Cacheable<T> + Raymarchable + 'static,
{
    fn cached(&self) -> Rc<Raymarchable> {
        Rc::new(self.cached())
    }
}
impl<T> Cacheable<Rc<Analytical>> for T
where
    T: Cacheable<T> + Analytical + 'static,
{
    fn cached(&self) -> Rc<Analytical> {
        Rc::new(self.cached())
    }
}
impl<T> Cacheable<Rc<IFS>> for T
where
    T: Cacheable<T> + IFS + 'static,
{
    fn cached(&self) -> Rc<IFS> {
        Rc::new(self.cached())
    }
}

serialize_trait_object!(Raymarchable);
