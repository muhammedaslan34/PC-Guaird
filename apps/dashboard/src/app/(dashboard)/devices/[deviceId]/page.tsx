import { DevicePollingProvider } from "@/components/devices/device-polling-provider";
import {
  getDevice,
  getDeviceAuditLogs,
  getDeviceCommands,
} from "@/lib/dashboard-api";

type DeviceDetailPageProps = {
  params: Promise<{ deviceId: string }>;
};

export default async function DeviceDetailPage({
  params,
}: DeviceDetailPageProps) {
  const { deviceId } = await params;
  const numericDeviceId = Number(deviceId);

  const [device, commands, auditLogs] = await Promise.all([
    getDevice(numericDeviceId),
    getDeviceCommands(numericDeviceId),
    getDeviceAuditLogs(numericDeviceId),
  ]);

  return (
    <DevicePollingProvider
      deviceId={numericDeviceId}
      initialDevice={device}
      initialCommands={commands}
      initialAuditLogs={auditLogs}
    />
  );
}
