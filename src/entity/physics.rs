use nalgebra::{vector, Complex, Unit};
use rapier2d::{
    math::{Point, Real, Rotation, Vector},
    prelude::{Collider, ColliderHandle, ColliderSet, RigidBody, RigidBodyHandle, RigidBodySet},
};

use crate::util::{resource::Resource, screen::crop_to_world};

use super::entity::Entity;

pub struct Physics {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Physics {
    pub fn from_resource(
        resource: &Resource,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> Option<Physics> {
        let rigid_body = resource.rigid_body.clone()?;
        let collider = resource.collider.clone()?;

        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let collider_handle =
            collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

        Some(Physics {
            rigid_body_handle,
            collider_handle,
        })
    }
}

pub trait PhysicsLike {
    fn get_rigid_body<'a>(&self, rigid_body_set: &'a RigidBodySet) -> Option<&'a RigidBody>;
    fn get_rigid_body_mut<'a>(
        &self,
        rigid_body_set: &'a mut RigidBodySet,
    ) -> Option<&'a mut RigidBody>;

    fn get_collider<'a>(&self, collider_set: &'a ColliderSet) -> Option<&'a Collider>;
    fn get_collider_mut<'a>(&self, collider_set: &'a mut ColliderSet) -> Option<&'a mut Collider>;

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

    fn pos(&self, rigid_body_set: &RigidBodySet) -> Option<Point<Real>> {
        Some(
            self.get_rigid_body(rigid_body_set)?
                .position()
                .transform_point(&Point::origin()),
        )
    }

    fn velocity(&self, rigid_body_set: &RigidBodySet) -> Option<Vector<Real>> {
        Some(*self.get_rigid_body(rigid_body_set)?.linvel())
    }

    fn rotation(&self, rigid_body_set: &RigidBodySet) -> Option<Unit<Complex<f32>>> {
        Some(*self.get_rigid_body(rigid_body_set)?.rotation())
    }

    fn update_entity_position(&self, rigid_body_set: &mut RigidBodySet) -> Option<()> {
        /*
        let d_pos = self.rotation_to_unit_vector()? * self.velocity()?;

        let ref mut pos = self.physics.as_mut()?.pos;

        // velocity
        *pos += d_pos;
        */

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
