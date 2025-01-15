use app::tasks;
use soph::prelude::*;
use soph_schedule::Schedule;

pub struct App;

impl ApplicationTrait for App {
    type Service = Schedule;

    fn with_schedule() -> impl ServiceTrait {
        Schedule::new().register(tasks::every_three_seconds())
    }
}
