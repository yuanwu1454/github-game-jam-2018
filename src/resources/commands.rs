use amethyst::shrev::EventChannel;

#[derive(Debug)]
pub enum Command {
  KillMatriarch,
}

pub type CommandChannel = EventChannel<Command>;