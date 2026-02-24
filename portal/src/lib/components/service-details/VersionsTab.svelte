<script lang="ts">
  import type { ArtifactInfo } from "$lib/types";
  import { Upload, Github } from "lucide-svelte";

  let {
    artifactInfo,
    artifactLoading,
    activeAction,
    uploadVersion = $bindable(),
    uploadFile = $bindable(),
    ghRepo = $bindable(),
    ghVersion = $bindable(),
    ghAsset = $bindable(),
    onUpload,
    onGithubInstall,
  }: {
    artifactInfo: ArtifactInfo | null;
    artifactLoading: boolean;
    activeAction: string;
    uploadVersion: string;
    uploadFile: File | null;
    ghRepo: string;
    ghVersion: string;
    ghAsset: string;
    onUpload: () => void;
    onGithubInstall: () => void;
  } = $props();
</script>

<div class="space-y-6 max-w-3xl">
  {#if artifactLoading}
    <div class="flex items-center justify-center py-12">
      <div
        class="w-8 h-8 border-2 border-current border-t-transparent rounded-full animate-spin"
      ></div>
    </div>
  {:else}
    {#if artifactInfo}
      <div class="card bg-surface-100-800 p-4 space-y-3">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium opacity-70">Current Version</span>
          <span class="font-mono text-sm font-medium"
            >{artifactInfo.current_version ?? "None"}</span
          >
        </div>
        {#if artifactInfo.available_versions.length > 0}
          <div>
            <span class="text-sm font-medium opacity-70 block mb-2"
              >Available Versions</span
            >
            <div class="flex flex-wrap gap-2">
              {#each artifactInfo.available_versions as ver}
                <span
                  class="border border-surface-200/10 rounded-md px-3 py-1.5 text-sm font-mono hover:bg-surface-100-800 transition-colors cursor-default"
                >
                  {ver}
                </span>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}

    <div class="border-t border-surface-200/10 pt-6">
      <div class="flex items-center gap-2 mb-4">
        <Upload class="w-5 h-5" />
        <h3 class="text-sm font-semibold uppercase tracking-wide">
          Manual Upload
        </h3>
      </div>
      <div class="flex items-end gap-3 flex-wrap">
        <div class="flex-1 min-w-40">
          <label class="label">
            <span class="label-text">Version</span>
            <input
              id="upload-version"
              type="text"
              class="input"
              placeholder="v1.0.0"
              bind:value={uploadVersion}
            />
          </label>
        </div>
        <div class="flex-1 min-w-50">
          <label class="label">
            <span class="label-text">Binary File</span>
            <input
              id="upload-file"
              type="file"
              class="input"
              onchange={(e) => {
                const target = e.currentTarget as HTMLInputElement;
                uploadFile = target.files?.[0] ?? null;
              }}
            />
          </label>
        </div>
        <button
          class="btn preset-outlined"
          disabled={!uploadFile || !uploadVersion || !!activeAction}
          onclick={onUpload}
        >
          {#if activeAction === "upload"}
            <span
              class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
            ></span>
          {/if}
          Upload
        </button>
      </div>
    </div>

    <div class="border-t border-surface-200/10 pt-6">
      <div class="flex items-center gap-2 mb-4">
        <Github class="w-5 h-5" />
        <h3 class="text-sm font-semibold uppercase tracking-wide">
          Install from GitHub
        </h3>
      </div>
      <div class="flex flex-wrap items-end gap-3">
        <div class="flex-1 min-w-50">
          <label class="label">
            <span class="label-text">Repository</span>
            <input
              id="gh-repo"
              type="text"
              class="input"
              placeholder="owner/repo"
              bind:value={ghRepo}
            />
          </label>
        </div>
        <div class="flex-1 min-w-30">
          <label class="label">
            <span class="label-text">Version</span>
            <input
              id="gh-version"
              type="text"
              class="input"
              placeholder="v1.0.0"
              bind:value={ghVersion}
            />
          </label>
        </div>
        <div class="flex-1 min-w-40">
          <label class="label">
            <span class="label-text">Asset Name</span>
            <input
              id="gh-asset"
              type="text"
              class="input"
              placeholder="binary-linux-amd64"
              bind:value={ghAsset}
            />
          </label>
        </div>
        <button
          class="btn preset-outlined"
          disabled={!ghRepo || !ghVersion || !ghAsset || !!activeAction}
          onclick={onGithubInstall}
        >
          {#if activeAction === "install"}
            <span
              class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
            ></span>
          {/if}
          Install
        </button>
      </div>
    </div>
  {/if}
</div>
