use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  core::{
    cgmath::Vector2,
    transform::Transform,
  },
};

use nphysics2d::math::Velocity;
use nalgebra::Vector2 as naVector2;

use ::{
  config::PhysicsConfig,
  components::{
    Matriarch,
    Walker,
    ConstantVelocity,
    Age,
    Direction,
    Color,
    Shape,
  },
  resources::{
    Command,
    CommandChannel,
    PhysicsWorld,
  },
};


///Drops a ram on the matriarch. It's made to appear like the ram is the matriarch but it's actually a different entity.
#[derive(Default)]
pub struct DropRam {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for DropRam {
  type SystemData = (
    Entities<'s>,
    Read<'s, CommandChannel>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Color>,
    ReadStorage<'s, Shape>,
    ReadStorage<'s, Walker>,
    Write<'s, PhysicsWorld>,
    ReadStorage<'s, Transform>,
    Read<'s, PhysicsConfig>,
    Read<'s, LazyUpdate>,
    ReadStorage<'s, Age>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, commands, matriarchs, colors, shapes, walkers, mut physics_world, transforms, physics_config, updater, ages): Self::SystemData) {
    let mut drop_ram = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::DropRam => drop_ram = true,
        _ => {},
      }
    }

    if drop_ram {
      for (e, m, t, w, a) in (&entities, &matriarchs, &transforms, &walkers, &ages).join() {
        if entities.is_alive(e) {
          //This test is to discard commands that were likely intended for a matriarch that just died
          if (a.seconds - m.age_when_promoted) < physics_config.matriarch_grace_period {
            continue;
          }
          debug!("Dropping ram on Matriarch {:?}", e);

          let collider = physics_world.create_rigid_body_with_box_collider_with_density(
            &Vector2::new(t.translation.x, t.translation.y),
            &Vector2::new(20.0, 20.0),
            0.0,
            physics_config.ram_density);

          let age = Age {
            seconds: 0.0,
            max: Some(physics_config.ram_life),
          };

          let dir = match w.direction {
            Direction::Left => -1.0,
            Direction::Right => 1.0,
          };

          let cv = ConstantVelocity {
            velocity: Velocity::new(
              naVector2::new(
                physics_config.ram_velocity.x * dir,
                physics_config.ram_velocity.y,
              ),
              0.0,
            ),
          };

          let mut builder = updater
            .create_entity(&entities)
            .with(collider)
            .with(age)
            .with(cv);

          if let Some(color) = colors.get(e) {
            builder = builder.with(*color);
          }

          if let Some(shape) = shapes.get(e) {
            builder = builder.with(shape.clone());
          }

          builder.build();
        }
      }
    }
  }
}