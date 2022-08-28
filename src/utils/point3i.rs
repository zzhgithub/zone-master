#[derive(Clone, Copy)]
pub struct PointN<T> {
    point: T,
}

// 二维的形状 由 (长 , 宽的顺序)
pub type Shap2i = PointN<[i32; 2]>;

/**
 * 三维 体素点（坐标只有整数）
 */
pub type Point3i = PointN<[i32; 3]>;

impl Default for Point3i {
    fn default() -> Self {
        Self {
            point: Default::default(),
        }
    }
}
