
use ash::vk;

struct queueFamilyIndices {
    graphics: u32,
    compute: u32,
    transfer: u32
}

pub struct VulkanDevice {
    physicalDevice: vk::PhysicalDevice,
    logicalDevice: vk::Device,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    enabledFeatures: vk::PhysicalDeviceFeatures,
    memoryProperites: vk::PhysicalDeviceMemoryProperties,
    queueFamilyProperties: Vec<vk::QueueFamilyProperties>,
    supportedExtensions: Vec<String>,
    commandPool: Option<vk::CommandPool>,
    enableDebugMarkers: bool,
    queueFamilyIndices: queueFamilyIndices
}

impl VulkanDevice {
    pub fn new(physicalDevice: vk::PhysicalDevice) -> Self {
        let device = physicalDevice;

        Self {
            physicalDevice: device
        }
    }
}

impl Drop for VulkanDevice {
    fn drop(&mut self) {

    }
}