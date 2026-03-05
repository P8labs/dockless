<script lang="ts">
  import type { ServiceDefinition } from "$lib/types";
  import { Plus, Trash2 } from "lucide-svelte";

  let {
    serviceDetail,
    onSave,
    saving,
  }: {
    serviceDetail: ServiceDefinition | null;
    onSave: (config: {
      auto_restart: boolean;
      restart_limit: number | null;
      env: Record<string, string>;
      args: string[];
      linux_capabilities: string[];
    }) => void;
    saving: boolean;
  } = $props();

  let autoRestart = $state(true);
  let restartLimit = $state<string>("");
  let envVars = $state<Array<{ key: string; value: string }>>([]);
  let args = $state<string[]>([]);
  let newArg = $state("");
  let capabilities = $state<string[]>([]);
  let capInput = $state("");

  $effect(() => {
    if (serviceDetail) {
      autoRestart = serviceDetail.auto_restart;
      restartLimit = serviceDetail.restart_limit?.toString() ?? "";
      envVars = Object.entries(serviceDetail.env).map(([key, value]) => ({
        key,
        value,
      }));
      args = [...serviceDetail.args];
      capabilities = [...(serviceDetail.linux_capabilities ?? [])];
    }
  });

  function addCapability() {
    const cap = capInput.trim();
    if (cap && !capabilities.includes(cap)) {
      capabilities = [...capabilities, cap];
    }
    capInput = "";
  }

  function removeCapability(i: number) {
    capabilities = capabilities.filter((_, idx) => idx !== i);
  }

  function addEnvVar() {
    envVars = [...envVars, { key: "", value: "" }];
  }

  function removeEnvVar(index: number) {
    envVars = envVars.filter((_, i) => i !== index);
  }

  function addArg() {
    if (newArg.trim()) {
      args = [...args, newArg.trim()];
      newArg = "";
    }
  }

  function removeArg(index: number) {
    args = args.filter((_, i) => i !== index);
  }

  function handleSave() {
    try {
      const env: Record<string, string> = {};
      const envVarPattern = /^[A-Za-z_][A-Za-z0-9_]*$/;

      for (const { key, value } of envVars) {
        const trimmedKey = (key || "").trim();
        if (trimmedKey) {
          if (!envVarPattern.test(trimmedKey)) {
            throw new Error(
              `Invalid environment variable name: "${trimmedKey}". ` +
                "Names must start with a letter or underscore, and contain only letters, numbers, and underscores.",
            );
          }
          env[trimmedKey] = value || "";
        }
      }

      let limit: number | null = null;
      const limitStr = String(restartLimit || "").trim();
      if (limitStr) {
        const parsed = parseInt(limitStr, 10);
        if (!isNaN(parsed) && parsed >= 0) {
          limit = parsed;
        } else {
          throw new Error("Restart limit must be a non-negative number");
        }
      }

      const validArgs = args
        .map((arg) => (arg || "").trim())
        .filter((arg) => arg.length > 0);

      onSave({
        auto_restart: autoRestart,
        restart_limit: limit,
        env,
        args: validArgs,
        linux_capabilities: capabilities.map((c) => c.trim()).filter(Boolean),
      });
    } catch (error) {
      console.error("Error saving settings:", error);
      throw error;
    }
  }
</script>

<div class="space-y-6 max-w-3xl">
  <div>
    <label class="flex items-center gap-3 cursor-pointer">
      <input
        type="checkbox"
        bind:checked={autoRestart}
        class="w-4 h-4 rounded"
      />
      <div>
        <p class="text-sm font-medium">Auto Restart</p>
        <p class="text-xs opacity-60">
          Automatically restart the service if it crashes
        </p>
      </div>
    </label>
  </div>

  <div>
    <label for="restart-limit" class="block text-xs opacity-60 mb-2">
      Restart Limit (leave empty for unlimited)
    </label>
    <input
      id="restart-limit"
      type="number"
      bind:value={restartLimit}
      placeholder="Unlimited"
      min="0"
      step="1"
      class="input w-full max-w-xs"
      oninput={(e) => {
        const value = e.currentTarget.value;
        if (value && (isNaN(Number(value)) || Number(value) < 0)) {
          e.currentTarget.value = "";
        }
      }}
    />
  </div>

  <div>
    <p class="text-sm font-medium mb-3">Command Arguments</p>
    <div class="space-y-2">
      {#each args as arg, index}
        <div class="flex items-center gap-2">
          <input
            type="text"
            value={arg}
            onchange={(e) => {
              args[index] = e.currentTarget.value;
            }}
            class="input flex-1"
          />
          <button
            class="btn preset-outlined-error w-10 h-10 p-0 flex items-center justify-center"
            onclick={() => removeArg(index)}
            title="Remove argument"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      {/each}
      <div class="flex items-center gap-2">
        <input
          type="text"
          bind:value={newArg}
          onkeypress={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              addArg();
            }
          }}
          placeholder="New argument"
          class="input flex-1"
        />
        <button
          class="btn preset-outlined w-10 h-10 p-0 flex items-center justify-center"
          onclick={addArg}
          title="Add argument"
        >
          <Plus class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>

  <div>
    <p class="text-sm font-medium mb-3">Environment Variables</p>
    <div class="space-y-2">
      {#each envVars as envVar, index}
        <div class="flex items-center gap-2">
          <input
            type="text"
            bind:value={envVar.key}
            placeholder="KEY"
            pattern="[A-Za-z_][A-Za-z0-9_]*"
            title="Variable name should contain only letters, numbers, and underscores, and not start with a number"
            class="input flex-1 font-mono"
            oninput={(e) => {
              const value = e.currentTarget.value;
              e.currentTarget.value = value.toUpperCase();
              envVar.key = e.currentTarget.value;
            }}
          />
          <span class="opacity-40">=</span>
          <input
            type="text"
            bind:value={envVar.value}
            placeholder="value"
            class="input flex-1 font-mono"
          />
          <button
            class="btn preset-outlined-error w-10 h-10 p-0 flex items-center justify-center"
            onclick={() => removeEnvVar(index)}
            title="Remove variable"
          >
            <Trash2 class="w-4 h-4" />
          </button>
        </div>
      {/each}
      <button
        class="btn preset-outlined inline-flex items-center gap-2"
        onclick={addEnvVar}
      >
        <Plus class="w-4 h-4" />
        Add Variable
      </button>
    </div>
  </div>

  <div>
    <p class="text-sm font-medium mb-1">Linux Capabilities</p>
    <p class="text-xs opacity-60 mb-3">
      Grant capabilities to the binary via <code class="font-mono">setcap</code
      >, e.g. <code class="font-mono">cap_net_raw+eip</code>
    </p>
    <div class="space-y-2">
      {#each capabilities as cap, i}
        <div class="flex items-center gap-2">
          <span
            class="font-mono text-xs px-2 py-1 rounded bg-surface-100-800 flex-1"
            >{cap}</span
          >
          <button
            class="btn preset-outlined-error w-8 h-8 p-0 flex items-center justify-center"
            onclick={() => removeCapability(i)}
            title="Remove"
          >
            <Trash2 class="w-3 h-3" />
          </button>
        </div>
      {/each}
      <div class="flex items-center gap-2">
        <input
          type="text"
          bind:value={capInput}
          placeholder="e.g. cap_net_raw+eip"
          class="input flex-1 font-mono text-sm"
          onkeydown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              addCapability();
            }
          }}
        />
        <button
          class="btn preset-outlined w-8 h-8 p-0 flex items-center justify-center"
          onclick={addCapability}
          title="Add capability"
        >
          <Plus class="w-4 h-4" />
        </button>
      </div>
    </div>
  </div>

  <div class="flex justify-end">
    <button
      class="btn preset-filled"
      onclick={handleSave}
      disabled={saving || !serviceDetail}
    >
      {#if saving}
        <span
          class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {/if}
      Save Settings
    </button>
  </div>
</div>
