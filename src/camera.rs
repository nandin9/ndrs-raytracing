//! 相机模块，负责场景渲染和光线追踪
//!
//! 提供Camera结构体用于配置渲染参数和生成光线

use super::rtweekend;
use super::color::Color;
use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::interval::Interval;
use super::vec3::{self, Point3, Vec3};

/// 相机结构体，包含渲染场景所需的所有参数
/// 
/// # Fields
/// - aspect_ratio: 图像宽高比
/// - image_width: 图像宽度(像素)
/// - samples_per_pixel: 每个像素的采样次数
/// - max_depth: 光线最大反弹次数
pub struct Camera {
    pub aspect_ratio: f64,  // 图像宽高比（宽度/高度）
    pub image_width: i32,   // 渲染图像宽度（像素数）
    pub samples_per_pixel: usize,  // 每个像素的采样次数
    pub max_depth: i32,     // 光线最大反弹次数
    pub vfov: f64,          // 垂直视野角度
    pub lookfrom: Point3,   // 相机位置原点
    pub lookat: Point3,     // 相机瞄准点
    pub vup: Vec3,          // 相机上方向向量
    pub defocus_angle: f64, // 散景模糊角度
    pub focus_dist: f64,    // 对焦距离
    image_height: i32,      // 渲染图像高度
    center: Point3,         // 相机中心位置
    pixel00_loc: Point3,    // 像素(0,0)的位置
    pixel_delta_u: Vec3,    // 向右相邻像素的偏移量
    pixel_delta_v: Vec3,    // 向下相邻像素的偏移量
    u: Vec3,                // 相机水平轴
    v: Vec3,                // 相机垂直轴
    w: Vec3,                // 相机前向轴
    defocus_disk_u: Vec3,   // 散景圆盘水平轴
    defocus_disk_v: Vec3,   // 散景圆盘垂直轴
}

impl Default for Camera {
    /// 创建默认相机配置
    fn default() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            samples_per_pixel: 4,
            max_depth: 10,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }
}

impl Camera {
    /// 渲染场景到标准输出(PPM格式)
    /// 
    /// # Arguments
    /// * `world` - 包含要渲染物体的Hittable对象
    /// 
    /// # 处理流程
    /// 1. 初始化相机参数
    /// 2. 逐像素计算颜色值
    /// 3. 输出PPM格式图像数据
    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        let stdout = std::io::stdout();

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    // pixel_color += self.ray_color(&r, world);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color.write_color(&mut stdout.lock(), self.samples_per_pixel).unwrap();
            }
        }

        eprintln!("\nDone.");
    }

    /// 初始化相机参数
    /// 
    /// 根据当前配置计算:
    /// - 图像高度
    /// - 视口大小和位置
    /// - 像素增量向量
    /// - 初始像素位置
    fn initialize(&mut self) {
        // self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        // self.center = Point3::default();
        self.center = self.lookfrom;

        // 确认视口的大小。
        // let focal_length = 1.0;
        // let focal_length = (self.lookfrom - self.lookat).length();

        let theta = rtweekend::degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        // let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // 计算相机坐标系的 u,v,w 单位基向量。
        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(vec3::cross(self.vup, self.w));
        self.v = vec3::cross(self.w, self.u);

        // 计算水平和垂直视口边缘上的向量。
        let viewport_u = self.u * viewport_width;
        let viewport_v = -self.v * viewport_height;

        // 计算水平和垂直视口边缘上的向量。
        // let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        // let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // 计算从像素到像素的水平和垂直增量向量。
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // 计算左上角像素的位置。
        let viewport_upper_left = self.center
            - (self.focus_dist * self.w)
            - (0.5 * viewport_u)
            - (0.5 * viewport_v);
        // let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // 计算相机失焦盘的基向量
        let defocus_radius = self.focus_dist * (rtweekend::degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    /// 计算给定光线的颜色
    /// 
    /// # Arguments
    /// * `r` - 要计算颜色的光线
    /// * `depth` - 剩余光线反弹次数
    /// * `world` - 包含物体的Hittable对象
    /// 
    /// # Returns
    /// 返回计算得到的颜色值，考虑光线反弹和材质散射
    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        let mut rec = HitRecord::default();  // 创建命中记录

        // 如果达到光线反弹次数限制，停止收集光线
        if depth <= 0 {
            return Color::default();  // 返回黑色(无光)
        }
        
        // 检查光线是否命中场景中的物体
        if world.hit(r, &Interval::new(0.001, rtweekend::INFINITY), &mut rec) {
            let mut scattered = Ray::default();  // 散射光线
            let mut attenuation = Color::default();  // 衰减颜色
            
            // 如果物体有材质
            if let Some(mat) = rec.mat.clone() {
                // 计算材质散射
                if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                    // 递归计算散射光线的颜色
                    return attenuation * Self::ray_color(&scattered, depth - 1, world);
                }
            }
            return Color::default();  // 无散射则返回黑色
        }

        // 计算天空背景颜色(渐变色)
        let unit_direction = vec3::unit_vector(r.direction());  // 归一化光线方向
        let a = 0.5 * (unit_direction.y() + 1.0);  // 计算垂直方向的混合系数
        // 混合白色和天蓝色，模拟天空效果
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    /// 生成通过像素(i,j)的光线
    /// 
    /// # Arguments
    /// * `i` - 像素列索引
    /// * `j` - 像素行索引
    /// 
    /// # Returns
    /// 返回从相机中心指向像素(i,j)的光线
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center = self.pixel00_loc + i as f64 * self.pixel_delta_u + j as f64 * self.pixel_delta_v;
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    /// 在像素区域内生成随机采样点
    /// 
    /// # Returns
    /// 返回像素区域内的随机偏移向量
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + rtweekend::random_double();
        let py = -0.5 + rtweekend::random_double();
        px * self.pixel_delta_u + py * self.pixel_delta_v
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = vec3::random_in_unit_disk();
        self.center + p.x() * self.defocus_disk_u + p.y() * self.defocus_disk_v
    }
}