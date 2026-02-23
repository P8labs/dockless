<script lang="ts">
  import Toaster from "$lib/components/Toast.svelte";
  import "./layout.css";

  let { children } = $props();
</script>

<div class="min-h-screen">
  {@render children()}
  <Toaster />
</div>
