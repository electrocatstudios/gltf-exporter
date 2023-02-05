# gltf-exporter
An expanded version of the example gltf export function from (gltf_json)[https://github.com/gltf-rs/gltf] crate.

Specifically it is based upon the example (here)[https://github.com/gltf-rs/gltf/blob/master/examples/export/main.rs]

The application uses a DSL to create GLTF objects and allows for UV mapping. This is deliberately not feature rich, if you need more features in the exported GLTF then it is probably best to clone this repo and add those. This exporter is created with the specific purpose of allowing code generated files to be created which can then be turned  into a GLTF, it will eventually tie in to a series on Bevy that is being prepared for YouTube. 