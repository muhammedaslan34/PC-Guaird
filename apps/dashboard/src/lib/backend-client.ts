type BackendRequestOptions = Omit<RequestInit, "body"> & {
  body?: unknown;
  token?: string | null;
};

export function getBackendUrl(): string {
  const backendUrl = process.env.BACKEND_URL;

  if (!backendUrl) {
    throw new Error("BACKEND_URL is not configured.");
  }

  return backendUrl.replace(/\/$/, "");
}

export async function backendFetch(
  path: string,
  { body, headers, token, ...init }: BackendRequestOptions = {},
): Promise<Response> {
  const requestHeaders = new Headers(headers);

  requestHeaders.set("Accept", "application/json");

  if (body !== undefined) {
    requestHeaders.set("Content-Type", "application/json");
  }

  if (token) {
    requestHeaders.set("Authorization", `Bearer ${token}`);
  }

  return fetch(`${getBackendUrl()}${path}`, {
    ...init,
    headers: requestHeaders,
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}
