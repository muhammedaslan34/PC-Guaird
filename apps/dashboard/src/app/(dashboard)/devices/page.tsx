import { DeviceList } from "@/components/devices/device-list";
import { getDevices } from "@/lib/dashboard-api";

export default async function DevicesPage() {
  const devices = await getDevices();

  return <DeviceList devices={devices} />;
}
