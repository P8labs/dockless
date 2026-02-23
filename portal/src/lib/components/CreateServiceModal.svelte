<script lang="ts">
  import * as api from "$lib/api";
  import { store } from "$lib/services.svelte";
  import { toasts } from "$lib/toasts.svelte";

  let { open, onclose }: { open: boolean; onclose: () => void } = $props();

  let id = $state("");
  let name = $state("");
  let binaryPath = $state("");
  let autoRestart = $state(true);
  let restartLimit = $state("");
  let args = $state("");
  let envRows = $state<{ key: string; value: string }[]>([
    { key: "", value: "" },
  ]);
  let loading = $state(false);

  function addEnvRow() {
    envRows = [...envRows, { key: "", value: "" }];
  }

  function removeEnvRow(index: number) {
    envRows = envRows.filter((_, i) => i !== index);
  }

  function reset() {
    id = "";
    name = "";
    binaryPath = "";
    autoRestart = true;
    restartLimit = "";
    args = "";
    envRows = [{ key: "", value: "" }];
  }

  async function handleCreate() {
    loading = true;
    try {
      const env: Record<string, string> = {};
      for (const row of envRows) {
        if (row.key.trim()) {
          env[row.key.trim()] = row.value;
        }
      }

      const result = await api.createService({
        id: id.trim(),
        name: name.trim() || id.trim(),
        binary_path: binaryPath.trim(),
        args: args.trim() ? args.split(",").map((a) => a.trim()) : [],
        env,
        auto_restart: autoRestart,
        restart_limit: restartLimit.trim()
          ? parseInt(restartLimit.trim(), 10)
          : null,
      });

      toasts.add(result.message, result.status ? "success" : "error");
      if (result.status) {
        reset();
        onclose();
        await store.refresh();
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to create service";
      toasts.add(msg, "error");
    } finally {
      loading = false;
    }
  }
</script>

{#if open}
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_interactive_supports_focus -->
  <div
    class="fixed inset-0 z-40 flex items-center justify-center p-4 bg-black/50 backdrop-blur-sm"
    onclick={onclose}
    onkeydown={(e) => {
      if (e.key === "Escape") onclose();
    }}
    role="dialog"
    aria-modal="true"
    aria-label="Create Service"
  >
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div
      class="bg-white rounded-xl border border-gray-200 p-6 max-w-lg w-full max-h-[90vh] overflow-y-auto"
      onclick={(e) => e.stopPropagation()}
      onkeydown={() => {}}
    >
      <div class="flex items-center justify-between mb-6">
        <h2 class="text-xl font-semibold text-gray-900">Create Service</h2>
        <button
          class="text-gray-400 hover:text-gray-600 transition-colors"
          aria-label="Close modal"
          onclick={() => {
            reset();
            onclose();
          }}
        >
          <svg
            class="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>

      <div class="space-y-4">
        <!-- ID -->
        <div>
          <label
            for="svc-id"
            class="block text-sm font-medium text-gray-700 mb-1.5">ID</label
          >
          <input
            id="svc-id"
            type="text"
            bind:value={id}
            placeholder="my-service"
            class="w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
          />
        </div>

        <!-- Name -->
        <div>
          <label
            for="svc-name"
            class="block text-sm font-medium text-gray-700 mb-1.5">Name</label
          >
          <input
            id="svc-name"
            type="text"
            bind:value={name}
            placeholder="My Service"
            class="w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
          />
        </div>

        <!-- Binary Path -->
        <div>
          <label
            for="svc-path"
            class="block text-sm font-medium text-gray-700 mb-1.5"
            >Binary Path</label
          >
          <input
            id="svc-path"
            type="text"
            bind:value={binaryPath}
            placeholder="/path/to/binary"
            class="w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm font-mono focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
          />
        </div>

        <!-- Auto Restart -->
        <div class="flex items-center justify-between py-2">
          <span class="text-sm font-medium text-gray-700">Auto Restart</span>
          <!-- svelte-ignore a11y_consider_explicit_label -->
          <button
            type="button"
            class="relative inline-flex h-6 w-11 items-center rounded-full transition-colors {autoRestart
              ? 'bg-black'
              : 'bg-gray-300'}"
            aria-label="Toggle auto restart"
            onclick={() => {
              autoRestart = !autoRestart;
            }}
          >
            <span
              class="inline-block h-4 w-4 rounded-full bg-white transition-transform {autoRestart
                ? 'translate-x-6'
                : 'translate-x-1'}"
            ></span>
          </button>
        </div>

        <!-- Restart Limit -->
        <div>
          <label
            for="svc-limit"
            class="block text-sm font-medium text-gray-700 mb-1.5"
            >Restart Limit</label
          >
          <input
            id="svc-limit"
            type="text"
            inputmode="numeric"
            bind:value={restartLimit}
            placeholder="No limit"
            class="w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
          />
        </div>

        <!-- Args -->
        <div>
          <label
            for="svc-args"
            class="block text-sm font-medium text-gray-700 mb-1.5"
            >Arguments <span class="text-gray-400 font-normal"
              >(comma-separated)</span
            ></label
          >
          <input
            id="svc-args"
            type="text"
            bind:value={args}
            placeholder="--port, 3000, --verbose"
            class="w-full border border-gray-200 rounded-lg px-3 py-2.5 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
          />
        </div>

        <!-- Environment Variables -->
        <div>
          <div class="flex items-center justify-between mb-1.5">
            <span class="text-sm font-medium text-gray-700"
              >Environment Variables</span
            >
            <button
              type="button"
              class="text-xs font-medium text-gray-600 hover:text-black transition-colors"
              onclick={addEnvRow}
            >
              + Add Variable
            </button>
          </div>
          <div class="space-y-2">
            {#each envRows as row, i}
              <div class="flex items-center gap-2">
                <input
                  type="text"
                  bind:value={row.key}
                  placeholder="KEY"
                  class="flex-1 border border-gray-200 rounded-lg px-3 py-2 text-sm font-mono focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                />
                <input
                  type="text"
                  bind:value={row.value}
                  placeholder="value"
                  class="flex-1 border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                />
                {#if envRows.length > 1}
                  <button
                    type="button"
                    class="text-gray-400 hover:text-red-500 transition-colors p-1"
                    aria-label="Remove environment variable"
                    onclick={() => removeEnvRow(i)}
                  >
                    <svg
                      class="w-4 h-4"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M6 18L18 6M6 6l12 12"
                      />
                    </svg>
                  </button>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div
        class="flex items-center justify-end gap-3 mt-6 pt-4 border-t border-gray-100"
      >
        <button
          class="rounded-lg px-4 py-2 text-sm font-medium text-gray-600 bg-gray-100 hover:bg-gray-200 transition-all duration-150"
          onclick={() => {
            reset();
            onclose();
          }}
        >
          Cancel
        </button>
        <button
          class="rounded-lg px-4 py-2 text-sm font-medium text-white bg-black hover:bg-gray-800 active:scale-95 transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
          disabled={!id.trim() || !binaryPath.trim() || loading}
          onclick={handleCreate}
        >
          {#if loading}
            <span
              class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"
            ></span>
          {/if}
          Create Service
        </button>
      </div>
    </div>
  </div>
{/if}
