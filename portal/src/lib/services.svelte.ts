import * as api from "./api";
import type {
  ServiceInfo,
  ServiceDefinition,
  HealthInfo,
  ServiceView,
} from "./types";

class ServiceStore {
  services = $state<ServiceInfo[]>([]);
  registry = $state<ServiceDefinition[]>([]);
  health = $state<HealthInfo | null>(null);
  loading = $state(true);
  private interval: ReturnType<typeof setInterval> | null = null;

  get merged(): ServiceView[] {
    return this.registry.map((def) => {
      const info = this.services.find((s) => s.id === def.id);
      return { ...def, state: info?.state ?? "Stopped" };
    });
  }

  async refresh() {
    try {
      const [services, registry, health] = await Promise.all([
        api.getServices(),
        api.getRegistry(),
        api.getHealth(),
      ]);
      this.services = services;
      this.registry = registry;
      this.health = health;
      this.loading = false;
    } catch (e) {
      console.error("Failed to refresh:", e);
      this.loading = false;
    }
  }

  startPolling(ms = 3000) {
    this.refresh();
    this.interval = setInterval(() => this.refresh(), ms);
  }

  stopPolling() {
    if (this.interval) {
      clearInterval(this.interval);
      this.interval = null;
    }
  }
}

export const store = new ServiceStore();
