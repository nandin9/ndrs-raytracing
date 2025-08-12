//! 材质模块
//!
//! 提供材质抽象和具体实现，控制光线与物体的交互方式

use super::ray::Ray;
use super::color::Color;
use super::hittable::HitRecord;
use super::vec3::{self};
use super::rtweekend;

/// 材质抽象接口，定义光线如何与物体表面交互
/// 
/// 所有材质类型都应实现此trait
pub trait Material {
    /// 计算光线的散射行为
    /// 
    /// # Arguments
    /// * `r_in` - 入射光线
    /// * `rec` - 命中记录
    /// * `attenuation` - 出参，存储光线衰减颜色
    /// * `scattered` - 出参，存储散射光线
    /// 
    /// # Returns
    /// 返回是否发生散射
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

/// 漫反射材质(兰伯特材质)
/// 
/// # Fields
/// - albedo: 反射率，决定材质的颜色
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    /// 创建新的漫反射材质
    /// 
    /// # Arguments
    /// * `a` - 反射率颜色
    pub fn new(a: Color) -> Self {
        Self {
            albedo: a,
        }
    }
}

impl Material for Lambertian {
    /// 实现漫反射材质的散射行为
    /// 
    /// 光线在表面随机反射，遵循兰伯特余弦定律
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        // false
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // 捕捉退化的散射方向
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

/// 金属材质，模拟金属表面反射
/// 
/// # Fields
/// - albedo: 金属颜色/反射率
/// 
/// 在光线追踪中，albedo 通常表示：
/// // 表面反射的光能量比例（0~1），可能还带有颜色分量。
/// // 标量 albedo（0~1）：表示表面反射光的能量比例，剩下的能量被吸收。
/// // 向量 albedo（RGB）：不仅表示反射比例，还表示反射颜色，比如 (0.8, 0.8, 0.0) 就是反射 80% 的红光和绿光，不反射蓝光。
pub struct Metal {
  pub albedo: Color,
  pub fuzz: f64,
}

impl Metal {
  /// 创建新的金属材质
  /// 
  /// # Arguments
  /// * `a` - 金属颜色/反射率
  pub fn new(a: Color, f: f64) -> Self {
    Self {
      albedo: a,
      fuzz: if f < 1.0 { f } else { 1.0 },
    }
  }
}

impl Material for Metal {
  /// 实现金属材质的散射行为
  /// 
  /// 光线在表面完美反射(镜面反射)
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    // 计算反射方向：入射光线方向关于法线的镜面反射
    // 1. 先归一化入射光线方向
    // 2. 使用vec3::reflect函数计算反射向量
    let reflected = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
    
    // 创建新的散射光线：
    *scattered = Ray::new(rec.p, reflected + self.fuzz * vec3::random_in_unit_sphere());
    
    // 设置衰减颜色为材质的反射率(albedo)
    // 金属会吸收部分光线能量，用albedo表示反射的颜色和强度
    *attenuation = self.albedo;
    
    // 确保反射光线在半球空间内（点积大于0：夹脚小于90度）
    vec3::dot(scattered.direction(), rec.normal) > 0.0
  }
}

/// 电介质材质（透明物体如玻璃、水等）
pub struct Dielectric {
  pub ir: f64, // 折射指数(Index of Refraction)
}

impl Dielectric {
    /// 创建电介质材质
    /// 
    /// # Arguments
    /// * `index_of_refraction` - 材质折射率(如玻璃为1.5)
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
    
    /// Schlick近似计算菲涅尔反射率
    /// 快速近似计算光线在介质交界处的反射概率
    /// 
    /// # Arguments
    /// * `cosine` - 入射角余弦
    /// * `ref_idx` - 折射率比值
    /// 
    /// # Reference
    /// http://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    #[inline]
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
       // 计算垂直入射时的基础反射率R0
       let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
       let r0 = r0 * r0;
       // 根据入射角混合反射率
       r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
   }
}

impl Material for Dielectric {
  /// 实现电介质材质的散射行为
  /// 同时考虑折射和全反射现象
  fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
    // 电介质不吸收光线（全透射或全反射）
    *attenuation = Color::new(1.0, 1.0, 1.0);
    
    // 根据光线入射面计算折射率比值
    let refraction_ratio = if rec.front_face { 
        1.0 / self.ir  // 从空气进入介质
    } else { 
        self.ir         // 从介质进入空气
    };

    let unit_direction = vec3::unit_vector(r_in.direction());
    let cos_theta = vec3::dot(-unit_direction, rec.normal).min(1.0); // 入射角余弦
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();            // 入射角正弦

    // 检查是否发生全反射（斯涅尔定律不成立）
    let cannot_refract = refraction_ratio * sin_theta > 1.0;
    let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rtweekend::random_double() { 
        vec3::reflect(unit_direction, rec.normal)  // 反射
    } else {
        vec3::refract(unit_direction, rec.normal, refraction_ratio)  // 折射
    };

    *scattered = Ray::new(rec.p, direction);
    true  // 总是发生散射（反射或折射）
  }
}