use shaderc;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn compiler_from_binary(device: &wgpu::Device, source: &str, stage: wgpu::ShaderStage) -> Result<wgpu::ShaderModule> {
    let mut compiler = shaderc::Compiler::new().unwrap();

    let kind = {
        match stage {
            wgpu::ShaderStage::VERTEX => shaderc::ShaderKind::Vertex,
            wgpu::ShaderStage::FRAGMENT => shaderc::ShaderKind::Fragment,
            wgpu::ShaderStage::COMPUTE => shaderc::ShaderKind::Compute,
            _ => panic!("Faild to convert shader type!"),
        }
    };

    let binary_result = compiler.compile_into_spirv(
        source, 
        kind,
        "shader.glsl", 
        "main", 
        None)?;
    
    let module = device.create_shader_module(wgpu::ShaderModuleSource::SpirV(binary_result.as_binary().into()));

    Ok(module)
}