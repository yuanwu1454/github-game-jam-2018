use amethyst::{
  ecs::prelude::*,
  shrev::ReaderId,
  core::{
    transform::Transform,
    cgmath::{
      Quaternion,
      Euler,
      Deg,
      Vector2,
    },
  },
  renderer::Shape,
};

use ::{
  components::{
    Matriarch,
    Walker,
    Shape as ShapeComponent,
    LaunchArea,
  },
  config::PhysicsConfig,
  resources::{
    Command,
    CommandChannel,
    PhysicsWorld,
  },
};

#[derive(Default)]
pub struct DropLift {
  command_reader: Option<ReaderId<Command>>,
}

impl<'s> System<'s> for DropLift {
  type SystemData = (
    Entities<'s>,
    Read<'s, CommandChannel>,
    ReadStorage<'s, Matriarch>,
    ReadStorage<'s, Transform>,
    ReadStorage<'s, Walker>,
    Write<'s, PhysicsWorld>,
    Read<'s, PhysicsConfig>,
    Read<'s, LazyUpdate>,
  );

  fn setup(&mut self, res: &mut Resources) {
    Self::SystemData::setup(res);
    self.command_reader = Some(res.fetch_mut::<CommandChannel>().register_reader());
  }

  fn run(&mut self, (entities, commands, matriarchs, transforms, walkers, mut physics_world, physics_config, updater): Self::SystemData) {
    let mut drop_lift = false;
    for command in commands.read(self.command_reader.as_mut().unwrap()) {
      match command {
        Command::DropLift => drop_lift = true,
        _ => {},
      }
    }

    if drop_lift {
      for (e, _, t, w) in (&entities, &matriarchs, &transforms, &walkers).join() {
        if entities.is_alive(e) {
          debug!("Dropping lift on Matriarch {:?}", e);

          let la = LaunchArea {
            direction: w.direction,
          };

          let shape = ShapeComponent {
            shape: Shape::Cone(10),
            scale: (
              0.1,
              physics_config.lift_width * 0.5,
              physics_config.lift_height * 0.5,
            ),
          };

          let mut transform = t.clone();
          transform.rotation = Quaternion::from(Euler { x: Deg(0.0), y: Deg(0.0), z: Deg(90.0) })
                             * Quaternion::from(Euler { x: Deg(0.0), y: Deg(90.0), z: Deg(0.0) });

          let sensor = physics_world.create_ground_box_sensor(
            &Vector2::new(transform.translation.x, transform.translation.y), //Pos
            &Vector2::new(physics_config.change_direction_width * 0.5, physics_config.change_direction_height * 0.5), //Size
            90.0);

          updater
            .create_entity(&entities)
            .with(la)
            .with(shape)
            .with(transform)
            .with(sensor)
            .build();
        }
      }
    }
  }
}