<script lang="ts">
  import { toasts } from "$lib/toasts.svelte";

  function getToastStyles(type: string): { bg: string; icon: string } {
    switch (type) {
      case "success":
        return {
          bg: "bg-green-50 text-green-800 border-green-200",
          icon: "text-green-500",
        };
      case "error":
        return {
          bg: "bg-red-50 text-red-800 border-red-200",
          icon: "text-red-500",
        };
      case "warning":
        return {
          bg: "bg-yellow-50 text-yellow-800 border-yellow-200",
          icon: "text-yellow-500",
        };
      default:
        return {
          bg: "bg-gray-50 text-gray-800 border-gray-200",
          icon: "text-gray-500",
        };
    }
  }

  function getIcon(type: string): string {
    switch (type) {
      case "success":
        return "M5 13l4 4L19 7";
      case "error":
        return "M6 18L18 6M6 6l12 12";
      case "warning":
        return "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z";
      default:
        return "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
    }
  }
</script>

<div class="fixed bottom-4 right-4 z-50 space-y-2">
  {#each toasts.items as toast (toast.id)}
    <div
      class="rounded-lg border px-4 py-3 text-sm min-w-70 max-w-sm flex items-start gap-3 backdrop-blur-sm {getToastStyles(
        toast.type,
      ).bg}"
    >
      <svg
        class="w-5 h-5 shrink-0 {getToastStyles(toast.type).icon}"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d={getIcon(toast.type)}
        />
      </svg>
      <span class="flex-1 font-medium">{toast.message}</span>
    </div>
  {/each}
</div>
