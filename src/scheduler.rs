/*
amFOSS Daemon: A discord bot for the amFOSS Discord server.
Copyright (C) 2024 amFOSS

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::tasks::{get_tasks, Task};

use serenity::client::Context as SerenityContext;
use tokio::spawn;
use tracing::{debug, error, trace};

/// Spawns a sleepy thread for each [`Task`].
pub async fn run_scheduler(ctx: SerenityContext) {
    trace!("Running scheduler");
    let tasks = get_tasks();

    for task in tasks {
        debug!("Spawing task {}", task.name());
        spawn(schedule_task(ctx.clone(), task));
    }
}

/// Runs the function [`Task::run`] and goes back to sleep until it's time to run again.
async fn schedule_task(ctx: SerenityContext, task: Box<dyn Task>) {
    loop {
        let next_run_in = task.run_in();
        debug!("Task {}: Next run in {:?}", task.name(), next_run_in);
        tokio::time::sleep(next_run_in).await;

        debug!("Running task {}", task.name());
        tokio::time::sleep(next_run_in).await;
        if let Err(e) = task.run(ctx.clone()).await {
            error!("Could not run task {}, error {}", task.name(), e);
        }
    }
}
