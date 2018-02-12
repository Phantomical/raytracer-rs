
use rhai::Engine;
use lib::{Vec3d, Vec2d, vec3, vec2};

pub fn register_swizzle_vec3d_fns(engine: &mut Engine) {

	fn vec3_get_xx(v: &mut Vec3d) -> Vec2d { vec2(v.x, v.x) }
	engine.register_get("xx", vec3_get_xx);

	fn vec3_get_xxx(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.x, v.x) }
	engine.register_get("xxx", vec3_get_xxx);

	fn vec3_get_xxy(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.x, v.y) }
	engine.register_get("xxy", vec3_get_xxy);

	fn vec3_get_xxz(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.x, v.z) }
	engine.register_get("xxz", vec3_get_xxz);

	fn vec3_get_xy(v: &mut Vec3d) -> Vec2d { vec2(v.x, v.y) }
	fn vec3_set_xy(v: &mut Vec3d, s: Vec2d) { v.x = s.x; v.y = s.y; }
	engine.register_get_set("xy", vec3_get_xy, vec3_set_xy);

	fn vec3_get_xyx(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.y, v.x) }
	engine.register_get("xyx", vec3_get_xyx);

	fn vec3_get_xyy(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.y, v.y) }
	engine.register_get("xyy", vec3_get_xyy);

	fn vec3_get_xyz(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.y, v.z) }
	fn vec3_set_xyz(v: &mut Vec3d, s: Vec3d) { v.x = s.x; v.y = s.y; v.z = s.z; }
	engine.register_get_set("xyz", vec3_get_xyz, vec3_set_xyz);

	fn vec3_get_xz(v: &mut Vec3d) -> Vec2d { vec2(v.x, v.z) }
	fn vec3_set_xz(v: &mut Vec3d, s: Vec2d) { v.x = s.x; v.z = s.y; }
	engine.register_get_set("xz", vec3_get_xz, vec3_set_xz);

	fn vec3_get_xzx(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.z, v.x) }
	engine.register_get("xzx", vec3_get_xzx);

	fn vec3_get_xzy(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.z, v.y) }
	fn vec3_set_xzy(v: &mut Vec3d, s: Vec3d) { v.x = s.x; v.z = s.y; v.y = s.z; }
	engine.register_get_set("xzy", vec3_get_xzy, vec3_set_xzy);

	fn vec3_get_xzz(v: &mut Vec3d) -> Vec3d { vec3(v.x, v.z, v.z) }
	engine.register_get("xzz", vec3_get_xzz);

	fn vec3_get_yx(v: &mut Vec3d) -> Vec2d { vec2(v.y, v.x) }
	fn vec3_set_yx(v: &mut Vec3d, s: Vec2d) { v.y = s.x; v.x = s.y; }
	engine.register_get_set("yx", vec3_get_yx, vec3_set_yx);

	fn vec3_get_yxx(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.x, v.x) }
	engine.register_get("yxx", vec3_get_yxx);

	fn vec3_get_yxy(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.x, v.y) }
	engine.register_get("yxy", vec3_get_yxy);

	fn vec3_get_yxz(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.x, v.z) }
	fn vec3_set_yxz(v: &mut Vec3d, s: Vec3d) { v.y = s.x; v.x = s.y; v.z = s.z; }
	engine.register_get_set("yxz", vec3_get_yxz, vec3_set_yxz);

	fn vec3_get_yy(v: &mut Vec3d) -> Vec2d { vec2(v.y, v.y) }
	engine.register_get("yy", vec3_get_yy);

	fn vec3_get_yyx(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.y, v.x) }
	engine.register_get("yyx", vec3_get_yyx);

	fn vec3_get_yyy(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.y, v.y) }
	engine.register_get("yyy", vec3_get_yyy);

	fn vec3_get_yyz(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.y, v.z) }
	engine.register_get("yyz", vec3_get_yyz);

	fn vec3_get_yz(v: &mut Vec3d) -> Vec2d { vec2(v.y, v.z) }
	fn vec3_set_yz(v: &mut Vec3d, s: Vec2d) { v.y = s.x; v.z = s.y; }
	engine.register_get_set("yz", vec3_get_yz, vec3_set_yz);

	fn vec3_get_yzx(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.z, v.x) }
	fn vec3_set_yzx(v: &mut Vec3d, s: Vec3d) { v.y = s.x; v.z = s.y; v.x = s.z; }
	engine.register_get_set("yzx", vec3_get_yzx, vec3_set_yzx);

	fn vec3_get_yzy(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.z, v.y) }
	engine.register_get("yzy", vec3_get_yzy);

	fn vec3_get_yzz(v: &mut Vec3d) -> Vec3d { vec3(v.y, v.z, v.z) }
	engine.register_get("yzz", vec3_get_yzz);

	fn vec3_get_zx(v: &mut Vec3d) -> Vec2d { vec2(v.z, v.x) }
	fn vec3_set_zx(v: &mut Vec3d, s: Vec2d) { v.z = s.x; v.x = s.y; }
	engine.register_get_set("zx", vec3_get_zx, vec3_set_zx);

	fn vec3_get_zxx(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.x, v.x) }
	engine.register_get("zxx", vec3_get_zxx);

	fn vec3_get_zxy(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.x, v.y) }
	fn vec3_set_zxy(v: &mut Vec3d, s: Vec3d) { v.z = s.x; v.x = s.y; v.y = s.z; }
	engine.register_get_set("zxy", vec3_get_zxy, vec3_set_zxy);

	fn vec3_get_zxz(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.x, v.z) }
	engine.register_get("zxz", vec3_get_zxz);

	fn vec3_get_zy(v: &mut Vec3d) -> Vec2d { vec2(v.z, v.y) }
	fn vec3_set_zy(v: &mut Vec3d, s: Vec2d) { v.z = s.x; v.y = s.y; }
	engine.register_get_set("zy", vec3_get_zy, vec3_set_zy);

	fn vec3_get_zyx(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.y, v.x) }
	fn vec3_set_zyx(v: &mut Vec3d, s: Vec3d) { v.z = s.x; v.y = s.y; v.x = s.z; }
	engine.register_get_set("zyx", vec3_get_zyx, vec3_set_zyx);

	fn vec3_get_zyy(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.y, v.y) }
	engine.register_get("zyy", vec3_get_zyy);

	fn vec3_get_zyz(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.y, v.z) }
	engine.register_get("zyz", vec3_get_zyz);

	fn vec3_get_zz(v: &mut Vec3d) -> Vec2d { vec2(v.z, v.z) }
	engine.register_get("zz", vec3_get_zz);

	fn vec3_get_zzx(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.z, v.x) }
	engine.register_get("zzx", vec3_get_zzx);

	fn vec3_get_zzy(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.z, v.y) }
	engine.register_get("zzy", vec3_get_zzy);

	fn vec3_get_zzz(v: &mut Vec3d) -> Vec3d { vec3(v.z, v.z, v.z) }
	engine.register_get("zzz", vec3_get_zzz);
}
pub fn register_swizzle_vec2d_fns(engine: &mut Engine) {

	fn vec2_get_xx(v: &mut Vec2d) -> Vec2d { vec2(v.x, v.x) }
	engine.register_get("xx", vec2_get_xx);

	fn vec2_get_xxx(v: &mut Vec2d) -> Vec3d { vec3(v.x, v.x, v.x) }
	engine.register_get("xxx", vec2_get_xxx);

	fn vec2_get_xxy(v: &mut Vec2d) -> Vec3d { vec3(v.x, v.x, v.y) }
	engine.register_get("xxy", vec2_get_xxy);

	fn vec2_get_xy(v: &mut Vec2d) -> Vec2d { vec2(v.x, v.y) }
	fn vec2_set_xy(v: &mut Vec2d, s: Vec2d) { v.x = s.x; v.y = s.y; }
	engine.register_get_set("xy", vec2_get_xy, vec2_set_xy);

	fn vec2_get_xyx(v: &mut Vec2d) -> Vec3d { vec3(v.x, v.y, v.x) }
	engine.register_get("xyx", vec2_get_xyx);

	fn vec2_get_xyy(v: &mut Vec2d) -> Vec3d { vec3(v.x, v.y, v.y) }
	engine.register_get("xyy", vec2_get_xyy);

	fn vec2_get_yx(v: &mut Vec2d) -> Vec2d { vec2(v.y, v.x) }
	fn vec2_set_yx(v: &mut Vec2d, s: Vec2d) { v.y = s.x; v.x = s.y; }
	engine.register_get_set("yx", vec2_get_yx, vec2_set_yx);

	fn vec2_get_yxx(v: &mut Vec2d) -> Vec3d { vec3(v.y, v.x, v.x) }
	engine.register_get("yxx", vec2_get_yxx);

	fn vec2_get_yxy(v: &mut Vec2d) -> Vec3d { vec3(v.y, v.x, v.y) }
	engine.register_get("yxy", vec2_get_yxy);

	fn vec2_get_yy(v: &mut Vec2d) -> Vec2d { vec2(v.y, v.y) }
	engine.register_get("yy", vec2_get_yy);

	fn vec2_get_yyx(v: &mut Vec2d) -> Vec3d { vec3(v.y, v.y, v.x) }
	engine.register_get("yyx", vec2_get_yyx);

	fn vec2_get_yyy(v: &mut Vec2d) -> Vec3d { vec3(v.y, v.y, v.y) }
	engine.register_get("yyy", vec2_get_yyy);
}
