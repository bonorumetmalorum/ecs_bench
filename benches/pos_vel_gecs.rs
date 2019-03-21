#![feature(test)]
extern crate test;
extern crate ecs;
use test::Bencher;

#[macro_use]
extern crate recs;
extern crate ecs_bench;


use ecs_bench::pos_vel::{Position, Velocity, N_POS_PER_VEL, N_POS};
use ecs::ECS;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use ecs::component::storage::Storage;
use ecs::component::iter::Iter;

#[derive(Clone)]
struct PC(pub Position);
#[derive(Clone)]
struct VC(pub Velocity);

impl Component for PC {
    type ComponentStorage = DenseComponentStorage<Self>;
}

impl Component for VC {
    type ComponentStorage = DenseComponentStorage<Self>;

}

fn build() -> ECS {
    let mut system: ECS = ECS::new();
    system.register_new_component::<PC>();
    system.register_new_component::<VC>();
    // setup entities
    for i in 0..N_POS {
        let ent = system.allocate_new_entity();
        let _ = system.add_component(ent, PC(Position { x: 0.0, y: 0.0 }));
        if i % N_POS_PER_VEL == 0 {
            let _ = system.add_component(ent, VC(Velocity { dx: 0.0, dy: 0.0 }));
        }
    }

    system
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut system = build();

    b.iter(|| {
        //update positions


        {
            let mut vel = system.get_mut::<VC>();
            let mut pos = system.get_mut::<PC>();
            let vel_iter = vel.get_mut_iter();
            let post_iter = pos.get_mut_iter();
            let joint_iter = vel_iter.join(post_iter).into_iterator_wrapper();

            for (vel, pos) in joint_iter {
                pos.0.x += vel.0.dx;
                pos.0.y += vel.0.dy;
            }
        }

        {
            //render stub
            let readonly_pos = system.get::<PC>();
            let position_iter = readonly_pos.get_iter();
            for pos in position_iter.into_iterator_wrapper(){
                let _ = pos;
            }
        }

    });


}
