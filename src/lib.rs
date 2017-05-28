#![no_std]

mod vec3d;



#[cfg(test)]
mod tests {
    use super::vec3d::{Vec3d};
    #[test]
    fn test_operators() {
        let a = Vec3d{x:1.0, y:2.0, z:3.0};
        let b = Vec3d{x:3.0, y:2.0, z:1.0};

        assert_eq!(a + b, Vec3d{x:4.0, y:4.0, z:4.0});
        assert_eq!(2.0 * a, Vec3d{x:2.0, y:4.0, z:6.0});
        assert_eq!(a * 2.0, Vec3d{x:2.0, y:4.0, z:6.0});
        assert_eq!(a / 2.0, Vec3d{x:0.5, y:1.0, z:1.5});
    }
}
