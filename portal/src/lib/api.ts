import type {
  ServiceInfo,
  ServiceDefinition,
  HealthInfo,
  ArtifactInfo,
  ApiResponse,
} from "./types";

const BASE = "http://localhost:8000/api";
async function request<T>(path: string, options?: RequestInit): Promise<T> {
  const url = `${BASE}${path}`;
  const res = await fetch(url, options);
  const data = await res.json();
  if (!res.ok) {
    throw new Error(data.message || `Request failed: ${res.status}`);
  }
  return data as T;
}

export async function getHealth(): Promise<HealthInfo> {
  return request("/health");
}

export async function getServices(): Promise<ServiceInfo[]> {
  return request("/services");
}

export async function getRegistry(): Promise<ServiceDefinition[]> {
  return request("/registry");
}

export async function createService(
  def: Partial<ServiceDefinition>,
): Promise<ApiResponse> {
  return request("/services", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(def),
  });
}

export async function startService(id: string): Promise<ApiResponse> {
  return request(`/services/${id}/start`, { method: "POST" });
}

export async function stopService(id: string): Promise<ApiResponse> {
  return request(`/services/${id}/stop`, { method: "POST" });
}

export async function restartService(id: string): Promise<ApiResponse> {
  return request(`/services/${id}/restart`, { method: "POST" });
}

export async function deleteService(id: string): Promise<ApiResponse> {
  return request(`/services/${id}`, { method: "DELETE" });
}

export async function uploadArtifact(
  id: string,
  file: File,
  version: string,
): Promise<ApiResponse> {
  const form = new FormData();
  form.append("version", version);
  form.append("file", file);
  return request(`/services/${id}/artifact/upload`, {
    method: "POST",
    body: form,
  });
}

export async function installGithubArtifact(
  id: string,
  repo: string,
  version: string,
  asset: string,
): Promise<ApiResponse> {
  return request(`/services/${id}/artifact/github`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ repo, version, asset }),
  });
}

export async function getArtifactInfo(id: string): Promise<ArtifactInfo> {
  return request(`/services/${id}/artifact`);
}
