<script lang="ts">
  import {
    iconNoContainer,
    iconBinary,
    iconColdStart,
    iconShield,
    iconUnix,
    iconRaspberryPi,
    iconWebApp,
    iconSystemd,
    iconGitHub,
    iconArrow,
    iconFork,
  } from "$lib/svgs";
  import { fade } from "svelte/transition";
  let open: string | null = "repo";

  const toggle = (key: string) => {
    open = open === key ? null : key;
  };
</script>

<section
  class="relative overflow-hidden bg-bg"
  style="background: radial-gradient(ellipse 65% 55% at 70% 55%, rgba(168,228,32,0.07), transparent 68%), #f9fafb;"
>
  <div
    class="max-w-6xl mx-auto px-6 py-24 grid grid-cols-1 md:grid-cols-2 gap-12 items-center"
  >
    <div>
      <span
        class="inline-flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-ink-mute mb-5"
      >
        <span class="block w-5 h-0.5 rounded bg-accent"></span>
        Edge Infrastructure
      </span>

      <h1
        class="text-[clamp(2.2rem,4.5vw,3.2rem)] font-bold leading-[1.15] tracking-tight text-ink mb-5 max-w-[22ch]"
      >
        Run services on tiny hardware,&nbsp;<em class="not-italic text-accent"
          >without containers</em
        >
      </h1>

      <p class="text-[1.05rem] leading-relaxed text-ink-mid max-w-[44ch] mb-8">
        Dockless is a lightweight program service for edge devices. No Docker,
        no orchestration overhead, just install, configure once, and it's there
        on every boot.
      </p>

      <div class="flex flex-wrap items-center gap-3">
        <a
          href="/docs/getting-started"
          id="hero-get-started"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-sm bg-accent text-amber-50 border border-accent-dark font-semibold text-sm hover:bg-accent-dark transition-colors"
        >
          Get started
          <span class="icon">{@html iconArrow}</span>
        </a>

        <a
          href="/docs"
          id="hero-docs"
          class="inline-flex items-center gap-2 px-5 py-2.5 rounded-sm bg-white border border-[#d1d5db] text-ink font-semibold text-sm hover:border-accent hover:bg-bg transition-colors"
        >
          Read the docs
        </a>
      </div>
    </div>

    <div class="hidden md:flex justify-end items-start pt-2">
      <img
        src="/primary_art.png"
        alt="Dockless runtime illustration"
        class="w-full max-w-115 object-contain scale-125"
        style="transform: translateY(20px);"
      />
    </div>
  </div>
</section>

<section class="bg-white py-24">
  <div class="max-w-6xl mx-auto px-6">
    <span
      class="inline-flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-ink-mute mb-4"
    >
      <span class="block w-4 h-0.5 rounded bg-accent"></span>
      What it does
    </span>
    <h2
      class="text-[clamp(1.6rem,3vw,2.25rem)] font-bold tracking-tight text-ink mb-3 max-w-[28ch]"
    >
      Designed for constrained environments
    </h2>
    <p class="text-base leading-relaxed text-ink-mute max-w-[52ch] mb-14">
      Dockless strips container complexity away so you can focus on your service
      logic. Minimal footprint. Predictable behaviour. Works on hardware you
      already have.
    </p>

    <div class="grid grid-cols-1 sm:grid-cols-2 gap-x-14 gap-y-10">
      {#each [{ icon: iconNoContainer, title: "No container runtime required", desc: "Deploy directly to edge devices without Docker, containerd, or any OCI toolchain. Reduces memory overhead significantly on low-RAM hardware." }, { icon: iconBinary, title: "Single binary deployment", desc: "Package your service as a single artifact. Copy it, run it. No dependency graphs to resolve on the target device." }, { icon: iconColdStart, title: "Fast cold starts", desc: "Services start in milliseconds. Ideal for intermittent workloads or devices that power-cycle frequently." }, { icon: iconShield, title: "Isolated process model", desc: "Each service runs in a controlled process with defined resource limits. Failures stay contained without full container overhead." }, { icon: iconUnix, title: "Unix-native & composable", desc: "Plays well with systemd, shell scripts, and standard Unix tooling. Add it to your existing workflows without friction." }, { icon: iconRaspberryPi, title: "Works on Raspberry Pi", desc: "Tested on ARMv7 and ARM64 boards. Run real services on the hardware sitting in your lab right now." }] as f}
        <div class="flex gap-4">
          <div
            class="shrink-0 w-9 h-9 rounded-lg bg-[rgba(168,228,32,0.12)] flex items-center justify-center text-accent mt-0.5 icon"
          >
            {@html f.icon}
          </div>
          <div>
            <div class="text-[0.95rem] font-semibold text-ink mb-1">
              {f.title}
            </div>
            <p class="text-sm leading-relaxed text-ink-mute">
              {f.desc}
            </p>
          </div>
        </div>
      {/each}
    </div>
  </div>
</section>

<section class="bg-bg-alt py-24 relative overflow-hidden">
  <div
    class="absolute inset-0 pointer-events-none"
    style="background: radial-gradient(ellipse 55% 60% at 95% 50%, rgba(168,228,32,0.07), transparent 70%);"
  ></div>
  <div
    class="max-w-6xl mx-auto px-6 relative grid grid-cols-1 lg:grid-cols-2 gap-16 items-start"
  >
    <div>
      <span
        class="inline-flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-ink-mute mb-4"
      >
        <span class="block w-4 h-0.5 rounded bg-accent-dark"></span>
        How it works
      </span>
      <h2
        class="text-[clamp(1.6rem,3vw,2.25rem)] font-bold tracking-tight text-ink mb-4 max-w-[28ch]"
      >
        A program service, not a CLI tool
      </h2>
      <p class="text-base leading-relaxed text-ink-mute max-w-[46ch] mb-10">
        Dockless runs as a persistent system service — configure it once and it
        is available on every boot. No manual intervention needed.
      </p>

      <div class="flex flex-col gap-8">
        <div class="flex gap-4">
          <div
            class="shrink-0 w-8 h-8 rounded-full bg-[rgba(168,228,32,0.15)] border border-[rgba(168,228,32,0.3)] flex items-center justify-center text-xs font-bold text-accent mt-0.5"
          >
            1
          </div>
          <div>
            <h3 class="text-[0.95rem] font-semibold text-ink mb-1">
              Install &amp; configure
            </h3>
            <p class="text-sm leading-relaxed text-ink-mute">
              Install the Dockless binary and drop a minimal config file. Point
              it at your services, set your preferred port, done.
            </p>
          </div>
        </div>

        <div class="flex gap-4">
          <div
            class="shrink-0 w-8 h-8 rounded-full bg-[rgba(168,228,32,0.15)] border border-[rgba(168,228,32,0.3)] flex items-center justify-center text-xs font-bold text-accent mt-0.5 icon"
          >
            <span class="icon">{@html iconSystemd}</span>
          </div>
          <div>
            <h3 class="text-[0.95rem] font-semibold text-ink mb-1">
              Enable via systemd (or equivalent)
            </h3>
            <p class="text-sm leading-relaxed text-ink-mute">
              Register Dockless as a systemd unit. It starts automatically on
              boot, recovers from crashes, and stays out of your way.
            </p>
          </div>
        </div>

        <div class="flex gap-4">
          <div
            class="shrink-0 w-8 h-8 rounded-full bg-[rgba(168,228,32,0.15)] border border-[rgba(168,228,32,0.3)] flex items-center justify-center text-xs font-bold text-accent mt-0.5 icon"
          >
            <span class="icon">{@html iconWebApp}</span>
          </div>
          <div>
            <h3 class="text-[0.95rem] font-semibold text-ink mb-1">
              Manage through the web UI
            </h3>
            <p class="text-sm leading-relaxed text-ink-mute">
              Open <code
                class="text-[0.8rem] bg-white px-1.5 py-0.5 rounded border border-border text-ink-mid"
                >[device-ip]:[port]</code
              > in any browser. Start, stop, inspect logs, and reconfigure services
              — no terminal required.
            </p>
          </div>
        </div>

        <div class="flex gap-4">
          <div
            class="shrink-0 w-8 h-8 rounded-full bg-[rgba(168,228,32,0.15)] border border-[rgba(168,228,32,0.3)] flex items-center justify-center text-xs font-bold text-accent mt-0.5"
          >
            ✦
          </div>
          <div>
            <h3 class="text-[0.95rem] font-semibold text-ink mb-1">
              Pre-baked Pi image <span
                class="text-xs font-normal text-ink-mute ml-1">coming soon</span
              >
            </h3>
            <p class="text-sm leading-relaxed text-ink-mute">
              Flash a ready-to-run ISO directly to your Raspberry Pi. Dockless
              will be fully configured and running on first boot — zero setup.
            </p>
          </div>
        </div>
      </div>
    </div>

    <div
      class="bg-white border border-border rounded-xl p-6 flex flex-col gap-4"
    >
      <div
        class="text-[0.7rem] font-semibold tracking-widest uppercase text-ink-mute"
      >
        /etc/dockless/config.toml
      </div>
      <div
        class="bg-bg border border-border rounded-lg px-5 py-4 font-mono text-[0.8rem] leading-[1.75] text-ink-mid"
      >
        <span class="text-ink-mute"># Dockless config</span><br />
        <span class="text-accent font-semibold">data_dir</span> =
        <span class="text-[#065f46]">"/etc/dockless/data"</span><br />
        <span class="text-accent font-semibold">listen_port</span> =
        <span class="text-[#065f46]">3080</span><br />
      </div>

      <div
        class="text-[0.7rem] font-semibold tracking-widest uppercase text-ink-mute mt-2"
      >
        Enable on boot
      </div>
      <div
        class="bg-bg border border-border rounded-lg px-5 py-4 font-mono text-[0.8rem] leading-[1.75] text-ink-mid"
      >
        <span class="text-ink-mute">$ </span> systemctl enable --now dockless<br
        />
        <span class="text-accent">[INFO]</span> Starting dockless runtime<br />
        <span class="text-accent">[INFO]</span> Version 0.1.0<br />
        <span class="text-accent">[INFO]</span> Node Id:
        30a29cf1-c6e1-4a5b-a7a9-76706f2724a7<br />
        <span class="text-accent">[INFO]</span> Portal at http://0.0.0.0:3080
      </div>
    </div>
  </div>
</section>

<section class="bg-white py-28">
  <div class="max-w-5xl mx-auto px-6">
    <div class="grid md:grid-cols-2 gap-16 items-start">
      <div>
        <h2
          class="text-[clamp(1.8rem,3vw,2.4rem)] font-bold tracking-tight text-ink mb-6"
        >
          Open by design.
        </h2>

        <p class="text-lg leading-relaxed text-ink-mute mb-6">
          Dockless is fully open source and built in public. You can inspect the
          code, follow architectural decisions, and run it without restrictions.
        </p>

        <p class="text-base leading-relaxed text-ink-mute">
          Development happens in the open. Issues, discussions, and roadmap
          direction are visible to everyone.
        </p>
      </div>

      <div class="border border-border rounded-sm bg-bg overflow-hidden">
        <!-- Tabs -->
        <div class="flex border-b border-border">
          <button
            class="flex-1 px-6 py-4 text-sm font-semibold transition-colors"
            class:text-ink={open === "repo"}
            class:text-ink-mute={open !== "repo"}
            class:border-b-2={open === "repo"}
            class:border-accent-dark={open === "repo"}
            on:click={() => toggle("repo")}
          >
            <span class="inline-flex items-center gap-2">
              <span class="icon">{@html iconGitHub}</span>
              Source Code
            </span>
          </button>

          <button
            class="flex-1 px-6 py-4 text-sm font-semibold transition-colors"
            class:text-ink={open === "issues"}
            class:text-ink-mute={open !== "issues"}
            class:border-b-2={open === "issues"}
            class:border-accent-dark={open === "issues"}
            on:click={() => toggle("issues")}
          >
            <span class="inline-flex items-center gap-2">
              <span class="icon">{@html iconFork}</span>
              Issues
            </span>
          </button>
        </div>

        <!-- Fixed Content Area -->
        <div class="relative h-56">
          {#if open === "repo"}
            <div
              class="absolute inset-0 px-6 py-6 flex flex-col justify-between"
              in:fade={{ duration: 200 }}
              out:fade={{ duration: 150 }}
            >
              <div>
                <p class="text-sm text-ink-mute leading-relaxed mb-4">
                  Dockless development happens entirely in public. Inspect
                  architectural decisions, follow commit history, and review
                  implementation details directly in the repository.
                </p>

                <ul class="text-sm text-ink-mute space-y-1">
                  <li>• Runtime architecture and service model</li>
                  <li>• Active commits and pull requests</li>
                  <li>• Transparent change history</li>
                </ul>
              </div>

              <a
                href="https://github.com/p8labs/dockless"
                target="_blank"
                rel="noopener"
                class="text-sm mt-2 font-semibold text-accent-dark hover:opacity-80 transition-opacity"
              >
                View repository →
              </a>
            </div>
          {/if}

          {#if open === "issues"}
            <div
              class="absolute inset-0 px-6 py-6 flex flex-col justify-between"
              in:fade={{ duration: 200 }}
              out:fade={{ duration: 150 }}
            >
              <div>
                <p class="text-sm text-ink-mute leading-relaxed mb-4">
                  Participate in discussions, propose improvements, and track
                  ongoing development milestones.
                </p>

                <ul class="text-sm text-ink-mute space-y-1">
                  <li>• Bug reports and feature requests</li>
                  <li>• Roadmap discussions</li>
                  <li>• Architectural feedback</li>
                </ul>
              </div>

              <a
                href="https://github.com/p8labs/dockless/issues"
                target="_blank"
                rel="noopener"
                class="text-sm font-semibold text-accent-dark hover:opacity-80 transition-opacity"
              >
                View issues →
              </a>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</section>
