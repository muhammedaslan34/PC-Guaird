import { redirect } from "next/navigation";
import { getStoredAuthToken } from "@/lib/auth";

export default async function Home() {
  const token = await getStoredAuthToken();

  redirect(token ? "/devices" : "/login");
}
