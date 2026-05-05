use child_agent::{app, service::ServiceRuntime};

fn main() {
    let runtime = ServiceRuntime::new();
    println!(
        "{} running as Windows Service '{}'",
        app::APP_NAME,
        app::service_name()
    );
    println!("status={}", runtime.snapshot().service_status);
}
