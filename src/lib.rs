use bevy_app::{stage, Plugin};
use bevy_asset::Handle;
use bevy_ecs::{Bundle, SystemStage};
use bevy_ecs::{Entity, IntoSystem, Query, Res, With, Without};
use bevy_math::{Vec2, Vec3};
use bevy_render::{
    camera::{Camera, OrthographicProjection, VisibleEntities, WindowOrigin},
    mesh::Mesh,
    pipeline::RenderPipeline,
    prelude::{Draw, RenderPipelines, Visible},
    render_graph::RenderGraph,
};
use bevy_sprite::{ColorMaterial, QUAD_HANDLE};
use bevy_transform::components::{Children, GlobalTransform, Parent, Transform};
use bevy_window::Windows;

mod layout;
mod render;
pub mod types;

use render::{UiRenderGraphBuilder, UI_PIPELINE_HANDLE};
pub use types::*;

#[derive(Bundle, Clone, Debug)]
pub struct AUINode {
    pub mesh: Handle<Mesh>, // TODO: maybe abstract this out
    pub draw: Draw,
    pub material: Handle<ColorMaterial>,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub node: ANode,
    pub render_data: AuiRender,
}

impl Default for AUINode {
    fn default() -> Self {
        Self {
            mesh: QUAD_HANDLE.typed(),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                UI_PIPELINE_HANDLE.typed(),
            )]),
            visible: Visible {
                is_transparent: true,
                ..Default::default()
            },
            node: Default::default(),
            material: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            render_data: Default::default(),
        }
    }
}

// Camera taken from bevy_ui

#[derive(Bundle, Debug)]
pub struct AUiCameraBundle {
    pub camera: Camera,
    pub orthographic_projection: OrthographicProjection,
    pub visible_entities: VisibleEntities,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for AUiCameraBundle {
    fn default() -> Self {
        // we want 0 to be "closest" and +far to be "farthest" in 2d, so we offset
        // the camera's translation by far and use a right handed coordinate system
        let far = 1000.0;
        AUiCameraBundle {
            camera: Camera {
                name: Some(render::camera::CAMERA_UI.to_string()),
                ..Default::default()
            },
            orthographic_projection: OrthographicProjection {
                far,
                window_origin: WindowOrigin::Center,
                ..Default::default()
            },
            visible_entities: Default::default(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, far - 0.1)),
            global_transform: Default::default(),
        }
    }
}

pub fn layout_system(
    roots: Query<Entity, (With<ANode>, Without<Parent>)>,
    nodes: Query<(&ANode, Option<&Children>)>,
    mut transforms: Query<(&mut Transform, &mut AuiRender)>,
    windows: Res<Windows>,
) {
    for (root, window) in roots.iter().zip(windows.iter()) {
        let window_size = Vec2::new(window.width(), window.height());
        layout::solve(root, window_size, 50., &nodes, &mut transforms);
    }
    println!("-------");
    for (t, s) in transforms.iter_mut() {
        println!("{:?} {:?}", t.translation.z, s.size);
    }
}

pub struct AUIPlugin;

const STAGE: &str = "aui";
impl Plugin for AUIPlugin {
    fn build(&self, app: &mut bevy_app::AppBuilder) {
        app.add_stage_before(stage::POST_UPDATE, STAGE, SystemStage::parallel())
            .add_system_to_stage(STAGE, layout_system.system());

        let resources = app.resources();
        resources
            .get_mut::<RenderGraph>()
            .unwrap()
            .add_ui_graph(resources);
    }
}
