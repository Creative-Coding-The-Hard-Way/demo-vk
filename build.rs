use {
    anyhow::{anyhow, Result},
    glob::glob,
    std::{
        io,
        process::{Command, Output},
    },
};

/// Globs all of the shader files in the source tree, based on file extension.
fn get_shader_file_paths(extension: &str) -> Result<Vec<String>> {
    let mut file_names = vec![];
    for file in glob(&format!("src/**/*.{}", extension))? {
        let file_name = file?.to_str().unwrap().to_string();
        println!("cargo::warning=source: {:?}", file_name);
        println!("cargo::rerun-if-changed={}", file_name);
        file_names.push(file_name);
    }
    Ok(file_names)
}

/// Runs the provided compiler command on all shader files and reports any
/// errors.
fn compile_shaders(
    extension: &str,
    compiler_command: impl Fn(&str, &str) -> io::Result<Output>,
) -> Result<()> {
    for shader_source_path in get_shader_file_paths(extension)? {
        let output_path = shader_source_path.replace(extension, "spv");
        let results = compiler_command(&shader_source_path, &output_path)?;
        if !results.status.success() {
            let error_message = String::from_utf8(results.stderr).unwrap();
            eprintln!(
                "Error while compiling slang shader!\n\n{}",
                error_message
            );
            return Err(anyhow!("Error while compiling shader!"));
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    compile_shaders("slang", |source_file, output_file| {
        Command::new("slangc")
            .args(["-o", output_file, "--", source_file])
            .output()
    })?;
    compile_shaders("glsl", |source_file, output_file| {
        Command::new("glslc")
            .args(["-o", output_file, "--target-spv=spv1.6", source_file])
            .output()
    })?;
    Ok(())
}
