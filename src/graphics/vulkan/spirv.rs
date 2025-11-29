use {
    crate::{
        graphics::vulkan::{raii, VulkanContext},
        trace,
    },
    anyhow::{bail, Result},
    ash::vk,
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
        "ShaderCompiler SPIR-V Module",
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
