use amethyst::{
  controls::FlyControlTag,
  core::{
    transform::components::Transform,
    cgmath::Vector3,
    Time,
  },
  ecs::prelude::*,
};

use ::{
  components::{
    Matriarch,
    Walker,
    Direction,
  },
  config::CameraConfig,
  resources::{
    Command,
    CommandChannel,
    SpawnStats,
  },
};

///Moves the camera. Tracks the matriarch (or the exit after some have exited).
#[derive(Default)]
pub struct CameraMovement {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for CameraMovement {
  type SystemData = (
    Read<'s, Time>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, FlyControlTag>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Walker>,
    Read<'s, CameraConfig>,
    Read<'s, CommandChannel>,
    Read<'s, SpawnStats>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (time, mut transforms, fly_tags, matriarchs, walkers, camera_config, commands, spawn_stats): Self::SystemData) {
    let delta = time.delta_seconds();

    let mut zoom = 0.0;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::Zoom(amount) => zoom = *amount,
        _ => {},
      }
    }

    let final_target = if spawn_stats.saved > 0 && camera_config.final_position.is_some() {
      Some((camera_config.final_position.unwrap(), true))
    } else {
      let mut matriarch_translation = Vector3::new(0.0, 0.0, 0.0);
      let mut num_matriarchs = 0;

      for (t, _matriarch, w) in (&transforms, &matriarchs, &walkers).join() {
        num_matriarchs += 1;
        matriarch_translation += t.translation;
        match w.direction {
          Direction::Right => matriarch_translation.x += camera_config.offset.x,
          Direction::Left => matriarch_translation.x -= camera_config.offset.x,
        }
        matriarch_translation.y += camera_config.offset.y;
      }

      if num_matriarchs > 0 {
        matriarch_translation /= num_matriarchs as f32;
        Some((matriarch_translation, false))
      } else {
        None
      }
    };

    if let Some((final_target, lerp_z)) = final_target {
      for (t, _tag) in (&mut transforms, &fly_tags).join() {
        t.translation.x += (final_target.x - t.translation.x) * delta * camera_config.convergence_speed;
        t.translation.y += (final_target.y - t.translation.y) * delta * camera_config.convergence_speed;

        if lerp_z {
          t.translation.z += (final_target.z - t.translation.z) * delta * camera_config.convergence_speed;
        } else {
          t.translation.z += zoom * delta * camera_config.zoom_speed;
          t.translation.z = t.translation.z
            .min(camera_config.z_max)
            .max(camera_config.z_min);
        }
      }
    }
  }
}