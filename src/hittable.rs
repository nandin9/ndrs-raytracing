//! 命中记录和可命中接口模块
//!
//! 提供光线与物体相交的记录结构和抽象接口

use std::rc::Rc;
use super::vec3::{self, Vec3, Point3};
use super::ray::Ray;
use super::interval::Interval;
use super::material::Material;

/// 光线与物体相交的记录
/// 
/// # Fields
/// - p: 命中点位置
/// - normal: 命中点法线向量
/// - mat: 命中物体的材质
/// - t: 光线参数值
/// - front_face: 是否命中物体正面
#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}
/// 可命中物体的抽象接口
/// 
/// 任何可以被光线命中的物体都应实现此trait
pub trait Hittable {
    /// 检查光线是否命中物体
    /// 
    /// # Arguments
    /// * `r` - 入射光线
    /// * `ray_t` - 光线参数有效范围
    /// * `hit_record` - 用于存储命中结果的记录
    /// 
    /// # Returns
    /// 如果光线命中物体返回true，否则返回false
    fn hit(&self, r: &Ray, ray_t: &Interval, hit_record: &mut HitRecord) -> bool;
}

impl HitRecord {
    /// 设置命中点的法线向量方向
    /// 
    /// # Arguments
    /// * `r` - 入射光线
    /// * `outward_normal` - 物体表面外法线(必须已归一化)
    /// 
    /// # Note
    /// 根据光线与法线的点积确定命中面是正面还是背面，
    /// 并相应调整法线方向始终指向光线入射方向
   pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
       // Sets the hit record normal vector.
       // NOTE: the parameter `outward_normal` is assumed to have unit length.

       self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
       self.normal = if self.front_face {
           outward_normal
       } else {
           -outward_normal
       };
   }
}