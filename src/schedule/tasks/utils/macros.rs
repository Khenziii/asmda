#[macro_export]
macro_rules! init_new_task {
    ($config:expr) => {
        pub fn get_task() -> Task {
            Task::new(
                Duration::from_secs($config.run_interval_seconds),
                Mutex::new($config.callback),
                $config.app_name,
                $config.is_enabled,
            )
        }
    };

    ($config:expr, $method_name:ident) => {
        pub fn $method_name() -> Task {
            Task::new(
                Duration::from_secs($config.run_interval_seconds),
                Mutex::new($config.callback),
                $config.app_name,
                $config.is_enabled,
            )
        }
    };
}

#[macro_export]
macro_rules! task_callback {
    ($func:path) => {
        Box::new(|| Box::pin($func()))
    };
}
