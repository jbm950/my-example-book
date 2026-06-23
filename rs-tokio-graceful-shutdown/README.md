Simple example to showcase graceful shutdown of an async tokio app using a
cancellation token to signal shutdown and a task tracker to ensure the tasks
have completed before the program ends.
