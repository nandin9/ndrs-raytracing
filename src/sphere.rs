//! 球体模块
//!
//! 提供球体几何形状的实现

use super::vec3::{
  self,
  Point3,
};
use std::rc::Rc;
use super::ray::Ray;
use super::material::Material;
use super::hittable::{
  HitRecord,
  Hittable,
};
use super::interval::Interval;

/// 球体几何形状
/// 
/// # Fields
/// - center: 球心位置
/// - radius: 球体半径
/// - mat: 球体材质
pub struct Sphere {
  center: Point3,
  radius: f64,
  mat: Rc<dyn Material>,
}

impl Sphere {
  /// 创建新的球体实例
  /// 
  /// # Arguments
  /// * `center` - 球心位置
  /// * `radius` - 球体半径
  /// * `material` - 球体材质
  pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Self {
    Self {
      center,
      radius,
      mat: material,
    }
  }
}

impl Hittable for Sphere {
    /// 实现球体的光线命中检测
    /// 
    /// 使用二次方程求解光线与球体的交点
    /// 
    /// # Arguments
    /// * `r` - 入射光线
    /// * `ray_t` - 光线参数有效范围
    /// * `hit_record` - 命中记录输出参数
    /// 
    /// # Returns
    /// 如果光线命中球体返回true，否则返回false
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool {
        // 计算球心到光线起点的向量
        let oc = self.center - r.origin();
        
        // 准备二次方程系数（光线方程: P(t)=A+tB，球面方程: |P-C|=r）
        // 方程形式: at² + 2bt + c = 0
        let a = r.direction().squared_length();  // a = B·B
        let b = vec3::dot(r.direction(), oc);    // b = B·(A-C) 
        let c = oc.squared_length() - self.radius * self.radius; // c = (A-C)·(A-C) - r²

        // 计算判别式 Δ = h² - a*c
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return false;  // 无实数解，光线未命中球体
        }
        let sqrtd = discriminant.sqrt();

        // 求解最近的合法交点（在ray_t区间内）
        let mut root = (b - sqrtd) / a;  // 较小的根
        if !ray_t.surrounds(root) {      // 检查是否在有效区间
            root = (b + sqrtd) / a;      // 尝试较大的根
            if !ray_t.surrounds(root) {
                return false;  // 两个根都不在有效区间
            }
        }

        // 填充命中记录
        hit_record.t = root;  // 记录光线参数t
        hit_record.p = r.at(root);  // 计算命中点坐标
        
        // 计算单位法向量（从球心指向命中点）
        let outward_normal = (hit_record.p - self.center) / self.radius;
        // 设置法线方向（根据光线入射方向确定正面/背面）
        hit_record.set_face_normal(r, outward_normal);
        
        // 复制材质引用（使用Rc共享所有权）
        hit_record.mat = Some(Rc::clone(&self.mat));

        true  // 命中成功
    }
}