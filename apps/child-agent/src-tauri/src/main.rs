use child_agent::{app, service::ServiceRuntime};

fn main() {
    let runtime = ServiceRuntime::new();
    println!(
        "{} service scaffold running as {} with tray {}",
        app::APP_NAME,
        app::service_name(),
        app::tray_name()
    );
    println!("{}", runtime.snapshot().service_status);
}
