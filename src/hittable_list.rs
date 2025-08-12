//! 可命中物体列表模块
//!
//! 提供HittableList结构体，用于管理多个可命中物体的集合

use std::rc::Rc;

use super::hittable::{
    HitRecord,
    Hittable,
};
use super::ray::Ray;
use super::interval::Interval;

/// 可命中物体列表，包含多个实现Hittable trait的对象
/// 
/// # Fields
/// - objects: 可命中物体集合，使用引用计数智能指针管理
#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    /// 创建包含单个物体的HittableList
    /// 
    /// # Arguments
    /// * `object` - 要添加的初始物体
    /// 
    /// # Returns
    /// 返回包含指定物体的新HittableList实例
    pub fn new(object: Rc<dyn Hittable>) -> Self {
        Self {
            objects: vec![object],
        }
    }

    /// 清空物体列表
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// 向列表中添加新物体
    /// 
    /// # Arguments
    /// * `object` - 要添加的物体
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    // pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
    //     let mut temp_rec = HitRecord::default();
    //     let mut hit_anything = false;
    //     let mut closest_so_far = ray_tmax;

    //     for object in self.objects.iter() {
    //         if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
    //             hit_anything = true;
    //             closest_so_far = temp_rec.t;
    //             *rec = temp_rec.clone();
    //         }
    //     }

    //     hit_anything
    // }
}

impl Hittable for HittableList {
    /// 检查光线是否命中列表中的任何物体
    /// 
    /// # Arguments
    /// * `r` - 入射光线
    /// * `ray_t` - 光线参数有效范围
    /// * `rec` - 用于存储命中结果的记录
    /// 
    /// # Returns
    /// 如果光线命中任何物体返回true，否则返回false
    /// 
    /// # Note
    /// 只记录最近的命中结果
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}