use nalgebra::{vector, Complex, Unit};
use rapier2d::{
    math::{Isometry, Point, Real, Rotation, Vector},
    parry::utils::IsometryOpt,
    prelude::{Collider, ColliderHandle, ColliderSet, RigidBody, RigidBodyHandle, RigidBodySet},
};

use crate::util::screen::crop_to_world;

use super::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub struct Physics {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

pub trait PhysicsLike {
    fn get_rigid_body<'a>(&self, rigid_body_set: &'a RigidBodySet) -> Option<&'a RigidBody>;
    fn get_rigid_body_mut<'a>(
        &self,
        rigid_body_set: &'a mut RigidBodySet,
    ) -> Option<&'a mut RigidBody>;

    fn get_collider<'a>(&self, collider_set: &'a ColliderSet) -> Option<&'a Collider>;
    fn get_collider_mut<'a>(&self, collider_set: &'a mut ColliderSet) -> Option<&'a mut Collider>;

    fn transform(&self, rigid_body_set: &RigidBodySet) -> Option<Isometry<Real>>;
    fn pos(&self, rigid_body_set: &RigidBodySet) -> Option<Point<Real>>;
    fn velocity(&self, rigid_body_set: &RigidBodySet) -> Option<Vector<Real>>;
    fn rotation(&self, rigid_body_set: &RigidBodySet) -> Option<Rotation<Real>>;

    fn update_entity_position(&self, rigid_body_set: &mut RigidBodySet) -> Option<()>;
}

impl PhysicsLike for Entity {
    fn get_rigid_body<'a>(&self, rigid_body_set: &'a RigidBodySet) -> Option<&'a RigidBody> {
        rigid_body_set.get(self.physics.as_ref()?.rigid_body_handle)
    }

    fn get_rigid_body_mut<'a>(
        &self,
        rigid_body_set: &'a mut RigidBodySet,
    ) -> Option<&'a mut RigidBody> {
        rigid_body_set.get_mut(self.physics.as_ref()?.rigid_body_handle)
    }

    fn get_collider<'a>(&self, collider_set: &'a ColliderSet) -> Option<&'a Collider> {
        collider_set.get(self.physics.as_ref()?.collider_handle)
    }
    fn get_collider_mut<'a>(&self, collider_set: &'a mut ColliderSet) -> Option<&'a mut Collider> {
        collider_set.get_mut(self.physics.as_ref()?.collider_handle)
    }

    fn transform(&self, rigid_body_set: &RigidBodySet) -> Option<Isometry<Real>> {
        Some(*self.get_rigid_body(rigid_body_set)?.position())
    }

    fn pos(&self, rigid_body_set: &RigidBodySet) -> Option<Point<Real>> {
        Some(
            self.transform(rigid_body_set)
                .transform_point(&Point::origin()),
        )
    }

    fn velocity(&self, rigid_body_set: &RigidBodySet) -> Option<Vector<Real>> {
        Some(*self.get_rigid_body(rigid_body_set)?.linvel())
    }

    fn rotation(&self, rigid_body_set: &RigidBodySet) -> Option<Unit<Complex<Real>>> {
        Some(*self.get_rigid_body(rigid_body_set)?.rotation())
    }

    fn update_entity_position(&self, rigid_body_set: &mut RigidBodySet) -> Option<()> {
        let pos = self.pos(rigid_body_set)?;

        let rigid_body = self.get_rigid_body_mut(rigid_body_set)?;

        // wrap
        let pos = crop_to_world(pos);

        let pos = vector!(pos.x, pos.y);

        // update
        rigid_body.set_translation(pos, true);

        Some(())
    }
}
