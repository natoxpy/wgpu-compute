# WGPU-COMPUTE
This is a wrapper library that makes using WGPU for compute purposes easier. 

# Usage
Here's some code to get you started.

```rs
let context = WGPUContext::new().await;

        let input_buffer = context.create_buffer_init(
            "input",
            wgpu::BufferUsages::STORAGE,
            bytemuck::bytes_of(&138),
        );

        let output_buffer = context.create_buffer(
            "output",
            wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            8,
        );

        let staging_buffer = context.create_buffer(
            "output stage",
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            8,
        );

        let shader = &context.create_shader(
            "compute shader",
            context.load_wgsl(
                "
            @group(0) @binding(0) var<storage, read> input : array<u32>; //input data
            @group(0) @binding(1) var<storage, read_write> output : array<atomic<u32>>; //input data
            //
            @compute @workgroup_size(1)
            fn main(
                @builtin(local_invocation_index) local_invocation_index: u32,
            ) {
                output[0] = input[0];
                atomicAdd(&output[1], 1u);
            }
            ",
            ),
        );

        let bind_group_layout = context.create_bind_group_layout(
            "bind group",
            &[
                context.create_bind_layput_entry(
                    0,
                    wgpu::ShaderStages::COMPUTE,
                    context.create_binding_type_buffer(true),
                ),
                context.create_bind_layput_entry(
                    1,
                    wgpu::ShaderStages::COMPUTE,
                    context.create_binding_type_buffer(false),
                ),
            ],
        );

        let pipeline_layout =
            &context.create_pipeline_layout("pipeline layout", &[&bind_group_layout]);

        let compute_pipeline =
            context.create_compute_pipeline("compute pipeline", pipeline_layout, shader, "main");

        let bind_group = context.create_bind_group(
            "bind group",
            &bind_group_layout,
            &[
                WGPUContext::create_bind_group_entry(0, &input_buffer),
                WGPUContext::create_bind_group_entry(1, &output_buffer),
            ],
        );

        let mut encoder = context.create_command_encoder("");

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute Pass"),
                timestamp_writes: None,
            });

            compute_pass.set_pipeline(&compute_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(128, 128, 128);
        }

        encoder.copy_buffer_to_buffer(
            &output_buffer,
            0,
            &staging_buffer,
            0,
            std::mem::size_of::<u32>() as u64,
        );

        encoder.copy_buffer_to_buffer(&output_buffer, 0, &staging_buffer, 0, output_buffer.size());

        context.queue.submit(Some(encoder.finish()));

        let buffer = staging_buffer.slice(..);
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();

        buffer.map_async(wgpu::MapMode::Read, move |x| sender.send(x).unwrap());
        context.device.poll(wgpu::Maintain::Wait);
        #[allow(unused_must_use)]
        receiver.receive().await.unwrap();
        let mapped = buffer.get_mapped_range().to_vec();

        let result = mapped
            .chunks(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect::<Vec<u32>>();

        println!("{:?}", result);
```
