use soph::prelude::*;
use soph_schedule::{support::Task, Schedule};

struct App;

impl ApplicationTrait for App {
    type Service = Schedule;

    fn with_schedule() -> impl ServiceTrait {
        Schedule::new().register(Task::background("0/3 * * * * *", || {
            Box::pin(async {
                println!("[background]task: 0/3 # # # # #");

                Ok(())
            })
        }))
    }
}

fn main() -> Result<()> {
    run::<App>()
}
