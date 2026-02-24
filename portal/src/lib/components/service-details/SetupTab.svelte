<script lang="ts">
  import { Upload, Github, AlertCircle } from "lucide-svelte";

  let {
    activeAction,
    uploadVersion = $bindable(),
    uploadFile = $bindable(),
    ghRepo = $bindable(),
    ghVersion = $bindable(),
    ghAsset = $bindable(),
    onUpload,
    onGithubInstall,
  }: {
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

<div class="max-w-3xl mx-auto">
  <div class="flex items-start gap-3 mb-6 p-4 rounded-lg bg-amber-500/10">
    <AlertCircle class="w-5 h-5 text-amber-600 shrink-0 mt-0.5" />
    <div>
      <h3 class="text-lg font-semibold mb-1">Service Setup Required</h3>
      <p class="text-sm opacity-70">
        This service needs to be configured with a binary version before it can
        be started. Upload or install a binary from GitHub.
      </p>
    </div>
  </div>

  <div class="space-y-6">
    <div class="card bg-surface-100-800 p-6">
      <div class="flex items-center gap-2 mb-4">
        <Upload class="w-5 h-5" />
        <h4 class="text-sm font-semibold uppercase tracking-wide">
          Manual Upload
        </h4>
      </div>
      <div class="flex items-end gap-3 flex-wrap">
        <div class="flex-1 min-w-40">
          <label class="label">
            <span class="label-text">Version</span>
            <input
              id="setup-upload-version"
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
              id="setup-upload-file"
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
          Upload & Activate
        </button>
      </div>
    </div>

    <div class="card bg-surface-100-800 p-6">
      <div class="flex items-center gap-2 mb-4">
        <Github class="w-5 h-5" />
        <h4 class="text-sm font-semibold uppercase tracking-wide">
          Install from GitHub
        </h4>
      </div>
      <div class="flex flex-wrap items-end gap-3">
        <div class="flex-1 min-w-50">
          <label class="label">
            <span class="label-text">Repository</span>
            <input
              id="setup-gh-repo"
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
              id="setup-gh-version"
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
              id="setup-gh-asset"
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
          Install & Activate
        </button>
      </div>
    </div>
  </div>
</div>
