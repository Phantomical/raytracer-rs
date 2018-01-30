
use vec2::*;
use vec3::*;
use vec4::*;

impl<T: Clone + Sized> Vec2<T> {
	pub fn xx(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.x.clone())
	}
	pub fn xxx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xxyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xy(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.y.clone())
	}
	pub fn xyx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xyxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xyy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xyyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yx(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.x.clone())
	}
	pub fn yxx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yxyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yy(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.y.clone())
	}
	pub fn yyx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yyxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yyy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yyyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
}

impl<T: Clone + Sized> Vec3<T> {
	pub fn xx(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.x.clone())
	}
	pub fn xxx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xxy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xxyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xxyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xxz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xxzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xxzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xxzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xy(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.y.clone())
	}
	pub fn xyx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xyxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xyxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xyy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xyyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xyyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xyz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xyzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xyzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xyzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xz(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.z.clone())
	}
	pub fn xzx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xzxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xzxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xzxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xzy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xzyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xzyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xzyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xzz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xzzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xzzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xzzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yx(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.x.clone())
	}
	pub fn yxx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yxy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yxyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yxyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yxz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yxzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yxzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yxzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yy(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.y.clone())
	}
	pub fn yyx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yyxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yyxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yyy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yyyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yyyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yyz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yyzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yyzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yyzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yz(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.z.clone())
	}
	pub fn yzx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yzxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yzxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yzxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yzy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yzyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yzyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yzyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yzz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yzzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yzzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yzzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zx(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.x.clone())
	}
	pub fn zxx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zxxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zxxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zxxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zxy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zxyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zxyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zxyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zxz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zxzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zxzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zxzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zy(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.y.clone())
	}
	pub fn zyx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zyxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zyxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zyxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zyy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zyyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zyyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zyyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zyz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zyzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zyzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zyzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zz(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.z.clone())
	}
	pub fn zzx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zzxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zzxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zzxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zzy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zzyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zzyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zzyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zzz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zzzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zzzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zzzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
}

impl<T: Clone + Sized> Vec4<T> {
	pub fn xx(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.x.clone())
	}
	pub fn xxx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xxxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xxxw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.x.clone(), self.w.clone())
	}
	pub fn xxy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xxyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xxyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xxyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xxyw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.y.clone(), self.w.clone())
	}
	pub fn xxz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xxzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xxzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xxzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xxzw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.z.clone(), self.w.clone())
	}
	pub fn xxw(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.x.clone(), self.w.clone())
	}
	pub fn xxwx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.w.clone(), self.x.clone())
	}
	pub fn xxwy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.w.clone(), self.y.clone())
	}
	pub fn xxwz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.w.clone(), self.z.clone())
	}
	pub fn xxww(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.x.clone(), self.w.clone(), self.w.clone())
	}
	pub fn xy(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.y.clone())
	}
	pub fn xyx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xyxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xyxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xyxw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.x.clone(), self.w.clone())
	}
	pub fn xyy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xyyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xyyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xyyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xyyw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.y.clone(), self.w.clone())
	}
	pub fn xyz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xyzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xyzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xyzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xyzw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.z.clone(), self.w.clone())
	}
	pub fn xyw(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.y.clone(), self.w.clone())
	}
	pub fn xywx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.w.clone(), self.x.clone())
	}
	pub fn xywy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.w.clone(), self.y.clone())
	}
	pub fn xywz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.w.clone(), self.z.clone())
	}
	pub fn xyww(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.y.clone(), self.w.clone(), self.w.clone())
	}
	pub fn xz(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.z.clone())
	}
	pub fn xzx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xzxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xzxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xzxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xzxw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.x.clone(), self.w.clone())
	}
	pub fn xzy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xzyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xzyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xzyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xzyw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.y.clone(), self.w.clone())
	}
	pub fn xzz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xzzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xzzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xzzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xzzw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.z.clone(), self.w.clone())
	}
	pub fn xzw(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.z.clone(), self.w.clone())
	}
	pub fn xzwx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.w.clone(), self.x.clone())
	}
	pub fn xzwy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.w.clone(), self.y.clone())
	}
	pub fn xzwz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.w.clone(), self.z.clone())
	}
	pub fn xzww(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.z.clone(), self.w.clone(), self.w.clone())
	}
	pub fn xw(&self) -> Vec2<T> {
		vec2(self.x.clone(), self.w.clone())
	}
	pub fn xwx(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.w.clone(), self.x.clone())
	}
	pub fn xwxx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.x.clone(), self.x.clone())
	}
	pub fn xwxy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.x.clone(), self.y.clone())
	}
	pub fn xwxz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.x.clone(), self.z.clone())
	}
	pub fn xwxw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.x.clone(), self.w.clone())
	}
	pub fn xwy(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.w.clone(), self.y.clone())
	}
	pub fn xwyx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.y.clone(), self.x.clone())
	}
	pub fn xwyy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.y.clone(), self.y.clone())
	}
	pub fn xwyz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.y.clone(), self.z.clone())
	}
	pub fn xwyw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.y.clone(), self.w.clone())
	}
	pub fn xwz(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.w.clone(), self.z.clone())
	}
	pub fn xwzx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.z.clone(), self.x.clone())
	}
	pub fn xwzy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.z.clone(), self.y.clone())
	}
	pub fn xwzz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.z.clone(), self.z.clone())
	}
	pub fn xwzw(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.z.clone(), self.w.clone())
	}
	pub fn xww(&self) -> Vec3<T> {
		vec3(self.x.clone(), self.w.clone(), self.w.clone())
	}
	pub fn xwwx(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.w.clone(), self.x.clone())
	}
	pub fn xwwy(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.w.clone(), self.y.clone())
	}
	pub fn xwwz(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.w.clone(), self.z.clone())
	}
	pub fn xwww(&self) -> Vec4<T> {
		vec4(self.x.clone(), self.w.clone(), self.w.clone(), self.w.clone())
	}
	pub fn yx(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.x.clone())
	}
	pub fn yxx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yxxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yxxw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.x.clone(), self.w.clone())
	}
	pub fn yxy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yxyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yxyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yxyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yxyw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.y.clone(), self.w.clone())
	}
	pub fn yxz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yxzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yxzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yxzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yxzw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.z.clone(), self.w.clone())
	}
	pub fn yxw(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.x.clone(), self.w.clone())
	}
	pub fn yxwx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.w.clone(), self.x.clone())
	}
	pub fn yxwy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.w.clone(), self.y.clone())
	}
	pub fn yxwz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.w.clone(), self.z.clone())
	}
	pub fn yxww(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.x.clone(), self.w.clone(), self.w.clone())
	}
	pub fn yy(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.y.clone())
	}
	pub fn yyx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yyxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yyxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yyxw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.x.clone(), self.w.clone())
	}
	pub fn yyy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yyyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yyyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yyyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yyyw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.y.clone(), self.w.clone())
	}
	pub fn yyz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yyzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yyzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yyzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yyzw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.z.clone(), self.w.clone())
	}
	pub fn yyw(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.y.clone(), self.w.clone())
	}
	pub fn yywx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.w.clone(), self.x.clone())
	}
	pub fn yywy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.w.clone(), self.y.clone())
	}
	pub fn yywz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.w.clone(), self.z.clone())
	}
	pub fn yyww(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.y.clone(), self.w.clone(), self.w.clone())
	}
	pub fn yz(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.z.clone())
	}
	pub fn yzx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yzxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn yzxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn yzxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn yzxw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.x.clone(), self.w.clone())
	}
	pub fn yzy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yzyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn yzyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn yzyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn yzyw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.y.clone(), self.w.clone())
	}
	pub fn yzz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yzzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn yzzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn yzzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn yzzw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.z.clone(), self.w.clone())
	}
	pub fn yzw(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.z.clone(), self.w.clone())
	}
	pub fn yzwx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.w.clone(), self.x.clone())
	}
	pub fn yzwy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.w.clone(), self.y.clone())
	}
	pub fn yzwz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.w.clone(), self.z.clone())
	}
	pub fn yzww(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.z.clone(), self.w.clone(), self.w.clone())
	}
	pub fn yw(&self) -> Vec2<T> {
		vec2(self.y.clone(), self.w.clone())
	}
	pub fn ywx(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.w.clone(), self.x.clone())
	}
	pub fn ywxx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.x.clone(), self.x.clone())
	}
	pub fn ywxy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.x.clone(), self.y.clone())
	}
	pub fn ywxz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.x.clone(), self.z.clone())
	}
	pub fn ywxw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.x.clone(), self.w.clone())
	}
	pub fn ywy(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.w.clone(), self.y.clone())
	}
	pub fn ywyx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.y.clone(), self.x.clone())
	}
	pub fn ywyy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.y.clone(), self.y.clone())
	}
	pub fn ywyz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.y.clone(), self.z.clone())
	}
	pub fn ywyw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.y.clone(), self.w.clone())
	}
	pub fn ywz(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.w.clone(), self.z.clone())
	}
	pub fn ywzx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.z.clone(), self.x.clone())
	}
	pub fn ywzy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.z.clone(), self.y.clone())
	}
	pub fn ywzz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.z.clone(), self.z.clone())
	}
	pub fn ywzw(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.z.clone(), self.w.clone())
	}
	pub fn yww(&self) -> Vec3<T> {
		vec3(self.y.clone(), self.w.clone(), self.w.clone())
	}
	pub fn ywwx(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.w.clone(), self.x.clone())
	}
	pub fn ywwy(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.w.clone(), self.y.clone())
	}
	pub fn ywwz(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.w.clone(), self.z.clone())
	}
	pub fn ywww(&self) -> Vec4<T> {
		vec4(self.y.clone(), self.w.clone(), self.w.clone(), self.w.clone())
	}
	pub fn zx(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.x.clone())
	}
	pub fn zxx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zxxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zxxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zxxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zxxw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.x.clone(), self.w.clone())
	}
	pub fn zxy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zxyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zxyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zxyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zxyw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.y.clone(), self.w.clone())
	}
	pub fn zxz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zxzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zxzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zxzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zxzw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.z.clone(), self.w.clone())
	}
	pub fn zxw(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.x.clone(), self.w.clone())
	}
	pub fn zxwx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.w.clone(), self.x.clone())
	}
	pub fn zxwy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.w.clone(), self.y.clone())
	}
	pub fn zxwz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.w.clone(), self.z.clone())
	}
	pub fn zxww(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.x.clone(), self.w.clone(), self.w.clone())
	}
	pub fn zy(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.y.clone())
	}
	pub fn zyx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zyxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zyxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zyxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zyxw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.x.clone(), self.w.clone())
	}
	pub fn zyy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zyyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zyyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zyyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zyyw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.y.clone(), self.w.clone())
	}
	pub fn zyz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zyzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zyzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zyzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zyzw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.z.clone(), self.w.clone())
	}
	pub fn zyw(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.y.clone(), self.w.clone())
	}
	pub fn zywx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.w.clone(), self.x.clone())
	}
	pub fn zywy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.w.clone(), self.y.clone())
	}
	pub fn zywz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.w.clone(), self.z.clone())
	}
	pub fn zyww(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.y.clone(), self.w.clone(), self.w.clone())
	}
	pub fn zz(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.z.clone())
	}
	pub fn zzx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zzxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zzxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zzxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zzxw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.x.clone(), self.w.clone())
	}
	pub fn zzy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zzyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zzyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zzyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zzyw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.y.clone(), self.w.clone())
	}
	pub fn zzz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zzzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zzzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zzzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zzzw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.z.clone(), self.w.clone())
	}
	pub fn zzw(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.z.clone(), self.w.clone())
	}
	pub fn zzwx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.w.clone(), self.x.clone())
	}
	pub fn zzwy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.w.clone(), self.y.clone())
	}
	pub fn zzwz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.w.clone(), self.z.clone())
	}
	pub fn zzww(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.z.clone(), self.w.clone(), self.w.clone())
	}
	pub fn zw(&self) -> Vec2<T> {
		vec2(self.z.clone(), self.w.clone())
	}
	pub fn zwx(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.w.clone(), self.x.clone())
	}
	pub fn zwxx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.x.clone(), self.x.clone())
	}
	pub fn zwxy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.x.clone(), self.y.clone())
	}
	pub fn zwxz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.x.clone(), self.z.clone())
	}
	pub fn zwxw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.x.clone(), self.w.clone())
	}
	pub fn zwy(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.w.clone(), self.y.clone())
	}
	pub fn zwyx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.y.clone(), self.x.clone())
	}
	pub fn zwyy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.y.clone(), self.y.clone())
	}
	pub fn zwyz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.y.clone(), self.z.clone())
	}
	pub fn zwyw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.y.clone(), self.w.clone())
	}
	pub fn zwz(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.w.clone(), self.z.clone())
	}
	pub fn zwzx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.z.clone(), self.x.clone())
	}
	pub fn zwzy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.z.clone(), self.y.clone())
	}
	pub fn zwzz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.z.clone(), self.z.clone())
	}
	pub fn zwzw(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.z.clone(), self.w.clone())
	}
	pub fn zww(&self) -> Vec3<T> {
		vec3(self.z.clone(), self.w.clone(), self.w.clone())
	}
	pub fn zwwx(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.w.clone(), self.x.clone())
	}
	pub fn zwwy(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.w.clone(), self.y.clone())
	}
	pub fn zwwz(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.w.clone(), self.z.clone())
	}
	pub fn zwww(&self) -> Vec4<T> {
		vec4(self.z.clone(), self.w.clone(), self.w.clone(), self.w.clone())
	}
	pub fn wx(&self) -> Vec2<T> {
		vec2(self.w.clone(), self.x.clone())
	}
	pub fn wxx(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.x.clone(), self.x.clone())
	}
	pub fn wxxx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.x.clone(), self.x.clone())
	}
	pub fn wxxy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.x.clone(), self.y.clone())
	}
	pub fn wxxz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.x.clone(), self.z.clone())
	}
	pub fn wxxw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.x.clone(), self.w.clone())
	}
	pub fn wxy(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.x.clone(), self.y.clone())
	}
	pub fn wxyx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.y.clone(), self.x.clone())
	}
	pub fn wxyy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.y.clone(), self.y.clone())
	}
	pub fn wxyz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.y.clone(), self.z.clone())
	}
	pub fn wxyw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.y.clone(), self.w.clone())
	}
	pub fn wxz(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.x.clone(), self.z.clone())
	}
	pub fn wxzx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.z.clone(), self.x.clone())
	}
	pub fn wxzy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.z.clone(), self.y.clone())
	}
	pub fn wxzz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.z.clone(), self.z.clone())
	}
	pub fn wxzw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.z.clone(), self.w.clone())
	}
	pub fn wxw(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.x.clone(), self.w.clone())
	}
	pub fn wxwx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.w.clone(), self.x.clone())
	}
	pub fn wxwy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.w.clone(), self.y.clone())
	}
	pub fn wxwz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.w.clone(), self.z.clone())
	}
	pub fn wxww(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.x.clone(), self.w.clone(), self.w.clone())
	}
	pub fn wy(&self) -> Vec2<T> {
		vec2(self.w.clone(), self.y.clone())
	}
	pub fn wyx(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.y.clone(), self.x.clone())
	}
	pub fn wyxx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.x.clone(), self.x.clone())
	}
	pub fn wyxy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.x.clone(), self.y.clone())
	}
	pub fn wyxz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.x.clone(), self.z.clone())
	}
	pub fn wyxw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.x.clone(), self.w.clone())
	}
	pub fn wyy(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.y.clone(), self.y.clone())
	}
	pub fn wyyx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.y.clone(), self.x.clone())
	}
	pub fn wyyy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.y.clone(), self.y.clone())
	}
	pub fn wyyz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.y.clone(), self.z.clone())
	}
	pub fn wyyw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.y.clone(), self.w.clone())
	}
	pub fn wyz(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.y.clone(), self.z.clone())
	}
	pub fn wyzx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.z.clone(), self.x.clone())
	}
	pub fn wyzy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.z.clone(), self.y.clone())
	}
	pub fn wyzz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.z.clone(), self.z.clone())
	}
	pub fn wyzw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.z.clone(), self.w.clone())
	}
	pub fn wyw(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.y.clone(), self.w.clone())
	}
	pub fn wywx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.w.clone(), self.x.clone())
	}
	pub fn wywy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.w.clone(), self.y.clone())
	}
	pub fn wywz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.w.clone(), self.z.clone())
	}
	pub fn wyww(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.y.clone(), self.w.clone(), self.w.clone())
	}
	pub fn wz(&self) -> Vec2<T> {
		vec2(self.w.clone(), self.z.clone())
	}
	pub fn wzx(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.z.clone(), self.x.clone())
	}
	pub fn wzxx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.x.clone(), self.x.clone())
	}
	pub fn wzxy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.x.clone(), self.y.clone())
	}
	pub fn wzxz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.x.clone(), self.z.clone())
	}
	pub fn wzxw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.x.clone(), self.w.clone())
	}
	pub fn wzy(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.z.clone(), self.y.clone())
	}
	pub fn wzyx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.y.clone(), self.x.clone())
	}
	pub fn wzyy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.y.clone(), self.y.clone())
	}
	pub fn wzyz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.y.clone(), self.z.clone())
	}
	pub fn wzyw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.y.clone(), self.w.clone())
	}
	pub fn wzz(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.z.clone(), self.z.clone())
	}
	pub fn wzzx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.z.clone(), self.x.clone())
	}
	pub fn wzzy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.z.clone(), self.y.clone())
	}
	pub fn wzzz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.z.clone(), self.z.clone())
	}
	pub fn wzzw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.z.clone(), self.w.clone())
	}
	pub fn wzw(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.z.clone(), self.w.clone())
	}
	pub fn wzwx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.w.clone(), self.x.clone())
	}
	pub fn wzwy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.w.clone(), self.y.clone())
	}
	pub fn wzwz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.w.clone(), self.z.clone())
	}
	pub fn wzww(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.z.clone(), self.w.clone(), self.w.clone())
	}
	pub fn ww(&self) -> Vec2<T> {
		vec2(self.w.clone(), self.w.clone())
	}
	pub fn wwx(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.w.clone(), self.x.clone())
	}
	pub fn wwxx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.x.clone(), self.x.clone())
	}
	pub fn wwxy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.x.clone(), self.y.clone())
	}
	pub fn wwxz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.x.clone(), self.z.clone())
	}
	pub fn wwxw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.x.clone(), self.w.clone())
	}
	pub fn wwy(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.w.clone(), self.y.clone())
	}
	pub fn wwyx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.y.clone(), self.x.clone())
	}
	pub fn wwyy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.y.clone(), self.y.clone())
	}
	pub fn wwyz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.y.clone(), self.z.clone())
	}
	pub fn wwyw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.y.clone(), self.w.clone())
	}
	pub fn wwz(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.w.clone(), self.z.clone())
	}
	pub fn wwzx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.z.clone(), self.x.clone())
	}
	pub fn wwzy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.z.clone(), self.y.clone())
	}
	pub fn wwzz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.z.clone(), self.z.clone())
	}
	pub fn wwzw(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.z.clone(), self.w.clone())
	}
	pub fn www(&self) -> Vec3<T> {
		vec3(self.w.clone(), self.w.clone(), self.w.clone())
	}
	pub fn wwwx(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.w.clone(), self.x.clone())
	}
	pub fn wwwy(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.w.clone(), self.y.clone())
	}
	pub fn wwwz(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.w.clone(), self.z.clone())
	}
	pub fn wwww(&self) -> Vec4<T> {
		vec4(self.w.clone(), self.w.clone(), self.w.clone(), self.w.clone())
	}
}
