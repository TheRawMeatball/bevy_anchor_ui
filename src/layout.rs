use bevy_ecs::{Entity, Query};
use bevy_math::Vec2;
use bevy_transform::components::{Children, Transform};

use crate::{AuiRender, ANode, AxisConstraint, Constraint};

pub(crate) fn solve(
    solve_target: Entity,
    space: Vec2,
    nodes: &Query<(&ANode, Option<&Children>)>,
    transforms: &mut Query<(&mut Transform, &mut AuiRender)>,
) {
    let (mut target_transform, mut render_data) = transforms.get_mut(solve_target).unwrap();
    let target_size = &mut render_data.size;
    let (solve_target, children) = nodes.get(solve_target).unwrap();

    match &solve_target.constraint {
        Constraint::Independent { x, y } => {
            let x = x.solve(solve_target.anchors_x, space.x);
            let y = y.solve(solve_target.anchors_y, space.y);

            target_transform.translation = Vec2::new(x.offset, y.offset).extend(0.);
            *target_size = Vec2::new(x.size, y.size);
        }
        Constraint::SetXWithY { y, aspect } => {
            let y = y.solve(solve_target.anchors_y, space.y);
            let x =
                AxisConstraint::Centered(y.size * aspect).solve(solve_target.anchors_x, space.x);

            target_transform.translation = Vec2::new(x.offset, y.offset).extend(0.);
            *target_size = Vec2::new(x.size, y.size);
        }
        Constraint::SetYWithX { x, aspect } => {
            let x = x.solve(solve_target.anchors_x, space.x);
            let y =
                AxisConstraint::Centered(x.size / aspect).solve(solve_target.anchors_y, space.y);

            target_transform.translation = Vec2::new(x.offset, y.offset).extend(0.);
            *target_size = Vec2::new(x.size, y.size);
        }
        Constraint::MaxAspect(aspect) => {
            let x_from_y = space.y * aspect;
            let y_from_x = space.x / aspect;

            *target_size = if x_from_y <= space.x {
                Vec2::new(space.x, y_from_x)
            } else {
                Vec2::new(x_from_y, space.y)
            };
        }
        Constraint::ParentSpecified(spec) => {
            let x = spec.padding_x.solve(solve_target.anchors_x, space.x);
            let y = spec.padding_y.solve(solve_target.anchors_y, space.y);
            *target_size = Vec2::new(x.size, y.size);
        }
    }

    if let Some(children) = children {
        if let Some(_spread_constraint) = &solve_target.children_spread {
            todo!();
            // let child_nodes = children
            //     .iter()
            //     .map(|c| (nodes.get_component::<ANode>(*c).unwrap(), c))
            //     .collect::<Vec<_>>();
            // let offset = spread_constraint.outer_margin;
            // let available_size = match spread_constraint.direction {
            //     Direction::Left | Direction::Right => size.x,
            //     Direction::Up | Direction::Down => size.y,
            // } - offset * 2.;
            // if spread_constraint.stretch {
            //     let total_weight: f32 = child_nodes
            //         .iter()
            //         .map(|c| c.0.constraint.unwrap_parent_specified().weight)
            //         .sum();

            //     for (node, entity) in child_nodes.into_iter() {
            //         let transform = transforms.get_mut(*entity).unwrap();

            //         let constraint = node.constraint.unwrap_parent_specified();
            //         let relative_weight = constraint.weight / total_weight;
            //     }
            // } else {
            //     for (i, (node, entity)) in child_nodes.into_iter().enumerate() {
            //         let mut transform = transforms.get_mut(*entity).unwrap();
            //         let constraint = node.constraint.unwrap_parent_specified();

            //     }
            // }
        } else {
            let ts = target_size.clone();
            for child in children.iter() {
                solve(*child, ts, nodes, transforms);
            }
        }
    }
}

impl AxisConstraint {
    fn solve(self, anchors: (f32, f32), true_space: f32) -> AxisConstraintSolve {
        let space = (anchors.1 - anchors.0) * true_space;

        let (p1, s) = match self {
            AxisConstraint::DoublePadding(p1, p2) => (p1, space - p1 - p2),
            AxisConstraint::PaddingAndSize(p1, s) => (p1, s),
            AxisConstraint::InversePaddingAndSize(p2, s) => (space - p2 - s, s),
            AxisConstraint::Centered(s) => ((space - s) / 2., s),
        };
        let offset = true_space * (anchors.0 - 0.5) + p1 + s / 2.;
        AxisConstraintSolve { offset, size: s }
    }
}

struct AxisConstraintSolve {
    offset: f32,
    size: f32,
}
