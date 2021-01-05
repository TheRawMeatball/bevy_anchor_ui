use bevy_reflect::Reflect;
use bevy_render::renderer::RenderResources;
use bevy_math::Vec2;

#[derive(Clone, Debug, Default)]
pub struct ANode {
    pub anchors: Anchors,
    pub constraint: Constraint,
    pub children_spread: Option<SpreadConstraint>,
    pub child_constraint: Option<ChildConstraint>,
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
pub struct ChildConstraint {
    pub weight: f32,
    pub min_size: f32,
    pub max_size: f32,
}

impl Default for ChildConstraint {
    fn default() -> Self {
        Self {
            weight: 1.,
            min_size: 0.,
            max_size: f32::MAX,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpreadConstraint {
    pub margin: f32,
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

// Anchors taken directly from bevy_ui (except for the functions x and y)

#[derive(Debug, Clone)]
pub struct Anchors {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Anchors {
    pub const BOTTOM_FULL: Anchors = Anchors::new(0.0, 1.0, 0.0, 0.0);
    pub const BOTTOM_LEFT: Anchors = Anchors::new(0.0, 0.0, 0.0, 0.0);
    pub const BOTTOM_RIGHT: Anchors = Anchors::new(1.0, 1.0, 0.0, 0.0);
    pub const CENTER: Anchors = Anchors::new(0.5, 0.5, 0.5, 0.5);
    pub const CENTER_BOTTOM: Anchors = Anchors::new(0.5, 0.5, 0.0, 0.0);
    pub const CENTER_FULL_HORIZONTAL: Anchors = Anchors::new(0.0, 1.0, 0.5, 0.5);
    pub const CENTER_FULL_VERTICAL: Anchors = Anchors::new(0.5, 0.5, 0.0, 1.0);
    pub const CENTER_LEFT: Anchors = Anchors::new(0.0, 0.0, 0.5, 0.5);
    pub const CENTER_RIGHT: Anchors = Anchors::new(1.0, 1.0, 0.5, 0.5);
    pub const CENTER_TOP: Anchors = Anchors::new(0.5, 0.5, 1.0, 1.0);
    pub const FULL: Anchors = Anchors::new(0.0, 1.0, 0.0, 1.0);
    pub const LEFT_FULL: Anchors = Anchors::new(0.0, 0.0, 0.0, 1.0);
    pub const RIGHT_FULL: Anchors = Anchors::new(1.0, 1.0, 0.0, 1.0);
    pub const TOP_FULL: Anchors = Anchors::new(0.0, 1.0, 1.0, 1.0);
    pub const TOP_LEFT: Anchors = Anchors::new(0.0, 0.0, 1.0, 1.0);
    pub const TOP_RIGHT: Anchors = Anchors::new(1.0, 1.0, 1.0, 1.0);

    pub const fn new(left: f32, right: f32, bottom: f32, top: f32) -> Self {
        Anchors {
            left,
            right,
            bottom,
            top,
        }
    }

    pub const fn x(&self) -> (f32, f32) {
        (self.left, self.right)
    }
    pub const fn y(&self) -> (f32, f32) {
        (self.bottom, self.top)
    }
}

impl Default for Anchors {
    fn default() -> Self {
        Anchors {
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
        }
    }
}
