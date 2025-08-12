//! 实用工具模块
//!
//! 提供数学常量和常用函数

/// 表示正无穷大的常量
pub const INFINITY: f64 = std::f64::INFINITY;

/// 圆周率π的常量
pub const PI: f64 = std::f64::consts::PI;

/// 角度转弧度
/// 
/// # Arguments
/// * `degrees` - 角度值
/// 
/// # Returns
/// 返回对应的弧度值
pub fn degrees_to_radians(degrees: f64) -> f64 {
  degrees * PI / 180.0
}

/// 生成[0,1)范围内的随机浮点数
/// 
/// # Returns
/// 返回0.0(包含)到1.0(不包含)之间的随机数
pub fn random_double() -> f64 {
   // Returns a random real in [0,1).
   rand::random::<f64>()
}

/// 生成指定范围内的随机浮点数
/// 
/// # Arguments
/// * `min` - 范围下限(包含)
/// * `max` - 范围上限(不包含)
/// 
/// # Returns
/// 返回min(包含)到max(不包含)之间的随机数
pub fn random_double_range(min: f64, max: f64) -> f64 {
   // Returns a random real in [min,max).
   min + (max - min) * random_double()
}