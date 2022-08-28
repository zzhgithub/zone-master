/**
 * 区域范围类型
 */
#[derive(Clone, Copy)]
pub struct Extent<T> {
    extent: T,
}

impl Extent2i {}

/**
 * 二维区域范围
 * 包含一个最小的 和最大点
 */
pub type Extent2i = Extent<[i32; 2]>;

/**
 * 三维区域范围
 */
pub type Extent3i = Extent<[i32; 3]>;
