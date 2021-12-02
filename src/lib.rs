pub mod common;

use crate::common::renderer::App;
use mobile_entry_point::mobile_entry_point;
use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
#[cfg(debug_assertions)]
use std::os::raw::c_void;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use std::{collections::HashSet, u64};

use anyhow::Result;
use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{Surface, Swapchain};
use ash::{vk, Device, Entry, Instance};
use ash_window::{create_surface, enumerate_required_extensions};
use cgmath::{Deg, Matrix4, Point3, Vector3};
use crevice::std140::{AsStd140, Std140};
use gpu_allocator::vulkan::*;
use memoffset::offset_of;
#[cfg(debug_assertions)]
use vk::DebugUtilsMessengerEXT;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};


const FPS_240: u128 = 1466;
const FPS_120: u128 = 8333;
const FPS_60: u128 = 16666;
const FPS_30: u128 = 33333; 


#[mobile_entry_point]
fn main() {

    let event_loop = EventLoop::new();
    let mut app = App::new(&event_loop).unwrap();


    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;
        app.egui_integration.handle_event(&event);


        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                app.recreate_swapchain().unwrap()
            },
            Event::MainEventsCleared => app.window.request_redraw(),
            Event::RedrawRequested(_window_id) => {

                match app.frameTime.elapsed() {
                    Ok(elapsed) => {

                        let last_frame_since = elapsed.as_micros();

                        if last_frame_since > FPS_60 {
        
                            app.frameRate = 1_000_000.0f64  / last_frame_since as f64;
                            app.frameTime = SystemTime::now();
                            
                            let start = SystemTime::now();
                            app.draw().unwrap();
                            app.frameTimeUsed = start.elapsed().unwrap().as_micros() as f64;
                            // app.frameTimeUsed = (start.elapsed().unwrap().as_micros() as f64 / FPS_60 as f64) * 100.0f64;
                        }
                    },
                    _ => { }
                }


                
            },
            _ => (),
        }
    })
}