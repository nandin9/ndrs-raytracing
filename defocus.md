# 简易版 Ray Tracing 景深模糊（Defocus Blur）

## 背景介绍

景深模糊（Defocus Blur）模拟的是现实相机中由于焦距和光圈大小导致的前景或背景模糊效果。  
在光线追踪中，这通过让相机发射的光线起点不再固定于一个点，而是随机分布在一个光圈区域来实现。

---

## 实现原理

1. **光圈 (Aperture)**：相机镜头开口大小，决定模糊强度。  
2. **焦距 (Focus Distance)**：相机对焦的距离。  
3. **随机采样光圈**：射出光线时，光线的起点从一个圆盘（光圈）上随机采样，模拟镜头的物理光圈。

---

## 关键代码示例（Rust风格）

```rust
use rand::Rng;

struct Vec3 { x: f64, y: f64, z: f64 } // 伪代码示意

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, v: Vec3, w: Vec3,  // 相机坐标系基向量
    lens_radius: f64,
    focus_dist: f64,
}

impl Camera {
    // 在光圈圆盘上随机采样一点
    fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    fn get_ray(&self, s: f64, t: f64) -> Ray {
        // 在光圈上随机偏移
        let rd = self.lens_radius * Self::random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset,
        }
    }
}
```

## 说明
- lens_radius 控制光圈大小，越大模糊越明显。
- random_in_unit_disk() 生成光圈上的随机点，模拟光线从不同位置射出。
- 通过改变光线起点（origin + offset），模仿了相机镜头的景深效果。
- 对场景多次采样并平均，产生模糊的焦外效果。

