<script lang="ts">
  import type { ServiceDefinition } from "$lib/types";

  let { serviceDetail }: { serviceDetail: ServiceDefinition | null } = $props();
</script>

<div class="space-y-4 max-w-2xl">
  <div>
    <p class="text-xs opacity-60 mb-1">Binary Path</p>
    <p class="text-sm font-mono break-all">
      {serviceDetail?.binary_path || "Not configured"}
    </p>
  </div>
  {#if serviceDetail?.current_version}
    <div>
      <p class="text-xs opacity-60 mb-1">Version</p>
      <p class="text-sm font-mono">
        {serviceDetail.current_version}
      </p>
    </div>
  {/if}
  {#if serviceDetail?.port}
    <div>
      <p class="text-xs opacity-60 mb-1">Exposed Port</p>
      <p class="text-sm font-mono">
        {serviceDetail.port}
      </p>
    </div>
  {/if}
  <div>
    <p class="text-xs opacity-60 mb-1">Auto Restart</p>
    <p class="text-sm">
      {serviceDetail?.auto_restart ? "Enabled" : "Disabled"}
    </p>
  </div>
  {#if serviceDetail?.restart_limit !== null && serviceDetail?.restart_limit !== undefined}
    <div>
      <p class="text-xs opacity-60 mb-1">Restart Limit</p>
      <p class="text-sm">{serviceDetail.restart_limit}</p>
    </div>
  {/if}
  {#if serviceDetail?.args && serviceDetail.args.length > 0}
    <div>
      <p class="text-xs opacity-60 mb-1">Arguments</p>
      <div class="flex flex-wrap gap-1.5">
        {#each serviceDetail.args as arg}
          <span class="text-xs font-mono px-2 py-1 rounded bg-surface-100-800">
            {arg}
          </span>
        {/each}
      </div>
    </div>
  {/if}
</div>
