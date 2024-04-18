#![allow(dead_code,unused)]
use tokio::signal;
use tracing;

/// macro function for retry n times with m sleep duration of async function.
#[macro_export]
macro_rules! retry_on_err {
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
                tracing::warn!(
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
        retry_on_err!($func_name, $func, $count, 0.4)
    };
    ($func_name:expr, $func:expr) => {
        retry_on_err!($func_name, $func, 10, 0.4)
    };
}

#[macro_export]
macro_rules! retry_inc_on_err {
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
                tracing::warn!(
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
        retry_inc_on_err!($func_name, $func, $count, $sleep_min, 0.3)
    };
    ($func_name:expr, $func:expr, $count:expr) => {
        retry_inc_on_err!($func_name, $func, $count, 0.8, 0.3)
    };
    ($func_name:expr, $func:expr) => {
        retry_inc_on_err!($func_name, $func, 10, 0.8, 0.3)
    };
}

pub use retry_on_err;
pub use retry_inc_on_err;


pub async fn graceful_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("shutting down...")
        },
        _ = terminate => {},
    }
}

// pub async fn retry() {
//     tokio::spawn(async move {
        
//     })
// }