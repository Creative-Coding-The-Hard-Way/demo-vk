//! This test verifies that allocating memory with the device buffer address
//! feature works as expected.

use {
    anyhow::Result,
    ash::vk,
    clap::Parser,
    demo_vk::{
        demo::{demo_main, Demo, Graphics},
        graphics::vulkan::{CPUBuffer, RequiredDeviceFeatures, UniformBuffer},
    },
    glfw::Window,
};

#[derive(Debug, Parser)]
struct Args {}

struct DeviceBufferAddressTest;

impl Demo for DeviceBufferAddressTest {
    type Args = Args;

    fn required_device_features() -> RequiredDeviceFeatures {
        RequiredDeviceFeatures {
            physical_device_vulkan12_features:
                vk::PhysicalDeviceVulkan12Features {
                    buffer_device_address: vk::TRUE,
                    ..Default::default()
                },
            ..Default::default()
        }
    }

    fn new(window: &mut Window, gfx: &mut Graphics<Args>) -> Result<Self> {
        let _ubuf = UniformBuffer::<f32>::allocate(&gfx.vulkan, 1)?;

        // This is the heart of the test. Allocate a buffer and fetch the device
        // address. Validation layers will report errors if the
        // underlying memory or device features are incorrectly managed.
        let buffer = CPUBuffer::<f32>::allocate(
            &gfx.vulkan,
            1,
            vk::BufferUsageFlags::INDEX_BUFFER
                | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        )?;
        let address = unsafe {
            gfx.vulkan
                .get_buffer_device_address(&vk::BufferDeviceAddressInfo {
                    buffer: buffer.buffer(),
                    ..Default::default()
                })
        };
        log::info!("Created buffer with address: {}", address);

        window.set_should_close(true);
        Ok(Self {})
    }
}

#[test]
fn no_validation_errors_should_be_raised() {
    let _ = demo_main::<DeviceBufferAddressTest>();
}
