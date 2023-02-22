## Crates

| Crate                             | Description
|:----------------------------------|:---
| apis/pipewire                     | Bindings to libpipewire
| apis/soundio                      | Bindings to libsoundio
| apis/vk                           | Bindings to libvulkan
| apis/wayland                      | Bindings to libwayland
| apis/xkbcommon                    | Bindings to libxkbcommon
| apis/xr                           | Bindings to libopenxr
| codegen/khrgen                    | Code generator for Vulkan and OpenXR
| codegen/wlgen                     | Code generator for Wayland
| components/app                    | Basic application functionality and data types
| components/atomic-sync            | Concurrent data structures based on atomics
| components/ecs                    | Entity component system based on archetype organized storage
| components/file-formats           | Uitility crate for reading and writing various file formats
| components/render-manager         | Library for managing concurrent use of the Vulkan API by render plugins
| components/math                   | Vector math library
| components/scene                  | Scene components, such as UI elements, 3D meshes and lights
| components/vec-map                | Map data structure based on a single continous linear memory block
| plugins/glsl                      | Legacy GLSL shaders
| plugins/sdft                      | Plugin for rendering 3D scenes with SDF tracing
| plugins/ui                        | Plugin for rendering UIs

## Requirements

- Rust 1.69 nightly or higher
- Wayland
- Pipewire
- libxkbcommon
- Vulkan
- OpenXR (optional)