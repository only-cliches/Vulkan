use ash::vk;


pub struct SwapChainBuffers {
    image: vk::Image,
    view: vk::ImageView,
}

pub struct VulkanSwapChain {
    instance: vk::Instance,
    device: vk::Device,
    physicalDevice: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    fpGetPhysicalDeviceSurfaceSupportKHR: vk::PFN_vkGetPhysicalDeviceSurfaceSupportKHR,
	fpGetPhysicalDeviceSurfaceCapabilitiesKHR: vk::PFN_vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
	fpGetPhysicalDeviceSurfaceFormatsKHR: vk::PFN_vkGetPhysicalDeviceSurfaceFormatsKHR,
	fpGetPhysicalDeviceSurfacePresentModesKHR: vk::PFN_vkGetPhysicalDeviceSurfacePresentModesKHR,
	fpCreateSwapchainKHR: vk::PFN_vkCreateSwapchainKHR,
	fpDestroySwapchainKHR: vk::PFN_vkDestroySwapchainKHR,
	fpGetSwapchainImagesKHR: vk::PFN_vkGetSwapchainImagesKHR,
	fpAcquireNextImageKHR: vk::PFN_vkAcquireNextImageKHR,
	fpQueuePresentKHR: vk::PFN_vkQueuePresentKHR,
    pub colorFormat: vk::Format,
    pub colorSpace: vk::ColorSpaceKHR,
    pub swapChain: Option<vk::SwapchainKHR>,
    pub imageCount: u32,
    pub images: Vec<vk::Image>,
    pub buffers: Vec<SwapChainBuffers>,
    pub queueNodeIndex: u32,
}

impl VulkanSwapChain {

}