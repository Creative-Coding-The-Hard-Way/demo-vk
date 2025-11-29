use {
    anyhow::{anyhow, Result},
    glob::glob,
    shader_compiler::compile_slang,
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
    compiler_command: impl Fn(&str, &str) -> Result<Vec<u8>>,
) -> Result<()> {
    for shader_source_path in get_shader_file_paths(extension)? {
        let output_path = shader_source_path.replace(extension, "spv");
        if let Err(error) = compiler_command(&shader_source_path, &output_path)
        {
            eprintln!("Error while compiling slang shader!\n\n{}", error);
            return Err(anyhow!("Error while compiling shader!"));
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    compile_shaders("slang", |source_file, output_file| {
        compile_slang(source_file, Some(output_file))
    })?;
    Ok(())
}
