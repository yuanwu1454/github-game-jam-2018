///Finds colliders without meshes and creates meshes for them

use amethyst::{
  assets::{
    AssetStorage,
    Loader,
  },
  ecs::prelude::*,
  renderer::{
    Material,
    MaterialDefaults,
    Mesh,
    MeshHandle,
    PosNormTex,
    Texture,
  },
};

use random_color::RandomColor;
use ::components::Shape as ShapeComponent;

#[derive(Default)]
pub struct ShapeVisualizer;

impl<'s> System<'s> for ShapeVisualizer {
  type SystemData = (
    Entities<'s>,
    ReadStorage<'s, ShapeComponent>,
    Read<'s, LazyUpdate>,
    ReadExpect<'s, MaterialDefaults>,
    ReadExpect<'s, Loader>,
    ReadExpect<'s, AssetStorage<Texture>>,
    ReadExpect<'s, AssetStorage<Mesh>>,
    ReadStorage<'s, MeshHandle>,
  );

  fn run(&mut self, (entities, shapes, updater, material_defaults, loader, texture_storage, mesh_storage, meshes): Self::SystemData) {
    //Create meshes for shapes that don't have them
    for (entity, shape, _) in (&entities, &shapes, !&meshes).join() {
      //Material
      let color = RandomColor::new().to_rgb_array();
      let color = [
        color[0] as f32 / 255.0,
        color[1] as f32 / 255.0,
        color[2] as f32 / 255.0,
        1.0];
      let material = create_colour_material(
        &material_defaults,
        &texture_storage,
        &loader,
        color,
      );
      updater.insert(entity, material);

      //Mesh
      let mesh = {
        let verts = shape.shape.generate_vertices::<Vec<PosNormTex>>(Some(shape.scale));
        let mesh = loader.load_from_data(verts.into(), (), &mesh_storage);
        mesh
      };
      updater.insert(entity, mesh);
    }
  }
}

/// Creates a solid material of the specified colour.
fn create_colour_material(
  material_defaults: &MaterialDefaults,
  texture_storage: &AssetStorage<Texture>,
  loader: &Loader,
  colour: [f32; 4]
) -> Material {
  let albedo = loader.load_from_data(colour.into(), (), texture_storage);
  Material {
    albedo,
    ..material_defaults.0.clone()
  }
}