//! 光线模块
//!
//! 提供光线数据结构及相关运算

use crate::vec3::{Point3, Vec3};

/// 光线结构体，表示从原点沿方向传播的光线
/// 
/// # Fields
/// - orig: 光线起点
/// - dir: 光线传播方向(已归一化)
#[derive(Clone, Copy, Debug, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    /// 创建新的光线
    /// 
    /// # Arguments
    /// * `origin` - 光线起点
    /// * `direction` - 光线方向(应已归一化)
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    /// 获取光线起点
    pub fn origin(&self) -> Point3 {
        self.orig
    }

    /// 获取光线方向
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// 计算光线在参数t处的位置
    /// 
    /// # Arguments
    /// * `t` - 光线参数
    /// 
    /// # Returns
    /// 返回光线上的点: origin + t * direction
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
