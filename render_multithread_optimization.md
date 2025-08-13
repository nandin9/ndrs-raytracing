# 多线程渲染优化

- 原始代码使用了 `Rc` 来管理共享数据，但 `Rc` 不是线程安全的，无法在多线程环境中共享，会导致编译错误
- 将 `Rc` 改为 `Arc`（Atomic Reference Counted），并在需要修改共享数据时配合 `Mutex`，这样可以安全地在多线程中共享和修改数据。
- 将整张图的像素按扫描线或者块切分给多个线程并行渲染
- 每个线程生成自己的像素颜色数据
- 最后主线程收集所有结果，输出

## 1.示例优化后的多线程渲染代码
```rust
use std::sync::{Arc, Mutex};
use crossbeam::thread::scope;

pub fn render_multi_thread(&mut self, world: &dyn Hittable) {
    self.initialize();

    // 提前计算并缓存常用参数（避免重复访问self）
    let width = self.image_width as usize;
    let height = self.image_height as usize;
    let samples_per_pixel = self.samples_per_pixel;
    let max_depth = self.max_depth;

    println!("P3\n{} {}\n255", width, height);

    // Arc<Mutex<...>>实现多线程安全访问
    let pixels = Arc::new(Mutex::new(vec![0u8; width * height * 3]));
    
    // 获取CPU核心数
    let thread_count = num_cpus::get(); 
    // 计算每个线程处理的行数（向上取整）
    let rows_per_thread = height / thread_count + 1; 
    // 获取相机不可变引用（多线程安全）
    let cam_ref = &*self; 

    // 确保所有线程在作用域结束时join）
    scope(|s| {
        for thread_idx in 0..thread_count {
            // 克隆共享指针（增加引用计数）
            let pixels = Arc::clone(&pixels);
            let world = world;
            let cam = cam_ref;

            // 当前线程负责的行范围
            let start_row = thread_idx * rows_per_thread;
            let end_row = ((thread_idx + 1) * rows_per_thread).min(height);

            // 启动工作线程
            s.spawn(move |_| {
                for j in start_row..end_row {
                    for i in 0..width {
                        // 像素颜色累加器
                        let mut pixel_color = Color::default();
                        
                        // 多重采样抗锯齿
                        for _ in 0..samples_per_pixel {
                            let r = cam.get_ray(i as i32, j as i32);
                            pixel_color += Self::ray_color(&r, max_depth, world);
                        }
                        
                        // 计算平均值并进行gamma校正（sqrt近似）
                        let scale = 1.0 / samples_per_pixel as f64;
                        pixel_color *= scale;
                        let ir = (pixel_color.x().sqrt() * 255.999) as u8;
                        let ig = (pixel_color.y().sqrt() * 255.999) as u8;
                        let ib = (pixel_color.z().sqrt() * 255.999) as u8;

                        // 锁定像素缓冲区（最小化临界区）
                        let offset = (j * width + i) * 3;
                        let mut pixels_lock = pixels.lock().unwrap();
                        pixels_lock[offset] = ir;
                        pixels_lock[offset + 1] = ig;
                        pixels_lock[offset + 2] = ib;
                        // MutexGuard离开作用域自动释放锁
                    }
                }
            });
        }
    }).unwrap();

    // 回收像素缓冲区所有权
    let pixels = Arc::try_unwrap(pixels).expect("Arc has other owners");
    let pixels = pixels.into_inner().unwrap();
    
    // 输出渲染结果
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for j in 0..height {
        for i in 0..width {
            let offset = (j * width + i) * 3;
            writeln!(
                out,
                "{} {} {}",
                pixels[offset], pixels[offset + 1], pixels[offset + 2]
            ).unwrap();
        }
    }

    eprintln!("\nDone.");
}
```

## 2.减少加锁
每个线程维护一个自己独立的 局部缓冲区（Vec<u8>），线程内写入这个缓冲区时不需要加锁（因为独占），线程全部计算完毕后，再把各自的局部缓冲区合并（或拷贝）到共享的最终缓冲区中。
```rust
s.spawn(move |_| {
    // 每个线程独立维护一个局部缓冲区
    let mut local_pixels = vec![0u8; (end_row - start_row) * width * 3];

    for (local_j, j) in (start_row..end_row).enumerate() {
        for i in 0..width {
            // ... 计算颜色 ...
            let local_offset = (local_j * width + i) * 3;
            local_pixels[local_offset] = ir;
            local_pixels[local_offset + 1] = ig;
            local_pixels[local_offset + 2] = ib;
        }
    }

    // 计算完成后，合并写入共享缓冲区（只锁一次）
    let mut pixels_lock = pixels.lock().unwrap();
    let global_offset = start_row * width * 3;
    pixels_lock[global_offset..global_offset + local_pixels.len()]
        .copy_from_slice(&local_pixels);
});
```


## 注意事项
- 如果 `Material` 等对象也需要跨线程共享，需要将其从 `Rc<dyn Material>` 改为 `Arc<dyn Material + Send + Sync>`。
- 使用 `Arc<Mutex<>>` 会有一定的锁竞争开销，可以考虑按线程独立缓存计算结果，再合并
