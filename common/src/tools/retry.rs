
/// macro function for retry n times with m sleep duration for async function.
/// Args: func_name: String, func: any async func, count: usize, interval: f32
#[macro_export]
macro_rules! retry {
    ($func_name:expr, $func:expr, $count:expr, $interval:expr) => {{
        let mut retries = 0;
        let result = loop {
            let result = $func;
            if result.is_ok() {
                break result;
            } else if retries >= $count {
                break result;
            } else {
                retries += 1;
                tracing::error!(
                    "{}/{} retrying {} function after {} seconds...", 
                    retries, 
                    $count, 
                    $func_name, 
                    $interval
                );
                let retry_in = std::time::Duration::from_secs_f32($interval);
                tokio::time::sleep(retry_in).await;
            }
        };
        result
    }};
    ($func_name:expr, $func:expr, $count:expr) => {
        retry!($func_name, $func, $count, 0.4)
    };
    ($func_name:expr, $func:expr) => {
        retry!($func_name, $func, 10, 0.4)
    };
}

/// macro function for increasing retry n times with min sleep duration and step duration between increasing tries for async function.
/// Maxim sleep time is ~20s.
/// Args: func_name: String, func: any async func, count: usize, sleep_min: f32, step: f32
#[macro_export]
macro_rules! increasing_retry {
    ($func_name:expr, $func:expr, $count:expr, $sleep_min:expr, $step:expr) => {{
        use rand::Rng;
        let mut retries = 0;
        let factor = 1.5;
        let sleep_min = $sleep_min;
        let step = $step;
        let limit_max = std::time::Duration::from_secs(20);
        let limit_min = std::time::Duration::from_secs_f32(sleep_min);
        let rand_factor = std::time::Duration::from_secs_f32(step);
        let mut rng = rand::rngs::OsRng;
        let mut retry_in = rand_factor.clone();
        let mut jitter = || rng.gen_range(std::time::Duration::ZERO..rand_factor);
        let result = loop {
            let result = $func;
            if result.is_ok() {
                break result;
            } else if retries >= $count {
                break result;
            } else {
                retries += 1;
                retry_in = retry_in.mul_f32(factor).min(limit_max).max(limit_min) + jitter();
                tracing::error!(
                    "{}/{} retrying {} function after {:?} seconds...", 
                    retries, 
                    $count, 
                    $func_name, 
                    retry_in
                );
                tokio::time::sleep(retry_in).await;
            }
        };
        result
    }};
    ($func_name:expr, $func:expr, $count:expr, $sleep_min:expr) => {
        increasing_retry!($func_name, $func, $count, $sleep_min, 0.3)
    };
    ($func_name:expr, $func:expr, $count:expr) => {
        increasing_retry!($func_name, $func, $count, 0.8, 0.3)
    };
    ($func_name:expr, $func:expr) => {
        increasing_retry!($func_name, $func, 10, 0.8, 0.3)
    };
}

/// macro function for infinite retry.
/// Args: func_name: String, func: any async func, interval: f32.
#[macro_export]
macro_rules! infinite_retry {
    ($func_name:expr, $func:expr, $interval:expr) => {{
        let result = loop {
            let result = $func;
            if result.is_ok() {
                break result;
            } else {
                tracing::error!(
                    "infinitely retrying {} function after {} seconds...", 
                    $func_name, 
                    $interval
                );
                let retry_in = std::time::Duration::from_secs_f32($interval);
                tokio::time::sleep(retry_in).await
            }
        };
        result
    }};
    ($func_name:expr, $func:expr) => {
        infinite_retry!($func_name, $func)
    }
}

/// macro function like retry but using thread::sleep().
/// Args: func_name: String, func: any func, interval: f32. 
#[macro_export]
macro_rules! block_retry {
    ($func_name:expr, $func:expr, $count:expr, $interval:expr) => {{
        let mut retries = 0;
        let result = loop {
            let result = $func;
            if result.is_ok() {
                break result;
            } else if retries >= $count {
                break result;
            } else {
                retries += 1;
                tracing::error!(
                    "{}/{} retrying {} function after {} seconds...", 
                    retries, 
                    $count, 
                    $func_name, 
                    $interval
                );
                let retry_in = std::time::Duration::from_secs_f32($interval);
                std::thread::sleep(retry_in);
            }
        };
        result
    }};
    ($func_name:expr, $func:expr, $count:expr) => {
        block_retry!($func_name, $func, $count, 0.4)
    };
    ($func_name:expr, $func:expr) => {
        block_retry!($func_name, $func, 10, 0.4)
    };
}

pub use retry;
pub use increasing_retry;
pub use infinite_retry;
pub use block_retry;
