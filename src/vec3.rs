//! 三维向量模块
//!
//! 提供三维向量和点运算的基本实现

use super::rtweekend;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub
};

/// 三维向量结构体
/// 
/// # Fields
/// - e: 包含x,y,z三个分量的数组
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Default for Vec3 {
    /// 创建零向量
    fn default() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.e[i]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        *self *= 1.0 / t;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]] }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]] }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 { e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]] }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self::Output {
        Vec3 { e: [self.e[0] * t, self.e[1] * t, self.e[2] * t] }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Self::Output {
        v * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, t: f64) -> Self::Output {
        (1 as f64 / t) * self
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Vec3 {
    /// 创建新向量
    /// 
    /// # Arguments
    /// * `e0` - x分量
    /// * `e1` - y分量
    /// * `e2` - z分量
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    /// 获取x分量
    pub fn x(&self) -> f64 { self.e[0] }
    /// 获取y分量
    pub fn y(&self) -> f64 { self.e[1] }
    /// 获取z分量
    pub fn z(&self) -> f64 { self.e[2] }

    /// 计算向量长度
    pub fn length(&self) -> f64 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    /// 计算向量长度的平方
    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// 生成随机向量，各分量在[0,1)范围内
    pub fn random() -> Self {
        Self { e: [rtweekend::random_double(), rtweekend::random_double(), rtweekend::random_double()] }
    }

    /// 生成指定范围内的随机向量
    /// 
    /// # Arguments
    /// * `min` - 分量最小值
    /// * `max` - 分量最大值
    pub fn random_range(min: f64, max: f64) -> Self {
        Self { e: [rtweekend::random_double_range(min, max), rtweekend::random_double_range(min, max), rtweekend::random_double_range(min, max)] }
    }

    /// 检查向量是否接近零
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }
}

/// 三维点类型别名
pub type Point3 = Vec3;

/// 计算向量点积
/// 
/// # Arguments
/// * `u` - 第一个向量
/// * `v` - 第二个向量
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

/// 计算向量叉积
/// 
/// # Arguments
/// * `u` - 第一个向量
/// * `v` - 第二个向量
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 { e: [
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    ]}
}

/// 计算单位向量
/// 
/// # Arguments
/// * `v` - 要归一化的向量
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(rtweekend::random_double_range(-1.0, 1.0), rtweekend::random_double_range(-1.0, 1.0), 0.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}   

/// 生成单位球体内的随机向量
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

/// 生成单位球面上的随机向量(已归一化)
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

/// 生成给定法线方向的半球面上的随机向量
/// 
/// # Arguments
/// * `normal` - 定义半球的法线
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_in_unit_sphere();
    if dot(on_unit_sphere, normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

/// 计算向量在表面上的反射向量
/// 
/// # Arguments
/// * `v` - 入射向量
/// * `n` - 表面法线(必须归一化)
pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
   v - 2.0 * dot(v, n) * n
}

/// 计算光线的折射方向（遵循斯涅尔定律）
///
/// # 参数
/// * `uv` - 入射光线单位方向向量
/// * `n` - 表面法线单位向量
/// * `etai_over_etat` - 折射率比值（入射介质折射率/折射介质折射率）
///
/// # 返回值
/// 返回折射后的光线方向向量
///
/// # 原理
/// 根据斯涅尔定律将入射向量分解为垂直和平行分量：
/// 1. 垂直分量按折射率比例缩放
/// 2. 平行分量保持能量守恒
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    // 计算入射角余弦（限制在[0,1]范围避免数值误差）
    let cos_theta = dot(-uv, n).min(1.0);
    
    // 计算折射向量的垂直分量（斯涅尔定律：η₁sinθ₁ = η₂sinθ₂）
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    
    // 计算折射向量的平行分量（确保能量守恒：|r_out|² = 1）
    let r_out_parallel = -(1.0 - r_out_perp.squared_length()).abs().sqrt() * n;
    
    // 返回合成折射方向（垂直分量 + 平行分量）
    r_out_perp + r_out_parallel
}

