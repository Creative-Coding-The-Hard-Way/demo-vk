use {
    anyhow::{Context, Result},
    ash::vk,
    demo_vk::{
        demo::Graphics,
        graphics::vulkan::{raii, spirv_words},
    },
    std::ffi::CString,
};

pub fn create_pipeline(gfx: &Graphics) -> Result<raii::Pipeline> {
    let pipeline_layout = raii::PipelineLayout::new(
        "FirstTriangle",
        gfx.vulkan.device.clone(),
        &vk::PipelineLayoutCreateInfo {
            set_layout_count: 0,
            p_set_layouts: std::ptr::null(),
            push_constant_range_count: 0,
            p_push_constant_ranges: std::ptr::null(),
            ..Default::default()
        },
    )?;

    let vertex_shader_bytes =
        spirv_words(include_bytes!("./triangle.vert.spv"))
            .context("Unable to unpack vertex shader bytes.")?;

    let vertex_shader_module = raii::ShaderModule::new(
        "Vertex Shader",
        gfx.vulkan.device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: vertex_shader_bytes.len() * 4,
            p_code: vertex_shader_bytes.as_ptr(),
            ..Default::default()
        },
    )?;

    let fragment_shader_bytes =
        spirv_words(include_bytes!("./triangle.frag.spv"))
            .context("Unable to unpack fragment shader bytes.")?;

    let fragment_shader_module = raii::ShaderModule::new(
        "Fragment Shader",
        gfx.vulkan.device.clone(),
        &vk::ShaderModuleCreateInfo {
            code_size: fragment_shader_bytes.len() * 4,
            p_code: fragment_shader_bytes.as_ptr(),
            ..Default::default()
        },
    )?;

    let entrypoint = CString::new("main").unwrap();

    let stages = [
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::VERTEX,
            module: vertex_shader_module.raw,
            p_name: entrypoint.as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: fragment_shader_module.raw,
            p_name: entrypoint.as_ptr(),
            ..Default::default()
        },
    ];

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        vertex_binding_description_count: 0,
        vertex_attribute_description_count: 0,
        ..Default::default()
    };
    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
        ..Default::default()
    };
    let tesselation_state = vk::PipelineTessellationStateCreateInfo {
        patch_control_points: 0,
        ..Default::default()
    };
    let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
        depth_clamp_enable: vk::FALSE,
        rasterizer_discard_enable: vk::FALSE,
        polygon_mode: vk::PolygonMode::FILL,
        cull_mode: vk::CullModeFlags::NONE,
        front_face: vk::FrontFace::COUNTER_CLOCKWISE,
        depth_bias_enable: vk::FALSE,
        depth_bias_constant_factor: 0.0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
        line_width: 1.0,
        ..Default::default()
    };
    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        sample_shading_enable: vk::FALSE,
        min_sample_shading: 1.0,
        p_sample_mask: std::ptr::null(),
        alpha_to_coverage_enable: vk::FALSE,
        alpha_to_one_enable: vk::FALSE,
        ..Default::default()
    };
    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
        depth_test_enable: vk::FALSE,
        depth_write_enable: vk::FALSE,
        depth_compare_op: vk::CompareOp::LESS,
        depth_bounds_test_enable: vk::FALSE,
        stencil_test_enable: vk::FALSE,
        min_depth_bounds: 0.0,
        max_depth_bounds: 1.0,
        ..Default::default()
    };
    let color_blend_statetachment_state =
        vk::PipelineColorBlendAttachmentState {
            blend_enable: vk::FALSE,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::RGBA,
        };
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: vk::FALSE,
        logic_op: vk::LogicOp::COPY,
        attachment_count: 1,
        p_attachments: &color_blend_statetachment_state,
        blend_constants: [0.0, 0.0, 0.0, 0.0],
        ..Default::default()
    };

    let dynamic_states =
        [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
    let dynamic_state = vk::PipelineDynamicStateCreateInfo {
        dynamic_state_count: dynamic_states.len() as u32,
        p_dynamic_states: dynamic_states.as_ptr(),
        ..Default::default()
    };

    let viewport_state = vk::PipelineViewportStateCreateInfo {
        viewport_count: 1,
        scissor_count: 1,
        ..Default::default()
    };

    let color_attachment_formats = [gfx.swapchain.format()];
    let mut rendering_info = vk::PipelineRenderingCreateInfo {
        view_mask: 0,
        color_attachment_count: 1,
        p_color_attachment_formats: color_attachment_formats.as_ptr(),
        ..Default::default()
    };

    let create_info = vk::GraphicsPipelineCreateInfo {
        stage_count: stages.len() as u32,
        p_stages: stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly_state,
        p_tessellation_state: &tesselation_state,
        p_viewport_state: &viewport_state,
        p_rasterization_state: &rasterization_state,
        p_multisample_state: &multisample_state,
        p_depth_stencil_state: &depth_stencil_state,
        p_color_blend_state: &color_blend_state,
        p_dynamic_state: &dynamic_state,
        layout: pipeline_layout.raw,
        render_pass: vk::RenderPass::null(),
        subpass: 0,
        base_pipeline_handle: vk::Pipeline::null(),
        base_pipeline_index: 0,
        ..Default::default()
    }
    .push_next(&mut rendering_info);

    raii::Pipeline::new_graphics_pipeline(
        gfx.vulkan.device.clone(),
        &create_info,
    )
}
