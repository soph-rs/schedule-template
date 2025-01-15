use soph_schedule::support::Task;

pub fn every_three_seconds() -> Task {
    Task::background("0/3 * * * * *", || {
        Box::pin(async {
            println!("[background]task: 0/3 # # # # #");

            Ok(())
        })
    })
}
