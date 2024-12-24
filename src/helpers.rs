use std::fs;

use wgpu::{util::DeviceExt, Buffer};

use crate::WGPUContext;

impl WGPUContext {
    pub fn create_buffer_init(
        &self,
        label: &str,
        usage: wgpu::BufferUsages,
        contents: &[u8],
    ) -> Buffer {
        self.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents,
                usage,
            })
    }
    pub fn create_buffer(&self, label: &str, usage: wgpu::BufferUsages, size: u64) -> Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage,
            mapped_at_creation: false,
        })
    }

    pub fn load_wgsl(&self, wgsl: &str) -> wgpu::ShaderSource {
        wgpu::ShaderSource::Wgsl(wgsl.to_string().into())
    }

    pub fn load_wgsl_file(&self, path: String) -> wgpu::ShaderSource {
        self.load_wgsl(&fs::read_to_string(path).unwrap())
    }

    pub fn create_shader(&self, label: &str, source: wgpu::ShaderSource) -> wgpu::ShaderModule {
        self.device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some(label),
                source,
            })
    }

    pub fn create_binding_type_buffer(&self, readonly: bool) -> wgpu::BindingType {
        wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage {
                read_only: readonly,
            },
            has_dynamic_offset: false,
            min_binding_size: None,
        }
    }

    pub fn create_bind_layput_entry(
        &self,
        binding: u32,
        visibility: wgpu::ShaderStages,
        ty: wgpu::BindingType,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            ty,
            count: None,
        }
    }

    pub fn create_bind_group_layout(
        &self,
        label: &str,
        entries: &[wgpu::BindGroupLayoutEntry],
    ) -> wgpu::BindGroupLayout {
        self.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(label),
                entries,
            })
    }

    pub fn create_pipeline_layout(
        &self,
        label: &str,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::PipelineLayout {
        self.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts,
                push_constant_ranges: &[],
            })
    }

    pub fn create_compute_pipeline(
        &self,
        label: &str,
        pipeline_layout: &wgpu::PipelineLayout,
        shader: &wgpu::ShaderModule,
        entry_point: &str,
    ) -> wgpu::ComputePipeline {
        self.device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(label),
                layout: Some(pipeline_layout),
                module: shader,
                entry_point: Some(entry_point),
                cache: None,
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            })
    }

    pub fn create_bind_group_entry(binding: u32, buffer: &wgpu::Buffer) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: buffer.as_entire_binding(),
        }
    }

    pub fn create_bind_group(
        &self,
        label: &str,
        layout: &wgpu::BindGroupLayout,
        entries: &[wgpu::BindGroupEntry],
    ) -> wgpu::BindGroup {
        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout,
            entries,
        })
    }

    pub fn create_command_encoder(&self, label: &str) -> wgpu::CommandEncoder {
        self.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some(label) })
    }
}
