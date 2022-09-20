use crate::Color;

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub struct Vec3 {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl Vec3 {
	pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

	pub fn length(&self) -> f64 {
		f64::sqrt(self.squared_length())
	}

	pub fn squared_length(&self) -> f64 {
		self.x * self.x
			+ self.y * self.y
			+ self.z * self.z
	}

	pub fn dot(&self, other: Vec3) -> f64 {
		self.x * other.x
			+ self.y * other.y
			+ self.z * other.z
	}

	pub fn cross(&self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}

	pub fn unit_vector(&self) -> Vec3 {
		*self / self.length()
	}
}

impl std::ops::Add for Vec3 {
	type Output = Vec3;

	fn add(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl std::ops::Sub for Vec3 {
	type Output = Vec3;

	fn sub(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl std::ops::Mul for Vec3 {
	type Output = Vec3;

	fn mul(self, other: Vec3) -> Vec3 {
		Vec3 {
			x: self.x * other.x,
			y: self.y * other.y,
			z: self.z * other.z,
		}
	}
}

impl std::ops::Mul<f64> for Vec3 {
	type Output = Vec3;

	fn mul(self, other: f64) -> Vec3 {
		Vec3 { x: self.x * other, y: self.y * other, z: self.z * other }
	}
}

impl std::ops::Div<f64> for Vec3 {
	type Output = Vec3;

	fn div(self, other: f64) -> Vec3 {
		Vec3 { x: self.x / other, y: self.y / other, z: self.z / other }
	}
}

impl std::ops::Neg for Vec3 {
	type Output = Vec3;

	fn neg(self) -> Vec3 {
		Vec3 { x: -self.x, y: -self.y, z: -self.z }
	}
}

impl std::fmt::Display for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "({} {} {})", self.x, self.y, self.z)
	}
}

impl From<Color> for Vec3 {
	fn from(color: Color) -> Vec3 {
		Vec3 { x: color.r as f64 / 255.999, y: color.g as f64 / 255.999, z: color.b as f64 / 255.999 }
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn add() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
		let v3 = v1 + v2;
		assert_eq!(v3.x, 5.0);
		assert_eq!(v3.y, 7.0);
		assert_eq!(v3.z, 9.0);
	}

	#[test]
	fn sub() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
		let v3 = v1 - v2;
		assert_eq!(v3.x, -3.0);
		assert_eq!(v3.y, -3.0);
		assert_eq!(v3.z, -3.0);
	}

	#[test]
	fn mul() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
		let v3 = v1 * v2;
		assert_eq!(v3.x, 4.0);
		assert_eq!(v3.y, 10.0);
		assert_eq!(v3.z, 18.0);
	}

	#[test]
	fn mul_scalar() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = v1 * 2.0;
		assert_eq!(v2.x, 2.0);
		assert_eq!(v2.y, 4.0);
		assert_eq!(v2.z, 6.0);
	}

	#[test]
	fn div() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = v1 / 2.0;
		assert_eq!(v2.x, 0.5);
		assert_eq!(v2.y, 1.0);
		assert_eq!(v2.z, 1.5);
	}

	#[test]
	fn neg() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = -v1;
		assert_eq!(v2.x, -1.0);
		assert_eq!(v2.y, -2.0);
		assert_eq!(v2.z, -3.0);
	}

	#[test]
	fn length() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		assert_eq!(v1.length(), 3.7416573867739413);
	}

	#[test]
	fn squared_length() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		assert_eq!(v1.squared_length(), 14.0);
	}

	#[test]
	fn dot() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
		assert_eq!(v1.dot(v2), 32.0);
	}

	#[test]
	fn cross() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = Vec3 { x: 4.0, y: 5.0, z: 6.0 };
		let v3 = v1.cross(v2);
		assert_eq!(v3.x, -3.0);
		assert_eq!(v3.y, 6.0);
		assert_eq!(v3.z, -3.0);
	}

	#[test]
	fn unit_vector() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = v1.unit_vector();
		assert_eq!(v2.x, 0.2672612419124244);
		assert_eq!(v2.y, 0.5345224838248488);
		assert_eq!(v2.z, 0.8017837257372732);
	}

	#[test]
	fn unit_vector_length() {
		let v1 = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
		let v2 = v1.unit_vector();
		assert_eq!(v2.length(), 1.0);
	}
}
