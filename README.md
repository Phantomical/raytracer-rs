# raytracer-rs
This is a raymarcher implemented in rust. It currently supports a variety of geometric primitives 
and lights. It implements shadows but does not implement global illumination yet. It supports exporting 

## Usage
Currently there is no scene description language supported so all scenes are created by creating the
scene description in Rust. The methods in `builder.rs` provide an easy way of doing this. Each object
within the scene requires a material and a raymarchable object to be provided. 

### Creating custom objects
Custom objects can be created by implementing the `object::Raymarchable` trait. The only method that
must be implemented is the `distance` function and all the other methods will be automatically 
implemented. If the default normal calculation is insufficient, then implementing the `normal_at` 
function allows for using custom normal calculation logic.

### Creating custom lights
Custom lights can be created by deriving the `light::Light` trait. This requires implementing the
`illumination` function and the `shadow_rays` method. The `illumination` should return the colour 
at that point, while `shadow_rays` should provide an iterator over all of the shadow rays required.
For hard shadows only one shadow ray is necessary, but multiple shadow rays will allow for soft
shadows as long as they are different rays.

## Implementation
The raytracer is implemented by raymarching for each line in parallel. This means that objects,
materials, and lights should not modify their state when called as it will likely result in 
undefined behaviour.

## Examples
Multiple example scenes are included with the library. They are located within the `src/bin` 
subdirectory. They can be run by executing 
```
cargo run --release --bin <example-name> <output-image-name>
```
in your favorite command line.
