use crate::utils::{extent::Extent2i, point3i::Point3i};

// 区域 Zone
#[derive(Clone, Copy)]
pub struct Zone {
    // 区域中心
    zone_centor: ZoneCentor,
    // 包含一个二维的范围
    extent2i: Extent2i,
}

pub type ZoneCentor = Point3i;
