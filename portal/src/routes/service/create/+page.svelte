<script lang="ts">
  import { goto } from "$app/navigation";
  import * as api from "$lib/api";
  import { toaster } from "$lib/components/Toast.svelte";
  import { store } from "$lib/services.svelte";
  import { ArrowLeft } from "lucide-svelte";

  let name = $state("");
  let loading = $state(false);

  async function handleCreate() {
    if (!name.trim()) return;

    loading = true;
    try {
      const result = await api.initService(name.trim());

      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });

      if (result.status && result.id) {
        await store.refresh();
        // Navigate to service configuration page
        goto(`/service/${result.id}`);
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to create service";
      toaster.create({ title: msg, type: "error" });
    } finally {
      loading = false;
    }
  }
</script>

<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  <button
    class="inline-flex items-center gap-2 text-sm mb-6 transition-colors btn"
    onclick={() => goto("/")}
  >
    <ArrowLeft class="w-4 h-4" />
    Back to Services
  </button>

  <div class="max-w-xl mx-auto">
    <div class="card bg-surface-50-950/60 p-8">
      <div class="mb-6">
        <h2 class="text-2xl font-semibold mb-2">Create Service</h2>
        <p class="text-sm opacity-70">
          Create a new service. You'll be able to configure it with environment
          variables, upload binaries, and manage versions after creation.
        </p>
      </div>

      <div class="space-y-6">
        <div>
          <label class="label">
            <span class="label-text">Service Name</span>
            <input
              id="svc-name"
              class="input"
              type="text"
              bind:value={name}
              placeholder="my-service"
              onkeydown={(e) => {
                if (e.key === "Enter" && name.trim() && !loading) {
                  handleCreate();
                }
              }}
            />
          </label>
          <p class="text-xs opacity-60 mt-1.5">
            A service ID will be automatically generated from the name
          </p>
        </div>

        <div class="flex items-center gap-3 pt-4">
          <button
            type="button"
            class="btn preset-outlined flex-1"
            disabled={!name.trim() || loading}
            onclick={handleCreate}
          >
            {#if loading}
              <span
                class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
              ></span>
            {/if}
            Create & Configure
          </button>
          <button
            type="button"
            class="btn preset-tonal"
            onclick={() => goto("/")}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
</main>
