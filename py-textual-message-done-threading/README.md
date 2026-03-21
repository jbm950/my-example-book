Same as the done-asyncio example except this time using threading.Thread to
move a task to concurrency. That way I can work with long tasks that didn't
implement asyncio.

Note there was some extra logic needed to make sure that all the started
threads complete before the app closes otherwise the `self.notify()` throws an
error because the app closes before that line is called.
