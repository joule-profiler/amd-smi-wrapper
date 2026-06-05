use crate::error::AmdSmiError;

pub(crate) type Result<T> = std::result::Result<T, AmdSmiError>;

#[derive(Debug, Clone, Copy)]
pub struct EnergyCount {
    pub energy_accumulator: u64,
    pub counter_resolution: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct GpuUsageInfo {
    pub gpu_usage: u32,
    pub mem_usage: u32,
}
