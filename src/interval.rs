//! 区间数学模块
//!
//! 提供区间运算功能，用于表示和操作数值范围

use super::rtweekend;

/// 数值区间结构体，表示[min, max]范围内的实数
/// 
/// # Fields
/// - min: 区间下限(包含)
/// - max: 区间上限(包含)
#[derive(Default)]
pub struct Interval {
  pub min: f64,
  pub max: f64,
}

impl Interval {
    /// 创建新的区间实例
    /// 
    /// # Arguments
    /// * `min` - 区间下限
    /// * `max` - 区间上限
    /// 
    /// # Panics
    /// 如果min > max会panic
  pub fn new(min: f64, max: f64) -> Self {
    Self { min, max }
  }

    /// 计算区间大小
    /// 
    /// # Returns
    /// 返回max - min的值
  pub fn size(&self) -> f64 {
    self.max - self.min
  }

    /// 检查值是否在区间内(包含边界)
    /// 
    /// # Arguments
    /// * `x` - 要检查的值
    /// 
    /// # Returns
    /// 如果min ≤ x ≤ max返回true，否则false
  pub fn contains(&self, x: f64) -> bool {
    self.min <= x && x <= self.max
  }

    /// 检查值是否在区间内(不包含边界)
    /// 
    /// # Arguments
    /// * `x` - 要检查的值
    /// 
    /// # Returns
    /// 如果min < x < max返回true，否则false
  pub fn surrounds(&self, x: f64) -> bool {
    self.min < x && x < self.max
  }

    /// 将值限制在区间范围内
    /// 
    /// # Arguments
    /// * `x` - 要限制的值
    /// 
    /// # Returns
    /// 如果x < min返回min，如果x > max返回max，否则返回x
  pub fn clamp(&self, x: f64) -> f64 {
    if x < self.min {
      self.min
    } else if x > self.max {
      self.max
    } else {
      x
    }
  }
}

/// 空区间常量，表示不包含任何值的区间
pub const EMPTY: Interval = Interval {
  min: rtweekend::INFINITY,
  max: -rtweekend::INFINITY,
};
/// 全域区间常量，表示包含所有实数的区间
pub const UNIVERSE: Interval = Interval {
  min: -rtweekend::INFINITY,
  max: rtweekend::INFINITY,
};