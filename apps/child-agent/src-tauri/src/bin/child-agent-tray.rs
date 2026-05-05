use child_agent::{app, ipc::TrayPipe, service::ServiceRuntime, tray::TrayStatusReader};

fn main() {
    let runtime = ServiceRuntime::new();
    let pipe = TrayPipe::connect(runtime);
    let mut reader = TrayStatusReader::new(pipe);
    let status = reader.read();

    println!(
        "{} running as tray companion '{}'",
        app::APP_NAME,
        app::tray_name()
    );
    println!(
        "connection={} pairing_code={}",
        status.connection_display(),
        status.pairing_code_display()
    );
}
