//! 颜色处理模块
//!
//! 提供颜色类型定义和颜色空间转换功能

use std::io::Write;
use super::interval::Interval;
use super::vec3::Vec3;

/// 颜色类型别名，使用Vec3表示RGB颜色
/// 
/// 三个分量分别对应:
/// - x: 红色分量
/// - y: 绿色分量
/// - z: 蓝色分量
pub type Color = Vec3;

/// 颜色强度范围限制，用于确保颜色值在[0.0, 0.999]范围内
/// 
/// 在转换为8位颜色值时避免超出范围
const INTENSITY: Interval = Interval{ min: 0.0, max: 0.999 };

/// 线性颜色空间到gamma颜色空间的转换
/// 
/// # Arguments
/// * `linear_component` - 线性空间颜色分量值
/// 
/// # Returns
/// 返回gamma校正后的颜色分量值
/// 
/// # Note
/// 对于非正数输入返回0.0
pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

impl Color {
    /// 将颜色值写入输出流(PPM格式)
    /// 
    /// # Arguments
    /// * `out` - 可写的输出流
    /// * `samples_per_pixel` - 每个像素的采样次数，用于颜色值归一化
    /// 
    /// # 处理流程
    /// 1. 根据采样次数归一化颜色值
    /// 2. 应用gamma校正
    /// 3. 将浮点颜色值转换为8位整数
    /// 4. 写入输出流
    /// 
    /// # Returns
    /// 返回io::Result表示写入操作是否成功
    pub fn write_color(&self, out: &mut dyn Write, samples_per_pixel: usize) -> std::io::Result<()> {
        let r = self.x();
        let g = self.y();
        let b = self.z();

        // Divide the color by the number of samples.
        let scale = 1.0 / samples_per_pixel as f64;
        let r = scale * r;
        let g = scale * g;
        let b = scale * b;
        
        // Apply the linear to gamma transform.
        let r = linear_to_gamma(r);
        let g = linear_to_gamma(g);
        let b = linear_to_gamma(b);

        // Write the translated [0,255] value of each color component.
        writeln!(out, "{} {} {}",
            (256.0 * INTENSITY.clamp(r)) as i32,
            (256.0 * INTENSITY.clamp(g)) as i32,
            (256.0 * INTENSITY.clamp(b)) as i32)
    }
}