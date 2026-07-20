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
