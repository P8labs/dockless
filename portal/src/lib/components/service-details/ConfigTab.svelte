<script lang="ts">
  import type { ServiceConfig } from "$lib/types";
  import { FileText, RotateCcw, Save } from "lucide-svelte";

  let {
    serviceConfig,
    configLoading,
    configValues = $bindable(),
    configSaving,
    onSave,
    onCreateTemplate,
    onDeleteTemplate,
  }: {
    serviceConfig: ServiceConfig | null;
    configLoading: boolean;
    configValues: Record<string, string | boolean>;
    configSaving: boolean;
    onSave: () => void;
    onCreateTemplate: (
      fields: Record<
        string,
        { value: string; field_type: string; description: string }
      >,
    ) => void;
    onDeleteTemplate: () => void;
  } = $props();

  let showTemplateInput = $state(false);
  let templateInput = $state("");
  let templateCreating = $state(false);
  let parseError = $state("");

  const exampleTemplate = `# Example TOML configuration template
[daemon]
interface = "eth0"
neighbor_check_interval_secs = 60
device_timeout_secs = 300
log_cleanup_enabled = true
log_retention_days = 30

[database]
path = "./service.db"

[api]
host = "0.0.0.0"
port = 8080`;

  function parseTomlTemplate(
    tomlString: string,
  ): Record<
    string,
    { value: string; field_type: string; description: string }
  > {
    const fields: Record<
      string,
      { value: string; field_type: string; description: string }
    > = {};
    const lines = tomlString.split("\n");
    let currentSection = "";
    let lastComment = "";

    for (const line of lines) {
      const trimmed = line.trim();

      // Skip empty lines
      if (!trimmed) {
        lastComment = "";
        continue;
      }

      // Handle comments
      if (trimmed.startsWith("#")) {
        lastComment = trimmed.substring(1).trim();
        continue;
      }

      // Handle sections [section]
      if (trimmed.startsWith("[") && trimmed.endsWith("]")) {
        currentSection = trimmed.substring(1, trimmed.length - 1);
        lastComment = "";
        continue;
      }

      // Handle key-value pairs
      const equalIndex = trimmed.indexOf("=");
      if (equalIndex > 0) {
        let key = trimmed.substring(0, equalIndex).trim();
        let value = trimmed.substring(equalIndex + 1).trim();

        // Remove quotes from keys (handles both "key" and 'key')
        if (
          (key.startsWith('"') && key.endsWith('"')) ||
          (key.startsWith("'") && key.endsWith("'"))
        ) {
          key = key.substring(1, key.length - 1);
        }

        // Remove quotes from string values
        if (
          (value.startsWith('"') && value.endsWith('"')) ||
          (value.startsWith("'") && value.endsWith("'"))
        ) {
          value = value.substring(1, value.length - 1);
        }

        // Determine field type
        let fieldType = "string";
        if (value === "true" || value === "false") {
          fieldType = "boolean";
        } else if (!isNaN(Number(value)) && value !== "") {
          fieldType = value.includes(".") ? "float" : "integer";
        }

        // If key already has dots (like "api.host"), use it as-is
        // Otherwise, prepend the current section if it exists
        const fullKey = key.includes(".")
          ? key
          : currentSection
            ? `${currentSection}.${key}`
            : key;

        // Extract a friendly name from the key for description
        const keyParts = fullKey.split(".");
        const friendlyName = keyParts[keyParts.length - 1].replace(/_/g, " ");

        fields[fullKey] = {
          value: value,
          field_type: fieldType,
          description: lastComment || `Configuration for ${friendlyName}`,
        };

        lastComment = "";
      }
    }

    return fields;
  }

  async function createTemplateFromInput() {
    if (!templateInput.trim()) {
      parseError = "Please enter a TOML configuration template";
      return;
    }

    templateCreating = true;
    parseError = "";

    try {
      const fields = parseTomlTemplate(templateInput);

      if (Object.keys(fields).length === 0) {
        parseError = "No valid configuration fields found in template";
        templateCreating = false;
        return;
      }

      await onCreateTemplate(fields);
      showTemplateInput = false;
      templateInput = "";
    } catch (e) {
      parseError = e instanceof Error ? e.message : "Failed to parse template";
    } finally {
      templateCreating = false;
    }
  }

  function resetTemplate() {
    if (
      confirm(
        "Are you sure you want to delete the current template? This cannot be undone.",
      )
    ) {
      onDeleteTemplate();
    }
  }
</script>

{#if configLoading}
  <div class="flex items-center justify-center py-12">
    <div
      class="w-8 h-8 border-2 border-current border-t-transparent rounded-full animate-spin"
    ></div>
  </div>
{:else if serviceConfig}
  {#if !serviceConfig.has_template}
    <div class="max-w-3xl mx-auto">
      {#if !showTemplateInput}
        <div class="text-center py-12">
          <FileText class="w-16 h-16 mx-auto mb-4 opacity-40" />
          <h3 class="text-lg font-semibold mb-2">No Configuration Template</h3>
          <p class="text-sm opacity-60 mb-2">
            Create a configuration template to enable service configuration
            management
          </p>
          <p class="text-xs opacity-40 mb-6">
            The template will be used to generate a user-friendly configuration
            interface
          </p>
          <button
            class="btn preset-filled-primary"
            onclick={() => (showTemplateInput = true)}
          >
            Create Config Template
          </button>
        </div>
      {:else}
        <div class="card bg-surface-50-950/60 p-6">
          <h3 class="text-lg font-semibold mb-4">
            Create Configuration Template
          </h3>
          <p class="text-sm opacity-70 mb-4">
            Paste your service's TOML configuration template below. Comments
            starting with # will be used as field descriptions.
          </p>

          <div class="mb-4">
            <div class="label mb-2">
              <span class="label-text">TOML Configuration Template</span>
            </div>
            <textarea
              id="template-input"
              bind:value={templateInput}
              placeholder={exampleTemplate}
              class="textarea font-mono text-sm h-96"
            ></textarea>
            {#if parseError}
              <p class="text-sm text-error-500 mt-2">{parseError}</p>
            {/if}
          </div>

          <div class="flex items-center gap-3">
            <button
              class="btn preset-filled"
              onclick={createTemplateFromInput}
              disabled={templateCreating || !templateInput.trim()}
            >
              {#if templateCreating}
                <span
                  class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
                ></span>
              {/if}
              Create Template
            </button>
            <button
              class="btn preset-outlined"
              onclick={() => {
                showTemplateInput = false;
                templateInput = "";
                parseError = "";
              }}
            >
              Cancel
            </button>
          </div>

          <div class="mt-6 p-4 bg-surface-100-800 rounded-lg">
            <p class="text-xs opacity-60 mb-2">Example Template:</p>
            <pre
              class="text-xs opacity-70 overflow-x-auto">{exampleTemplate}</pre>
          </div>
        </div>
      {/if}
    </div>
  {:else}
    <div class="max-w-3xl">
      <div class="flex items-center justify-between mb-6">
        <div>
          <h3 class="text-lg font-semibold">Service Configuration</h3>
          <p class="text-sm opacity-60 mt-1">
            {serviceConfig.fields.length} configuration field{serviceConfig
              .fields.length === 1
              ? ""
              : "s"}
          </p>
          {#if serviceConfig.has_config}
            <span
              class="inline-block mt-2 px-2 py-1 text-xs rounded bg-green-100 text-green-800 border border-green-200"
              >Editing Real Config</span
            >
          {:else if serviceConfig.has_template}
            <span
              class="inline-block mt-2 px-2 py-1 text-xs rounded bg-yellow-100 text-yellow-800 border border-yellow-200"
              >Template Only (not active config)</span
            >
          {/if}
        </div>
        <button
          class="btn preset-ghost inline-flex items-center gap-2 text-sm"
          onclick={resetTemplate}
          title="Reset and create a new template"
        >
          <RotateCcw class="w-4 h-4" />
          Reset Template
        </button>
      </div>

      <div class="space-y-4 mb-6">
        {#each serviceConfig.fields as field}
          <div class="card bg-surface-50-950/30 p-4">
            <label class="label mb-2" for={`config-${field.key}`}>
              <span class="label-text font-medium">{field.key}</span>
            </label>

            {#if field.field_type === "boolean"}
              <label class="flex items-center gap-3 cursor-pointer">
                <input
                  id={`config-${field.key}`}
                  type="checkbox"
                  checked={configValues[field.key] === true ||
                    configValues[field.key] === "true"}
                  onchange={(e) => {
                    configValues[field.key] = e.currentTarget.checked;
                  }}
                  class="checkbox"
                />
                <span class="text-sm opacity-70">{field.description}</span>
              </label>
            {:else if field.field_type === "integer"}
              <input
                id={`config-${field.key}`}
                type="number"
                step="1"
                bind:value={configValues[field.key]}
                class="input"
              />
              {#if field.description}
                <p class="text-xs opacity-60 mt-2">{field.description}</p>
              {/if}
            {:else if field.field_type === "float"}
              <input
                id={`config-${field.key}`}
                type="number"
                step="0.01"
                bind:value={configValues[field.key]}
                class="input"
              />
              {#if field.description}
                <p class="text-xs opacity-60 mt-2">{field.description}</p>
              {/if}
            {:else}
              <input
                id={`config-${field.key}`}
                type="text"
                bind:value={configValues[field.key]}
                class="input"
              />
              {#if field.description}
                <p class="text-xs opacity-60 mt-2">{field.description}</p>
              {/if}
            {/if}
          </div>
        {/each}
      </div>

      <div class="flex items-center gap-3 pt-4 border-t border-surface-200/10">
        <button
          class="btn preset-filled inline-flex items-center gap-2"
          disabled={configSaving}
          onclick={onSave}
        >
          {#if configSaving}
            <span
              class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
            ></span>
          {:else}
            <Save class="w-4 h-4" />
          {/if}
          Save Configuration
        </button>
        <p class="text-xs opacity-60">
          Service must be restarted for changes to take effect
        </p>
      </div>
    </div>
  {/if}
{/if}
