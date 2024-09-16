use std::ops::{Add, Mul, Sub};

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<Vec<i32>> for Vec3 {
    fn from(vec: Vec<i32>) -> Self {
        if vec.len() != 3 {
            panic!()
        }

        Vec3 {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Mat3 {
    data: [i32; 9],
}

impl Mat3 {
    pub fn rotation_x(theta: i32) -> Mat3 {
        let theta: f32 = deg_to_rad(theta);
        vec![
            vec![1, 0, 0],
            vec![0, theta.cos().round() as i32, -theta.sin().round() as i32],
            vec![0, theta.sin().round() as i32, theta.cos().round() as i32],
        ]
        .into()
    }

    pub fn rotation_y(theta: i32) -> Mat3 {
        let theta: f32 = deg_to_rad(theta);
        vec![
            vec![theta.cos().round() as i32, 0, theta.sin().round() as i32],
            vec![0, 1, 0],
            vec![-theta.sin().round() as i32, 0, theta.cos().round() as i32],
        ]
        .into()
    }

    pub fn rotation_z(theta: i32) -> Mat3 {
        let theta: f32 = deg_to_rad(theta);
        vec![
            vec![theta.cos().round() as i32, -theta.sin().round() as i32, 0],
            vec![theta.sin().round() as i32, theta.cos().round() as i32, 0],
            vec![0, 0, 1],
        ]
        .into()
    }
}

impl From<Vec<Vec<i32>>> for Mat3 {
    fn from(vec: Vec<Vec<i32>>) -> Self {
        if vec.len() != 3 && vec.iter().any(|v| v.len() != 3) {
            panic!()
        }

        let mut data = [0; 9];

        for (i, x) in vec.iter().flatten().enumerate() {
            data[i] = *x;
        }

        Mat3 { data }
    }
}

impl Mul<Vec3> for &Mat3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Self::Output {
        Vec3 {
            x: other.x * self.data[0] + other.y * self.data[1] + other.z * self.data[2],
            y: other.x * self.data[3] + other.y * self.data[4] + other.z * self.data[5],
            z: other.x * self.data[6] + other.y * self.data[7] + other.z * self.data[8],
        }
    }
}

impl Mul<&Mat3> for &Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: &Mat3) -> Self::Output {
        let mut data = [0; 9];

        for row in 0..3 {
            for col in 0..3 {
                data[row * 3 + col] = self.data[(row * 3) + 0] * rhs.data[col]
                    + self.data[(row * 3) + 1] * rhs.data[col + 3]
                    + self.data[(row * 3) + 2] * rhs.data[col + 6];
            }
        }

        Mat3 { data }
    }
}

fn deg_to_rad(theta_deg: i32) -> f32 {
    theta_deg as f32 * std::f32::consts::PI / 180.0
}

#[cfg(test)]
mod tests_space {
    use super::*;

    #[test]
    fn test_vec3() {
        let a: Vec3 = vec![1, 2, 3].into();
        let b: Vec3 = vec![4, 5, 6].into();
        assert_eq!(a + b, vec![5, 7, 9].into());
    }

    #[test]
    fn test_mat3() {
        let a: Mat3 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]].into();

        assert_eq!(a.data[4], 5);
    }

    #[test]
    fn mat_mul() {
        let identity: Mat3 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]].into();
        assert_eq!(&Mat3::rotation_x(90) * &identity, Mat3::rotation_x(90));
    }

    #[test]
    fn test_rot() {
        let identity: Mat3 = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]].into();
        let a: Vec3 = vec![1, 1, 1].into();

        let b = &Mat3::rotation_y(180) * a;

        assert_eq!(&identity * a, a);
        assert_eq!(&Mat3::rotation_x(90) * a, vec![1, -1, 1].into());
        assert_eq!(&Mat3::rotation_x(-90) * a, vec![1, 1, -1].into());
        assert_eq!(&Mat3::rotation_y(90) * a, vec![1, 1, -1].into());
        assert_eq!(&Mat3::rotation_y(-90) * a, vec![-1, 1, 1].into());
        assert_eq!(&Mat3::rotation_z(90) * a, vec![-1, 1, 1].into());
        assert_eq!(&Mat3::rotation_z(-90) * a, vec![1, -1, 1].into());

        assert_eq!(&Mat3::rotation_x(270) * b, vec![-1, -1, -1].into());
        assert_eq!(
            &(&Mat3::rotation_x(270) * &Mat3::rotation_y(180)) * a,
            vec![-1, -1, -1].into()
        );
    }
}
