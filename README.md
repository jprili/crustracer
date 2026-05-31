# crustracer
Small path-tracing engine in Rust https://raytracing.github.io/books/RayTracingInOneWeekend.html.

# Setup
To setup the project, clone this repository and build the cargo file.
```bash
git clone https://github.com/jprili/crustracer
cargo build --release
```

and to run, do:
```bash
cargo run > [filename].ppm
```

We can convert the output to png by using `ffmpeg`:
```bash
ffmpeg -i [filename].ppm [output-name].png
```

# Sample renders
![sample-01](image.png)

![sample-02](final_render.png)

# TODO
Next steps would be:
1. Clean-up the Camera module
2. Add parallelism
3. Build with WASM target? 
   Might want to put it on my website but I doubt the 
   host's computing power is enough (without getting banned).
