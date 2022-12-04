use specs::{prelude::*};
use crate::delta_time;

pub struct DeltaTimeSys {}

impl<'a> System<'a> for DeltaTimeSys {
    type SystemData = Write<'a, delta_time::DeltaTime>;

    fn run(&mut self, mut dt: Self::SystemData) {
        log::trace!("running DeltaTimeSys system");

        dt.tick();
        log::warn!("delta time updated: {:?}", dt.dt);
    }
}