<script lang="ts">
  import { goto } from "$app/navigation";
  import * as api from "$lib/api";

  import { store } from "$lib/services.svelte";
  import { toasts } from "$lib/toasts.svelte";
  import { X } from "lucide-svelte";

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

<main class="flex items-center justify-center min-h-screen max-w-xl mx-auto">
  <div class="w-full p-6 card bg-surface-50-950/60">
    <div class="flex items-start justify-between mb-6 flex-col">
      <h2 class="text-xl font-semibold">Create Service</h2>
      <span>fill the details and register a new service.</span>
    </div>

    <div class="space-y-4">
      <div>
        <label class="label">
          <span class="label-text">Service ID</span>
          <input
            id="svc-id"
            class="input"
            type="text"
            bind:value={id}
            placeholder="my-service"
          />
        </label>
      </div>

      <div>
        <label class="label">
          <span class="label-text">Name</span>
          <input
            id="svc-name"
            class="input"
            type="text"
            bind:value={name}
            placeholder="My Service"
          />
        </label>
      </div>

      <div>
        <label class="label">
          <span class="label-text">Binary Path</span>
          <input
            id="svc-path"
            class="input"
            type="text"
            bind:value={binaryPath}
            placeholder="/path/to/binary"
          />
        </label>
      </div>

      <div>
        <label class="flex items-center space-x-2">
          <input
            class="checkbox"
            type="checkbox"
            aria-label="Toggle auto restart"
            bind:checked={autoRestart}
          />
          <p>Auto Restart</p>
        </label>
      </div>

      <div>
        <label class="label">
          <span class="label-text">Restart Limit</span>
          <input
            id="svc-limit"
            class="input"
            type="number"
            inputmode="numeric"
            bind:value={restartLimit}
            placeholder="No limit"
          />
        </label>
      </div>

      <div>
        <label class="label">
          <span class="label-text"
            >Arguments <span class="text-gray-400 font-normal"
              >(comma-separated)</span
            ></span
          >
          <input
            id="svc-args"
            class="input"
            type="text"
            bind:value={args}
            placeholder="--port, 3000, --verbose"
          />
        </label>
      </div>

      <div>
        <div class="flex items-center justify-between mb-1.5">
          <span class="label-text">Environment Variables</span>
          <button type="button" class="label-text btn-sm" onclick={addEnvRow}>
            + Add Variable
          </button>
        </div>
        <div class="space-y-2">
          {#each envRows as row, i}
            <div class="flex items-center gap-2">
              <input
                class="input"
                type="text"
                bind:value={row.key}
                placeholder="KEY"
              />
              <input
                class="input"
                type="text"
                bind:value={row.value}
                placeholder="value"
              />

              {#if envRows.length > 1}
                <button
                  type="button"
                  class="btn label-text hover:text-red-500/70 transition-colors p-1"
                  aria-label="Remove environment variable"
                  onclick={() => removeEnvRow(i)}
                >
                  <X />
                </button>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    </div>

    <div class="flex items-center justify-end gap-3 mt-6 pt-4">
      <button type="button" class="btn preset-filled" onclick={() => goto("/")}>
        Back
      </button>
      <button
        type="button"
        class="btn preset-outlined"
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
</main>
