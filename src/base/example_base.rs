use std::{iter::Map, time::{Duration, SystemTime}};

use ash::vk;

use super::vulkan_swap_chain::VulkanSwapChain;


struct CommandLineOption {
    commands: Vec<String>,
    vaue: String,
    hasValue: bool, // false,
    help: String, 
    set: bool, // false
}

struct CommandLineParser {
    options: Map<&'static str, CommandLineOption>
}

impl CommandLineParser {

	pub fn add(name: &'static str, commands: Vec<String>, hasValue: bool, help: String) {
        
    }
	pub fn printHelp() {

    }
	pub fn parse(arguments: Vec<&'static str>) {

    }
	pub fn isSet(name: String) {

    }
	pub fn getValueAsString(name: &'static str, defaultValue: &'static str) -> String {

    }
	pub fn getValueAsInt(name: &'static str, defaultValue: i32) -> i32 {

    }
}

struct VulkanSemaphores {
    pub presentComplete: vk::Semaphore,
    pub renderComplete: vk::Semaphore
}

pub struct ExampleBase {
    pub viewUpdated: bool, // false
    destWidth: u32, // 0
    destHeight: u32, // 0
    resizing: bool, // false
    shaderDir: &'static str, // "glsl"
    frameCounter: u32, // 0
    lastFPS: u32, // 0
    lastTimestamp: SystemTime, // SystemTime::now()
    instance: vk::Instance,
    supportedInstanceExtensions: Vec<String>,
    physicalDevice: vk::PhysicalDevice,
    deviceProperties: vk::PhysicalDeviceProperties,
    deviceFeatures: vk::PhysicalDeviceFeatures,
    deviceMemoryProperties: vk::PhysicalDeviceMemoryProperties,
    enabledFeatures: vk::PhysicalDeviceFeatures,
    enabledDeviceExtensions: Vec<&'static str>,
    enabledInstanceExtensions: Vec<&'static str>,
    deviceCreatepNextChain: Option<()>,
    device: vk::Device,
    queue: vk::Queue,
    format: vk::Format,
    cmdPool: vk::CommandPool,
    submitPipelineStages: vk::PipelineStageFlags, // VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT
    submitInfo: vk::SubmitInfo,
    drawCmdBuffers: Vec<vk::CommandBuffer>,
    renderPass: vk::RenderPass,
    frameBuffers: Vec<vk::Framebuffer>,
    currentBuffer: u32,
    descriptorPool: Option<vk::DescriptorPool>,
    shaderModules: Vec<vk::ShaderModule>,
    pipelineCache: vk::PipelineCache,
    swapChain: VulkanSwapChain,
    semaphores: VulkanSemaphores,
    waitFences: Vec<vk::Fence>,
    pub prepared: bool, // false,
    pub resized: bool, // false,
    pub width: u32, // 1280,
    pub height: u32, // 720,
    // pub UIOverlay: imgui?
    pub commandLineParser: CommandLineParser,
    pub frameTimer: f32,
    pub benchmark: 
}


impl ExampleBase {
    fn getWindowTitle() -> &'static str {

    }

    fn windowResize() {

    }
	fn handleMouseMove(x: i32, y: i32)  {

    }
	fn nextFrame() {

    }
	fn updateOverlay() {

    }
	fn createPipelineCache() {

    }
	fn createCommandPool() {

    }
	fn createSynchronizationPrimitives() {

    }
	fn initSwapchain() {

    }
	fn setupSwapChain() {

    }
	fn createCommandBuffers() {

    }
	fn destroyCommandBuffers() {

    }
    fn getShadersPath() {

    }

}