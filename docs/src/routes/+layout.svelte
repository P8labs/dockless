<script lang="ts">
  import "./layout.css";

  let { children } = $props();
</script>

<nav class="sticky top-0 z-50 backdrop-blur-lg">
  <div class="max-w-6xl mx-auto px-6 flex items-center justify-between h-15">
    <a href="/" class="flex items-center gap-2 font-bold text-[1.05rem]">
      <img src="/dockless.png" alt="Dockless" class="h-8 object-contain" />
    </a>

    <div class="flex items-center gap-8">
      <a
        href="/docs"
        class="text-sm font-medium text-ink-mute hover:text-ink transition-colors"
        >Docs</a
      >
      <a
        href="https://github.com/p8labs/dockless"
        target="_blank"
        rel="noopener"
        class="text-sm font-medium text-ink-mute hover:text-ink transition-colors"
        >GitHub</a
      >
      <a
        href="mailto:hello@p8labs.tech"
        class="text-sm font-medium text-ink-mute hover:text-ink transition-colors"
        >Contact</a
      >
      <a
        href="/docs/getting-started"
        id="nav-get-started"
        class="hover:underline text-sm font-semibold hover:text-accent-dark text-ink-mute transition-colors"
      >
        Get Started
      </a>
    </div>
  </div>
</nav>

{@render children()}
<footer class="py-6">
  <div
    class="max-w-6xl mx-auto px-6 flex flex-wrap items-center justify-between gap-4"
  >
    <span class="font-semibold text-ink text-[0.95rem]"
      >© 2026 Dockless, Built at <a
        class="underline hover:text-accent transition-colors"
        href="https://p8labs.tech">P8labs</a
      >.</span
    >

    <div class="flex flex-wrap items-center gap-6">
      <nav class="flex gap-6">
        <a
          href="/docs"
          class="text-sm text-ink-mute hover:text-ink transition-colors">Docs</a
        >
        <a
          href="https://github.com/p8labs/dockless"
          target="_blank"
          rel="noopener"
          class="text-sm text-ink-mute hover:text-ink transition-colors"
          >GitHub</a
        >
        <a
          href="mailto:hello@p8labs.tech"
          class="text-sm text-ink-mute hover:text-ink transition-colors"
          >Contact</a
        >
      </nav>
    </div>
  </div>
</footer>
