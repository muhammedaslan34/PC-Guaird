mod app;
mod ipc;
mod service;
mod tray;

#[cfg(test)]
mod tests;

fn main() {
    let runtime = service::ServiceRuntime::new();
    println!(
        "{} service scaffold running as {} with tray {}",
        app::APP_NAME,
        app::service_name(),
        app::tray_name()
    );
    println!("{}", runtime.snapshot().service_status);
}
