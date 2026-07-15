<script lang="ts">
  import init, { init_system } from 'onyx-core';
  import wasmBase64 from 'onyx-core/onyx_core_bg.wasm';
  import { slide } from 'svelte/transition';

  let engineState = $state('Loading Iron Core...');
  let isCoreReady = $state(false);
  let workspaces = $state<Array<{id: string; name: string; type: string; tier: string}>>([]);
  let expandedTier = $state<string>('Personal');
  let justCreatedId = $state<string | null>(null);

  const tiers = [
    { type: 'Personal', tier: 'Free', desc: 'Individual finance tracking with zero-based budgeting.' },
    { type: 'Freelance', tier: 'Premium', desc: 'Contractor management, invoicing, and tax estimations.' },
    { type: 'Business', tier: 'Premium', desc: 'Full business accounting suite for multiple entities.' },
  ];

  $effect(() => {
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    init({ module_or_path: wasmBytes }).then(() => {
      init_system();
      engineState = 'Iron Core ready';
      isCoreReady = true;
    }).catch(err => {
      engineState = 'Core catastrophic failure: ' + err;
      console.error(err);
    });
  });

  function toggleTier(type: string) {
    expandedTier = expandedTier === type ? null : type;
    justCreatedId = null;
  }

  function createWorkspace(type: string, tierLabel: string) {
    if (!isCoreReady) return;
    const ws = { id: crypto.randomUUID(), name: `New ${type} Workspace`, type, tier: tierLabel };
    workspaces = [...workspaces, ws];
    justCreatedId = ws.id;
  }

  function openWorkspace(id: string) {
    console.log('Opening workspace:', id);
  }
</script>

<div class="onyx-container">
  <header class="onyx-header">
    <h1 class="onyx-title">Onyx Ledger</h1>
    <button class="onyx-icon-btn" aria-label="Menu" title="Settings">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/></svg>
    </button>
  </header>

  {#if !isCoreReady}
    <div class="onyx-loading">
      <p>{engineState}</p>
    </div>
  {:else}
    <div class="tier-row">
      {#each tiers as t}
        {@const isExpanded = expandedTier === t.type}
        {@const isDimmed = expandedTier !== null && !isExpanded}
        {@const created = justCreatedId ? workspaces.find(w => w.id === justCreatedId && w.type === t.type) : null}

        <div
          class="tier-panel"
          class:tier-expanded={isExpanded}
          class:tier-dimmed={isDimmed}
        >
          <button class="tier-header" onclick={() => toggleTier(t.type)}>
            <span class="tier-header-left">
              <span class="tier-name">{t.type}</span>
              <span class="tier-badge" class:badge-free={t.tier === 'Free'} class:badge-premium={t.tier !== 'Free'}>
                {t.tier}
              </span>
            </span>
            <svg class="chevron" class:chevron-open={isExpanded} width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg>
          </button>

          {#if isExpanded}
            <div class="tier-body" transition:slide>
              <p class="tier-desc">{t.desc}</p>
              <button class="tier-btn" onclick={() => createWorkspace(t.type, t.tier)}>
                New Workspace
              </button>

              {#if created}
                <div class="created-banner">
                  <span class="created-name">{created.name}</span>
                  <button class="open-btn" onclick={() => openWorkspace(created.id)}>Open</button>
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .onyx-container {
    width: 100% !important;
    max-width: 1100px !important;
    margin: 0 auto !important;
    padding: 2.5rem 2rem !important;
    font-family: var(--font-interface);
    color: var(--text-normal);
    box-sizing: border-box;
  }

  .onyx-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 3rem !important;
    border-bottom: 2px solid var(--background-modifier-border);
    padding-bottom: 1.25rem;
  }

  .onyx-title {
    margin: 0;
    font-size: 2.4em;
    font-weight: 800;
    letter-spacing: -0.5px;
    color: var(--text-normal);
  }

  .onyx-icon-btn {
    background: transparent;
    box-shadow: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 10px;
    border-radius: var(--radius-s);
    transition: color 0.2s ease, background 0.2s ease;
  }

  .onyx-icon-btn:hover {
    color: var(--text-normal);
    background: var(--background-modifier-hover);
  }

  .onyx-loading {
    text-align: center;
    padding: 4rem;
    color: var(--text-muted);
    font-family: var(--font-monospace);
  }

  .tier-row {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 2rem;
    align-items: start;
    width: 100% !important;
  }

  @media (max-width: 800px) {
    .tier-row {
      grid-template-columns: 1fr;
    }
  }

  .tier-panel {
    width: 100% !important;
    background: var(--background-secondary);
    border: 2px solid var(--background-modifier-border);
    border-radius: 16px;
    overflow: hidden;
    transition: border-color 0.3s ease, opacity 0.3s ease, box-shadow 0.3s ease, background 0.3s ease;
  }

  .tier-panel.tier-expanded {
    border-color: var(--interactive-accent);
    box-shadow: 0 0 0 2px var(--interactive-accent), 0 4px 20px rgba(0, 0, 0, 0.15);
    background: color-mix(in srgb, var(--interactive-accent) 6%, var(--background-secondary));
  }

  .tier-panel.tier-dimmed {
    opacity: 0.3;
  }

  .tier-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100% !important;
    padding: 1.5rem 2rem !important;
    background: none;
    border: none;
    cursor: pointer;
    font-family: inherit;
    color: var(--text-normal);
    transition: background 0.2s ease;
  }

  .tier-header:hover {
    background: var(--background-modifier-hover);
  }

  .tier-panel.tier-expanded .tier-header:hover {
    background: color-mix(in srgb, var(--interactive-accent) 10%, var(--background-secondary));
  }

  .tier-header-left {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .tier-name {
    font-size: 1.4em;
    font-weight: 700;
    color: var(--text-normal);
  }

  .tier-badge {
    font-size: 0.75em;
    padding: 4px 12px;
    border-radius: 20px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .badge-free {
    background: var(--background-modifier-success);
    color: var(--text-on-accent);
  }

  .badge-premium {
    background: var(--interactive-accent);
    color: var(--text-on-accent);
  }

  .chevron {
    color: var(--text-muted);
    transition: transform 0.3s ease;
    flex-shrink: 0;
  }

  .chevron-open {
    transform: rotate(180deg);
  }

  .tier-body {
    padding: 0 2rem 1.5rem !important;
  }

  .tier-desc {
    margin: 0 0 1.25rem;
    color: var(--text-muted);
    font-size: 1em;
    line-height: 1.6;
  }

  .tier-btn {
    width: 100% !important;
    padding: 12px 0 !important;
    background: var(--interactive-normal);
    color: var(--text-normal);
    border: none;
    border-radius: var(--radius-m);
    font-size: 0.95em;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.2s ease, color 0.2s ease;
  }

  .tier-btn:hover {
    background: var(--interactive-accent);
    color: var(--text-on-accent);
  }

  .created-banner {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: 12px;
    padding: 12px 16px;
    background: var(--background-primary-alt);
    border: 1px solid var(--interactive-accent);
    border-radius: var(--radius-m);
    animation: banner-in 0.25s ease-out;
  }

  @keyframes banner-in {
    from { opacity: 0; transform: translateY(-6px); }
    to   { opacity: 1; transform: translateY(0); }
  }

  .created-name {
    font-size: 0.9em;
    font-weight: 500;
    color: var(--text-normal);
  }

  .open-btn {
    padding: 6px 16px;
    background: var(--interactive-accent);
    color: var(--text-on-accent);
    border: none;
    border-radius: var(--radius-s);
    font-size: 0.85em;
    font-weight: 600;
    cursor: pointer;
    font-family: inherit;
    transition: opacity 0.15s ease;
  }

  .open-btn:hover {
    opacity: 0.85;
  }
</style>
