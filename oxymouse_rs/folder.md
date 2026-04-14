oxymouse_rs/
├── Cargo.toml
└── src/
    ├── lib.rs           (Equivalent to __init__.py)
    ├── mouse.rs         (The main Coordinator)
    ├── utils.rs         (General helpers)
    └── algorithms/      (Folder for specific math)
        ├── mod.rs
        ├── bezier.rs    (Bezier curves)
        ├── gaussian.rs  (Gaussian noise)
        └── perlin.rs    (Perlin noise)