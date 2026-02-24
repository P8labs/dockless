<script lang="ts">
  import type { ServiceConfig } from "$lib/types";
  import { Plus, Trash2 } from "lucide-svelte";

  let {
    serviceConfig,
    configLoading,
    configValues = $bindable(),
    configSaving,
    onSave,
    onCreateTemplate,
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
  } = $props();

  let showAddField = $state(false);
  let newFieldKey = $state("");
  let newFieldValue = $state("");
  let newFieldType = $state("string");
  let newFieldDesc = $state("");
  let templateCreating = $state(false);

  function addField() {
    if (!newFieldKey.trim()) return;
    configValues[newFieldKey] = newFieldValue;
    if (serviceConfig) {
      serviceConfig.fields.push({
        key: newFieldKey,
        value: newFieldValue,
        field_type: newFieldType,
        description: newFieldDesc || `Configuration for ${newFieldKey}`,
      });
    }
    newFieldKey = "";
    newFieldValue = "";
    newFieldType = "string";
    newFieldDesc = "";
    showAddField = false;
  }

  function removeField(key: string) {
    delete configValues[key];
    if (serviceConfig) {
      serviceConfig.fields = serviceConfig.fields.filter((f) => f.key !== key);
    }
  }

  async function createTemplate() {
    if (templateCreating) return;
    templateCreating = true;
    const fields: Record<
      string,
      { value: string; field_type: string; description: string }
    > = {
      port: {
        value: "8080",
        field_type: "integer",
        description: "Service port number",
      },
      host: {
        value: "0.0.0.0",
        field_type: "string",
        description: "Service host address",
      },
      debug: {
        value: "false",
        field_type: "boolean",
        description: "Enable debug mode",
      },
    };
    await onCreateTemplate(fields);
    templateCreating = false;
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
    <div class="text-center py-12">
      <p class="text-sm opacity-60 mb-4">No config.example.toml found</p>
      <p class="text-xs opacity-40 mb-6">
        Create a config template to enable service configuration management
      </p>
      <button
        class="btn preset-filled-primary"
        disabled={templateCreating}
        onclick={createTemplate}
      >
        {#if templateCreating}
          <span
            class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
          ></span>
        {/if}
        Create Config Template
      </button>
    </div>
  {:else}
    <div class="max-w-2xl">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-semibold">Configuration Fields</h3>
        <button
          class="btn btn-sm preset-outlined"
          onclick={() => (showAddField = !showAddField)}
        >
          <Plus class="w-4 h-4" />
          Add Field
        </button>
      </div>

      {#if showAddField}
        <div class="card bg-surface-100-900 p-4 mb-4">
          <h4 class="text-sm font-medium mb-3">New Configuration Field</h4>
          <div class="grid grid-cols-2 gap-3 mb-3">
            <div>
              <label class="label" for="new-field-key">
                <span class="label-text text-xs">Key</span>
              </label>
              <input
                id="new-field-key"
                type="text"
                bind:value={newFieldKey}
                placeholder="field_name"
                class="input input-sm"
              />
            </div>
            <div>
              <label class="label" for="new-field-type">
                <span class="label-text text-xs">Type</span>
              </label>
              <select
                id="new-field-type"
                bind:value={newFieldType}
                class="select select-sm"
              >
                <option value="string">String</option>
                <option value="integer">Integer</option>
                <option value="float">Float</option>
                <option value="boolean">Boolean</option>
              </select>
            </div>
          </div>
          <div class="mb-3">
            <label class="label" for="new-field-value">
              <span class="label-text text-xs">Default Value</span>
            </label>
            {#if newFieldType === "boolean"}
              <label class="flex items-center gap-2">
                <input
                  id="new-field-value"
                  type="checkbox"
                  checked={newFieldValue === "true"}
                  onchange={(e) => {
                    newFieldValue = e.currentTarget.checked ? "true" : "false";
                  }}
                  class="checkbox"
                />
                <span class="text-sm">Enabled</span>
              </label>
            {:else}
              <input
                id="new-field-value"
                type={newFieldType === "integer" || newFieldType === "float"
                  ? "number"
                  : "text"}
                step={newFieldType === "float" ? "0.01" : "1"}
                bind:value={newFieldValue}
                placeholder="Default value"
                class="input input-sm"
              />
            {/if}
          </div>
          <div class="mb-3">
            <label class="label" for="new-field-desc">
              <span class="label-text text-xs">Description</span>
            </label>
            <input
              id="new-field-desc"
              type="text"
              bind:value={newFieldDesc}
              placeholder="Field description"
              class="input input-sm"
            />
          </div>
          <div class="flex gap-2">
            <button class="btn btn-sm preset-filled" onclick={addField}>
              Add
            </button>
            <button
              class="btn btn-sm preset-outlined"
              onclick={() => {
                showAddField = false;
                newFieldKey = "";
                newFieldValue = "";
                newFieldDesc = "";
              }}
            >
              Cancel
            </button>
          </div>
        </div>
      {/if}

      <div class="space-y-4 mb-6">
        {#each serviceConfig.fields as field}
          <div class="card bg-surface-50-950/30 p-4">
            <div class="flex items-start justify-between mb-2">
              <div class="font-medium">{field.key}</div>
              <button
                class="btn btn-sm preset-ghost text-error-500"
                onclick={() => removeField(field.key)}
                title="Remove field"
              >
                <Trash2 class="w-4 h-4" />
              </button>
            </div>
            {#if field.field_type === "boolean"}
              <label class="flex items-center gap-2 cursor-pointer">
                <input
                  id={`config-${field.key}`}
                  type="checkbox"
                  checked={configValues[field.key] === true}
                  onchange={(e) => {
                    configValues[field.key] = e.currentTarget.checked;
                  }}
                  class="checkbox"
                />
                <span class="text-sm opacity-70">{field.description}</span>
              </label>
            {:else if field.field_type === "integer" || field.field_type === "float"}
              <input
                id={`config-${field.key}`}
                type="number"
                step={field.field_type === "float" ? "0.01" : "1"}
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
          class="btn preset-outlined"
          disabled={configSaving}
          onclick={onSave}
        >
          {#if configSaving}
            <span
              class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin"
            ></span>
          {/if}
          Save Configuration
        </button>
        <p class="text-xs opacity-60">
          Service will need to be restarted for changes to take effect
        </p>
      </div>
    </div>
  {/if}
{/if}
