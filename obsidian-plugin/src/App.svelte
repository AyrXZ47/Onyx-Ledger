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
  /* === CONTENEDOR GLOBAL === */
  .onyx-container {
    width: 100% !important;
    max-width: 800px !important;
    margin: 0 auto !important;
    padding: 2rem 1rem !important;
    font-family: var(--font-interface);
    color: var(--text-normal);
    box-sizing: border-box;
  }

  /* === CABECERA === */
  .onyx-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 3rem !important;
    border-bottom: 2px solid var(--background-modifier-border);
    padding-bottom: 1rem;
  }

  .onyx-title {
    margin: 0;
    font-size: 2.2em;
    font-weight: 800;
    letter-spacing: -0.5px;
    color: var(--text-normal);
  }

  .onyx-icon-btn {
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    color: var(--text-muted);
    transition: color 0.2s ease;
  }
  .onyx-icon-btn:hover {
    color: var(--text-normal);
  }

  /* === LOADING STATE === */
  .onyx-loading {
    text-align: center;
    padding: 2rem;
    color: var(--text-muted);
  }

  /* === LAYOUT DE PANELES === */
  .tier-row {
    display: flex;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .tier-panel {
    flex: 1 1 200px;
    background: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: var(--radius-l);
    overflow: hidden;
    transition: border-color 0.2s ease, opacity 0.3s ease, transform 0.2s ease;
    display: flex;
    flex-direction: column;
  }

  .tier-panel.tier-expanded {
    border-color: var(--interactive-accent);
    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
    transform: scale(1.02);
  }

  .tier-panel.tier-dimmed {
    opacity: 0.5;
    pointer-events: none;
  }

  /* === CABECERA DEL PANEL (Reemplaza a <summary>) === */
  .tier-header {
    padding: 1.5rem !important;
    font-size: 1.2em;
    font-weight: 600;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    background: var(--background-primary-alt);
    border: none;
    width: 100%;
    text-align: left;
    margin: 0;
    transition: background 0.2s ease;
  }
  .tier-header:hover {
    background: var(--background-modifier-hover);
  }

  .tier-header-left {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .tier-name {
    font-weight: 600;
  }

  /* === CUERPO DEL PANEL (Reemplaza a .accordion-content) === */
  .tier-body {
    padding: 1.5rem !important;
    border-top: 1px solid var(--background-modifier-border);
    background: var(--background-secondary);
    line-height: 1.6;
    color: var(--text-muted);
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .tier-desc {
    margin: 0;
    font-size: 0.95em;
  }

  /* === CHEVRON ICON === */
  .chevron {
    transition: transform 0.3s ease;
  }
  .chevron.chevron-open {
    transform: rotate(180deg);
  }

  /* === ETIQUETAS (Reemplaza a .badge) === */
  .tier-badge {
    font-size: 0.75em;
    padding: 4px 10px;
    border-radius: 12px;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }
  
  .badge-free {
    background-color: var(--background-modifier-success);
    color: var(--text-on-accent);
  }

  .badge-premium {
    background-color: var(--interactive-accent);
    color: var(--text-on-accent);
  }

  /* === BOTONES PRINCIPALES (Reemplaza a button.mod-cta) === */
  .tier-btn, 
  .open-btn {
    margin-top: 1rem;
    width: 100%;
    padding: 0.75rem 1.5rem !important;
    font-size: 1em;
    font-weight: 600;
    border-radius: var(--radius-m);
    background-color: var(--interactive-accent);
    color: var(--text-on-accent);
    border: none;
    cursor: pointer;
    transition: opacity 0.2s ease, transform 0.1s ease;
  }

  .tier-btn:hover, 
  .open-btn:hover {
    opacity: 0.9;
  }

  /* === BANNER DE CREACIÓN === */
  .created-banner {
    margin-top: 1rem;
    padding: 1rem;
    background: var(--background-modifier-success);
    border-radius: var(--radius-m);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }

  .created-name {
    font-weight: 600;
    color: var(--text-normal);
  }

  .open-btn {
    margin-top: 0 !important;
    width: auto !important;
    padding: 0.5rem 1rem !important;
  }
</style>
