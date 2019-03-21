# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 506 µs/iter (+/- 11)      | 19 µs/iter (+/- 0)      | 852 µs/iter (+/- 10)      | 72 µs/iter (+/- 2)
 [constellation] | 311 µs/iter (+/- 13) | 9 µs/iter (+/- 0) | 502 µs/iter (+/- 6) | 118 µs/iter (+/- 6)
 [ecs]           | {pos_vel_build_ecs}           | {pos_vel_update_ecs}           | {parallel_build_ecs}           | {parallel_update_ecs}
 [froggy]        | 623 µs/iter (+/- 5)        | 16 µs/iter (+/- 0)        | 1,639 µs/iter (+/- 28)        | 66 µs/iter (+/- 8)
 [specs]         | 380 µs/iter (+/- 8)         | 5 µs/iter (+/- 0)         | 722 µs/iter (+/- 9)         | 50 µs/iter (+/- 2)
 [trex]          | 1,157 µs/iter (+/- 7)          | 239 µs/iter (+/- 11)          | 1,536 µs/iter (+/- 30)          | 430 µs/iter (+/- 28)

[calx-ecs]: https://github.com/rsaarelm/calx-ecs
[constellation]: https://github.com/TomGillen/constellation/
[ecs]: https://github.com/HeroesGrave/ecs-rs
[froggy]: https://github.com/kvark/froggy
[specs]: https://github.com/slide-rs/specs
[trex]: https://github.com/rcolinray/trex


Visualization of benchmarks, smaller is better.
![update benchmarks graph](./graph/update.png)
![build benchmarks graph](./graph/build.png)

### pos_vel
 * 1000 entities with `position` and `velocity` components
 * 9000 entities with `position` components only
 * stub `render` system
 * `physics` system: `position += velocity`

### parallel
 * 10000 entities with 3 simple components `R`, `W1` and `W2`
 * `w1` system reads `R` and writes to `W1`
 * `w2` system reads `R` and writes to `W2`
 * systems could be run in parallel

## Notes
 * the benchmarks explore a limited subset of ECS use-cases and do not necessarily reflect the peformance of large-scale applications
 * [froggy](https://github.com/kvark/froggy) is technically not an ECS, but a Component Graph System (CGS)
