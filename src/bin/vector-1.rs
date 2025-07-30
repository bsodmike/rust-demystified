#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;
use nalgebra::distance_squared;
use nalgebra::Point3;
use nalgebra::SimdComplexField;
use nalgebra::Vector3;
use rand::prelude::*;
use rand::Rng;

pub trait NewtonianMechanics<T: SimdComplexField> {
    fn get_mass(&self) -> T;

    fn get_position(&self) -> Point3<T>;
    fn get_velocity(&self) -> Vector3<T>;
    fn set_position(&mut self, position: Point3<T>);
    fn set_velocity(&mut self, direction: Vector3<T>);

    fn compute_force_vec(&self, other: &Self) -> Option<Vector3<T>>;
}

#[derive(Clone, Debug)]
pub struct PointMass<T: SimdComplexField> {
    position: Point3<T>,
    velocity: Vector3<T>,
    mass: T,
}

impl<T: SimdComplexField> PointMass<T> {
    pub fn new(position: Point3<T>, velocity: Vector3<T>, mass: T) -> Self {
        Self {
            position,
            velocity,
            mass,
        }
    }
}

const G: f32 = 6.67430e-5;

macro_rules! impl_mechanics_for_pointmass {
    ($t:ty) => {
        impl NewtonianMechanics<$t> for PointMass<$t> {
            fn get_mass(&self) -> $t {
                self.mass
            }

            fn get_position(&self) -> Point3<$t> {
                self.position
            }
            fn get_velocity(&self) -> Vector3<$t> {
                self.velocity
            }
            fn set_position(&mut self, position: Point3<$t>) {
                self.position = position;
            }
            fn set_velocity(&mut self, velocity: Vector3<$t>) {
                self.velocity = velocity;
            }

            fn compute_force_vec(&self, other: &Self) -> Option<Vector3<$t>> {
                if self.get_position() == other.get_position() {
                    return None;
                }
                let dist2 = distance_squared(&self.get_position(), &other.get_position());

                let force = <$t>::from(G) * self.get_mass() * other.get_mass() / dist2;

                let direction = other.get_position() - self.get_position();
                dbg!(direction);

                Some((direction * (force)).into())
            }
        }
    };
}

impl_mechanics_for_pointmass!(f32);
impl_mechanics_for_pointmass!(f64);

pub type PointMassCollection = Vec<PointMass<f32>>;

#[derive(Clone)]
pub struct Population {
    items: PointMassCollection,
}

impl Population {
    pub fn new(points: usize) -> Self {
        Self {
            items: Vec::with_capacity(points),
        }
    }

    pub fn add(&mut self, point_mass: PointMass<f32>) {
        self.items.push(point_mass);
    }

    pub fn get(&self) -> &PointMassCollection {
        &self.items
    }

    pub fn get_mut(&mut self) -> &mut PointMassCollection {
        &mut self.items
    }

    pub fn compute_next_positions(pop1: &mut Population, pop2: &mut Population, ns_per_frame: u64) {
        let mut p1_items = pop1.get_mut();
        let p2_items = pop2.get_mut();

        compute_next_positions(&mut p1_items, &p2_items, ns_per_frame);
    }
}

pub fn compute_next_positions<T>(
    next_set: &mut [impl NewtonianMechanics<T>],
    last_set: &[impl NewtonianMechanics<T>],
    ns_per_frame: u64,
) where
    T: SimdComplexField,
{
    let time_const = ns_per_frame as f64 / 1e9;

    for (i, point) in last_set.iter().enumerate() {
        let mut force = Vector3::new(T::zero(), T::zero(), T::zero());

        for other in last_set.iter() {
            if let Some(f) = point.compute_force_vec(other) {
                force += f;
            } else {
                continue;
            }
        }
        let acceleration: Vector3<T> = force / point.get_mass();
        let old_velocity: Vector3<T> = point.get_velocity();

        let new_velocity = old_velocity.clone()
            + (acceleration * T::from_simd_real(nalgebra::convert(time_const)));

        let new_position = point.get_position() + (old_velocity + new_velocity.clone());

        next_set[i].set_velocity(new_velocity);
        next_set[i].set_position(new_position);
    }
}

pub fn run(num_points: usize, max_init_speed: f32, max_mass: f32, spawn_radius: f32) {
    let mut population = Population::new(num_points);

    let mut rng = rand::rngs::StdRng::from_entropy();

    population.add(PointMass::new(
        nalgebra::Point3::new(
            rng.gen_range(-spawn_radius..spawn_radius),
            rng.gen_range(-spawn_radius..spawn_radius),
            rng.gen_range(-spawn_radius..spawn_radius),
        ),
        nalgebra::Vector3::new(
            rng.gen_range(-max_init_speed..max_init_speed),
            rng.gen_range(-max_init_speed..max_init_speed),
            rng.gen_range(-max_init_speed..max_init_speed),
        ),
        rng.gen_range(1.0..max_mass),
    ));
    let mass = rand::random::<f32>() * max_mass;

    dbg!(population.get());
}

const NUM_ENTITIES: usize = 1;
const MAX_INIT_SPEED: f32 = 1000.0;
const MAX_MASS: f32 = 100000000.0;
const SPAWN_RADIUS: f32 = 10000.0;

pub fn main() -> Result<(), Error> {
    run(NUM_ENTITIES, MAX_INIT_SPEED, MAX_MASS, SPAWN_RADIUS);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_force_vec_pre_computed() {
        let point1 = PointMass::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 1e4);
        let point2 = PointMass::new(Point3::new(2.0, 3.0, 6.0), Vector3::new(0.0, 0.0, 0.0), 1e4);

        let force = point1.compute_force_vec(&point2).unwrap();
        assert_eq!(
            force,
            Vector3::new(272.4204060374474, 408.63060905617107, 817.2612181123421)
        );
    }

    // FIXME: verify the formula here.
    #[test]
    fn test_compute_force_vec_unverified() {
        let point1 = PointMass::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0), 1e4);
        let point2 = PointMass::new(Point3::new(2.0, 3.0, 6.0), Vector3::new(0.0, 0.0, 0.0), 1e4);

        let force = point1.compute_force_vec(&point2).unwrap();
        assert!(
            force
                - Vector3::new(0.2857142857142857, 0.42857142857142855, 0.8571428571428571) * 1e8
                    / 49.0
                < Vector3::new(1e-6, 1e-6, 1e-6)
        );
    }
}
