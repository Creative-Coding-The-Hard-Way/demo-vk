use {
    anyhow::{Context, Result, bail},
    std::path::Path,
};

/// Compiles the shader file into usable SPIRV.
///
/// This method invokes `slangc` in a subprocess and therefore expects `slangc`
/// to be present in the system PATH.
///
/// # Params
///
/// * [shader] - The filesystem path to the shader's source.
/// * [output_file] - Optionally write the comiled shader to a file, rather than
///   returning the bytes directly.
///
/// # Returns
///
/// The compiled shader artifact as spirv words.
pub fn compile_slang(
    shader: impl AsRef<Path>,
    output_file: Option<impl AsRef<Path>>,
) -> Result<Vec<u8>> {
    let shader = shader.as_ref();
    let shader_path_str = shader.to_str().with_context(|| {
        format!("Unable to decode {:?} as unicode!", shader)
    })?;
    let mut args = if let Some(ref output_file) = output_file {
        let output_file = output_file.as_ref();
        let output_path_str = output_file.to_str().with_context(|| {
            format!("Unable to decode {:?} as unicode!", output_file)
        })?;
        vec!["-o", output_path_str]
    } else {
        vec![]
    };
    args.extend_from_slice(&[
        "-matrix-layout-column-major", // compatible with nalgebra
        "-target",
        "spirv",
        "--",
        shader_path_str,
    ]);
    let output = std::process::Command::new("slangc")
        .args(args)
        .output()
        .with_context(|| "Error executing slangc!")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Error compiling shader!\n\n{}", stderr);
    }

    Ok(output.stdout)
}

pub fn compile_glsl(
    shader: impl AsRef<Path>,
    output_file: Option<impl AsRef<Path>>,
) -> Result<Vec<u8>> {
    let shader = shader.as_ref();
    let shader_path_str = shader.to_str().with_context(|| {
        format!("Unable to decode {:?} as unicode!", shader)
    })?;
    let mut args = if let Some(ref output_file) = output_file {
        let output_file = output_file.as_ref();
        let output_path_str = output_file.to_str().with_context(|| {
            format!("Unable to decode {:?} as unicode!", output_file)
        })?;
        vec!["-o", output_path_str]
    } else {
        vec!["-o", "-"]
    };
    args.extend_from_slice(&["--target-spv=spv1.5", shader_path_str]);
    let output = std::process::Command::new("glslc")
        .args(args)
        .output()
        .with_context(|| "Error executing slangc!")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Error compiling shader!\n\n{}", stderr);
    }

    Ok(output.stdout)
}
