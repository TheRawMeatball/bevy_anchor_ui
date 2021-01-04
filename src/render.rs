use bevy_asset::{Assets, HandleUntyped};
use bevy_ecs::Resources;
use bevy_math::Vec2;
use bevy_reflect::Reflect;
use bevy_reflect::TypeUuid;
use bevy_render::{
    camera::ActiveCameras,
    pass::{
        LoadOp, Operations, PassDescriptor, RenderPassDepthStencilAttachmentDescriptor,
        TextureAttachment,
    },
    pipeline::*,
    prelude::Msaa,
    render_graph::{
        base, CameraNode, PassNode, RenderGraph, RenderResourcesNode, WindowSwapChainNode,
        WindowTextureNode,
    },
    renderer::RenderResources,
    shader::{Shader, ShaderStage, ShaderStages},
    texture::TextureFormat,
};

use crate::ANode;

pub trait UiRenderGraphBuilder {
    fn add_ui_graph(&mut self, resources: &Resources) -> &mut Self;
}

mod node {
    pub const AUI_PASS: &str = "aui-pass";
    pub const AUI_CAM: &str = "aui-cam";
    pub const AUI_NODE: &str = "aui-node";
}

pub mod camera {
    pub const AUI_CAM: &str = "AUI-CAM";
}

#[derive(Default, RenderResources, Reflect, Clone, Debug)]
pub struct AuiRender {
    pub size: Vec2,
}

pub const UI_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 3232131430);

pub fn build_ui_pipeline(shaders: &mut Assets<Shader>, sample_count: u32) -> PipelineDescriptor {
    PipelineDescriptor {
        rasterization_state: Some(RasterizationStateDescriptor {
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
            clamp_depth: false,
        }),
        depth_stencil_state: Some(DepthStencilStateDescriptor {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: CompareFunction::Less,
            stencil: StencilStateDescriptor {
                front: StencilStateFaceDescriptor::IGNORE,
                back: StencilStateFaceDescriptor::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
        }),
        color_states: vec![ColorStateDescriptor {
            format: TextureFormat::default(),
            color_blend: BlendDescriptor {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendDescriptor {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        sample_count,
        ..PipelineDescriptor::new(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("ui.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("ui.frag"),
            ))),
        })
    }
}

impl UiRenderGraphBuilder for RenderGraph {
    fn add_ui_graph(&mut self, resources: &Resources) -> &mut Self {
        let mut pipelines = resources.get_mut::<Assets<PipelineDescriptor>>().unwrap();
        let mut shaders = resources.get_mut::<Assets<Shader>>().unwrap();
        let msaa = resources.get::<Msaa>().unwrap();
        pipelines.set_untracked(
            UI_PIPELINE_HANDLE,
            build_ui_pipeline(&mut shaders, msaa.samples),
        );

        let mut ui_pass_node = PassNode::<&ANode>::new(PassDescriptor {
            color_attachments: vec![msaa.color_attachment_descriptor(
                TextureAttachment::Input("color_attachment".to_string()),
                TextureAttachment::Input("color_resolve_target".to_string()),
                Operations {
                    load: LoadOp::Load,
                    store: true,
                },
            )],
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachmentDescriptor {
                attachment: TextureAttachment::Input("depth".to_string()),
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
            sample_count: msaa.samples,
        });

        ui_pass_node.add_camera(camera::AUI_CAM);
        self.add_node(node::AUI_PASS, ui_pass_node);

        self.add_slot_edge(
            base::node::PRIMARY_SWAP_CHAIN,
            WindowSwapChainNode::OUT_TEXTURE,
            node::AUI_PASS,
            if msaa.samples > 1 {
                "color_resolve_target"
            } else {
                "color_attachment"
            },
        )
        .unwrap();

        self.add_slot_edge(
            base::node::MAIN_DEPTH_TEXTURE,
            WindowTextureNode::OUT_TEXTURE,
            node::AUI_PASS,
            "depth",
        )
        .unwrap();

        if msaa.samples > 1 {
            self.add_slot_edge(
                base::node::MAIN_SAMPLED_COLOR_ATTACHMENT,
                WindowSwapChainNode::OUT_TEXTURE,
                node::AUI_PASS,
                "color_attachment",
            )
            .unwrap();
        }

        // ensure ui pass runs after main pass
        self.add_node_edge(base::node::MAIN_PASS, node::AUI_PASS)
            .unwrap();

        // setup ui camera
        self.add_system_node(node::AUI_CAM, CameraNode::new(camera::AUI_CAM));
        self.add_node_edge(node::AUI_CAM, node::AUI_PASS).unwrap();
        self.add_system_node(node::AUI_NODE, RenderResourcesNode::<AuiRender>::new(true));
        self.add_node_edge(node::AUI_NODE, node::AUI_PASS).unwrap();
        let mut active_cameras = resources.get_mut::<ActiveCameras>().unwrap();
        active_cameras.add(camera::AUI_CAM);
        self
    }
}
