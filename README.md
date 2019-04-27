# Benchmarks of various Rust Entity Component Systems

## Benchmarks
Benchmarks are run on [Travis CI](https://travis-ci.org/lschmierer/ecs_bench/).

Benchmarks are located in `benches/[bench_name]_[ecs_crate_name].rs`.

 Library         | pos_vel build                 | pos_vel update                 | parallel build                 | parallel update
 --------------- |:-----------------------------:|:------------------------------:|:------------------------------:|:--------------------------------:
 [calx-ecs]      | 504 µs/iter (+/- 9)      | 19 µs/iter (+/- 0)      | 848 µs/iter (+/- 11)      | 71 µs/iter (+/- 4)
 [constellation] | 307 µs/iter (+/- 4) | 9 µs/iter (+/- 0) | 502 µs/iter (+/- 9) | 117 µs/iter (+/- 5)
 [ecs]           | {pos_vel_build_ecs}           | {pos_vel_update_ecs}           | {parallel_build_ecs}           | {parallel_update_ecs}
 [froggy]        | 611 µs/iter (+/- 13)        | 12 µs/iter (+/- 0)        | 1,640 µs/iter (+/- 308)        | 68 µs/iter (+/- 18)
 [specs]         | 393 µs/iter (+/- 3)         | 5 µs/iter (+/- 0)         | 698 µs/iter (+/- 18)         | 49 µs/iter (+/- 4)
 [trex]          | 1,153 µs/iter (+/- 27)          | 237 µs/iter (+/- 16)          | 1,522 µs/iter (+/- 54)          | 440 µs/iter (+/- 7)

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
