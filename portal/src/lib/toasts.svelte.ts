export interface Toast {
  id: number;
  message: string;
  type: "success" | "error" | "warning";
}

class ToastStore {
  items = $state<Toast[]>([]);
  private nextId = 0;

  add(message: string, type: Toast["type"] = "success") {
    const id = this.nextId++;
    this.items = [...this.items, { id, message, type }];
    setTimeout(() => this.remove(id), 3000);
  }

  remove(id: number) {
    this.items = this.items.filter((t) => t.id !== id);
  }
}

export const toasts = new ToastStore();
