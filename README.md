# Raytracer

A simple raytracer implemented from the [Ray Tracing in One Weekend](https://raytracing.github.io/) series, in [Rust] :crab:.

## Features

- Samples from the first book ([In One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)) are implemented.
- Extensible material, collider and shader implementations using trait objects
- Parallel rendering (use half CPU count by default, override by setting the `RT_THREAD_COUNT` environment variable).

## TODO

- Add more colliders, scenes, materials and shaders
- Add dynamic rendering by editing scene settings in the "window renderer" (using egui)
- Add scene scripting (using Lua? WASM?)
- Implement things from the second book ([The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html))
- Implement things from the third book ([The Rest of Your Life](https://raytracing.github.io/books/RayTracingTheRestOfYourLife.html))

[Rust]: https://www.rust-lang.org
