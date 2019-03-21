#![feature(test)]

extern crate ecs;
extern crate test;
extern crate ecs_bench;

use test::Bencher;
use ecs_bench::parallel::{R, W1, W2, N};
use ecs::ECS;
use ecs::component::Component;
use ecs::component::dense_component_storage::DenseComponentStorage;
use ecs::component::storage::Storage;
use ecs::component::iter::Iter;

#[derive(Clone)]
struct RC(pub R);

impl Component for RC {
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct W1C(pub W1);
#[derive(Clone)]
struct W2C(pub W2);

impl Component for W1C {
    type ComponentStorage = DenseComponentStorage<Self>;
}

impl Component for W2C {
    type ComponentStorage = DenseComponentStorage<Self>;
}

fn build() -> ECS {
    let mut system: ECS = ECS::new();
    let _ = system.register_new_component::<RC>();
    let _ = system.register_new_component::<W1C>();
    let _ = system.register_new_component::<W2C>();
    // setup entities
    for _ in 0..N {
        let ent = system.allocate_new_entity();
        let _ = system.add_component(ent, RC(R{x: 0.0}));
        let _ = system.add_component(ent, W1C(W1{ x: 0.0 }));
        let _ = system.add_component(ent, W2C(W2{ x: 0.0 }));
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
        {
            let mut componentr = system.get_mut::<RC>();
            let mut componentw1 = system.get_mut::<W1C>();
            let riter = componentr.get_mut_iter();
            let w1iter = componentw1.get_mut_iter();
            let rw1iter = riter.join(w1iter).into_iterator_wrapper();
            for (r, w1) in rw1iter {
                w1.0.x = r.0.x;
            }
        }
        {
            let mut componentr = system.get_mut::<RC>();
            let mut componentw2 = system.get_mut::<W2C>();
            let riter = componentr.get_mut_iter();
            let w2iter = componentw2.get_mut_iter();
            let rw2iter = riter.join(w2iter).into_iterator_wrapper();
            for (r, w2) in rw2iter {
                w2.0.x = r.0.x;
            }
        }
    });
}

