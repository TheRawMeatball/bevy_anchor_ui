use bevy_reflect::Reflect;
use bevy_render::renderer::RenderResources;
use bevy_math::Vec2;

#[derive(Clone, Debug)]
pub struct ANode {
    pub anchors_x: (f32, f32),
    pub anchors_y: (f32, f32),
    pub constraint: Constraint,
    pub children_spread: Option<SpreadConstraint>,
}

impl Default for ANode {
    fn default() -> Self {
        Self {
            anchors_x: (0., 1.),
            anchors_y: (0., 1.),
            constraint: Default::default(),
            children_spread: None,
        }
    }
}
#[derive(Clone, Debug)]
pub enum Constraint {
    Independent {
        x: AxisConstraint,
        y: AxisConstraint,
    },
    SetXWithY {
        y: AxisConstraint,
        aspect: f32,
    },
    SetYWithX {
        x: AxisConstraint,
        aspect: f32,
    },
    MaxAspect(f32),
    /// All and only direct children of nodes with a children spread constraint will use this option.
    ParentSpecified(ParentSpecified),
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint::Independent {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

#[derive(Default, RenderResources, Reflect, Clone, Debug)]
pub struct AuiRender {
    pub size: Vec2,
}

#[derive(Clone, Debug)]
pub struct ParentSpecified {
    pub weight: f32,
    pub min_size: Option<f32>,
    pub preffered_size: Option<f32>,
    pub max_size: Option<f32>,
    pub padding_x: AxisConstraint,
    pub padding_y: AxisConstraint,
}

impl Constraint {
    fn unwrap_parent_specified(&self) -> &ParentSpecified {
        match self {
            Constraint::ParentSpecified(v) => v,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpreadConstraint {
    pub inner_margin: f32,
    pub outer_margin: f32,
    pub stretch: bool,
    pub direction: Direction,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum AxisConstraint {
    DoublePadding(f32, f32),
    PaddingAndSize(f32, f32),
    InversePaddingAndSize(f32, f32),
    Centered(f32),
}

impl Default for AxisConstraint {
    fn default() -> Self {
        AxisConstraint::DoublePadding(0., 0.)
    }
}
