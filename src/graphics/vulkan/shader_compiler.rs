use {
    crate::{
        graphics::vulkan::{raii, VulkanContext},
        trace,
    },
    anyhow::{bail, Context, Result},
    ash::vk,
    std::path::Path,
};

/// Creates a Vulkan shader module from the provided SPIR-V code.
///
/// SPIR-V is expected to be a valid array of u32 words. If the provided bytes
/// cannot be reinterpreted as words, this method will return an error.
pub fn spirv_module(
    ctx: &VulkanContext,
    shader_bytes: &[u8],
) -> Result<raii::ShaderModule> {
    let words = spirv_words(shader_bytes)?;
    raii::ShaderModule::new(
        ctx.device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: words.len() * 4,
            p_code: words.as_ptr(),
            ..Default::default()
        },
    )
}

/// Convert an unaligned slice of bytes into an aligned chunk of u32 words.
///
/// This is needed because SPIRV is expected to always take the form of 32
/// bytes. It is not always safe to simply reinterpret a slice of u8's due to
/// alignment.
pub fn spirv_words(shader_bytes: &[u8]) -> Result<Vec<u32>> {
    if !shader_bytes.len().is_multiple_of(4) {
        bail!(trace!(
            "Invalid length for compiled SPIRV bytes! {}",
            shader_bytes.len()
        )());
    }
    let shader_words: Vec<u32> = shader_bytes
        .chunks(4)
        .map(|w| u32::from_le_bytes([w[0], w[1], w[2], w[3]]))
        .collect();

    Ok(shader_words)
}

/// Compiles the shader file into usable SPIRV.
///
/// This method invokes `slangc` in a subprocess and therefore expects `slangc`
/// to be present in the system PATH.
///
/// # Params
///
/// * [shader] - The filesystem path to the shader's source.
pub fn compile_slang(
    ctx: &VulkanContext,
    shader: impl AsRef<Path>,
) -> Result<raii::ShaderModule> {
    let shader = shader.as_ref();
    let shader_path_str = shader
        .to_str()
        .with_context(trace!("Unable to decode {:?} as unicode!", shader))?;
    let output = std::process::Command::new("slangc")
        .args([
            "-matrix-layout-column-major", // compatible with nalgebra
            "-target",
            "spirv",
            "--",
            shader_path_str,
        ])
        .output()
        .with_context(trace!("Error executing slangc!"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!(trace!("Error when compiling shader!\n\n{}", stderr)());
    }

    spirv_module(ctx, &output.stdout).with_context(trace!(
        "Error creating shader module for {:?}",
        shader_path_str
    ))
}
