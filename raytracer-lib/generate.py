
vars = ['x', 'y']
ty = "Vec3d"
ret = ['f64', 'Vec2d, Vec3d']

print("""
use rhai::Engine;
use lib::{Vec3d, Vec2d, vec3, vec2};

pub fn register_swizzle_fns(engine: &mut Engine) {""")

for x in vars:
    for y in vars:  
        if x != y:
            text = """
\tfn vec2_get_{0}{1}(v: &mut Vec2d) -> Vec2d {{ vec2(v.{0}, v.{1}) }}
\tfn vec2_set_{0}{1}(v: &mut Vec2d, s: Vec2d) {{ v.{0} = s.x; v.{1} = s.y; }}
\tengine.register_get_set("{0}{1}", vec2_get_{0}{1}, vec2_set_{0}{1});""".format(x, y)
            print(text)

            for z in vars:
                text = """
\tfn vec2_get_{0}{1}{2}(v: &mut Vec2d) -> Vec3d {{ vec3(v.{0}, v.{1}, v.{2}) }}
\tengine.register_get("{0}{1}{2}", vec2_get_{0}{1}{2});""".format(x, y, z)
                print(text)
        else:
            text = """
\tfn vec2_get_{0}{1}(v: &mut Vec2d) -> Vec2d {{ vec2(v.{0}, v.{1}) }}
\tengine.register_get("{0}{1}", vec2_get_{0}{1});""".format(x, y)
            print(text)

            for z in vars:
                text = """
\tfn vec2_get_{0}{1}{2}(v: &mut Vec2d) -> Vec3d {{ vec3(v.{0}, v.{1}, v.{2}) }}
\tengine.register_get("{0}{1}{2}", vec2_get_{0}{1}{2});""".format(x, y, z)
                print(text)

print("}\n")