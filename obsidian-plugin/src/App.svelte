<script>
  import init, { init_system, create_workspace } from 'onyx-core';
  import wasmBase64 from 'onyx-core/onyx_core_bg.wasm';

  let engineReady = $state(false);
  let engineError = $state(null);
  let workspaces = $state([]);
  let currentScreen = $state('dashboard');
  let activeWorkspaceId = $state(null);

  $effect(() => {
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    init({ module_or_path: wasmBytes })
      .then(() => {
        init_system();
        engineReady = true;
      })
      .catch(err => {
        engineError = err;
        console.error(err);
      });
  });

  function handleAddWorkspace(type) {
    const name = type + ' Project';
    const id = create_workspace(name, type);
    workspaces = [...workspaces, { id, name, type }];
    activeWorkspaceId = id;
  }

  function handleOpenWorkspace(id) {
    activeWorkspaceId = id;
    currentScreen = 'workspace';
  }

  const tiers = [
    { type: 'Personal', label: 'Free', desc: 'Individual finance tracking' },
    { type: 'Freelance', label: 'Plus', desc: 'Contractor & freelance management' },
    { type: 'Business', label: 'Premium', desc: 'Full business accounting suite' },
  ];
</script>

{#if !engineReady}
  <div class="onyx-loading">
    <div class="onyx-spinner"></div>
  </div>
{:else if engineError}
  <div class="onyx-error">
    <h2>Core initialization failed</h2>
    <p>{engineError.message ?? engineError}</p>
  </div>
{:else}
  <div class="onyx-app">
    <header class="onyx-header">
      <h1 class="onyx-title">Onyx Ledger</h1>
      <button class="hamburger" aria-label="Menu">
        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round">
          <line x1="4" y1="6" x2="20" y2="6"/>
          <line x1="4" y1="12" x2="20" y2="12"/>
          <line x1="4" y1="18" x2="20" y2="18"/>
        </svg>
      </button>
    </header>

    <section class="tier-section">
      <div class="tier-grid">
        {#each tiers as tier}
          <div class="tier-card">
            <div class="tier-card-head">
              <span class="tier-type">{tier.type}</span>
              <span class="tier-badge">{tier.label}</span>
            </div>
            <p class="tier-desc">{tier.desc}</p>
            <button class="tier-btn" onclick={() => handleAddWorkspace(tier.type)}>
              New Workspace
            </button>
          </div>
        {/each}
      </div>
    </section>

    {#if workspaces.length > 0}
      <section class="projects-section">
        <h2 class="section-title">Existing Projects</h2>
        <div class="projects-grid">
          {#each workspaces as ws}
            <div class="project-card" onclick={() => handleOpenWorkspace(ws.id)} role="button" tabindex="0">
              <div class="project-info">
                <span class="project-name">{ws.name}</span>
                <span class="project-type">{ws.type}</span>
              </div>
              <button class="open-btn" onclick={(e) => { e.stopPropagation(); handleOpenWorkspace(ws.id); }}>
                Open
              </button>
            </div>
          {/each}
        </div>
      </section>
    {/if}
  </div>
{/if}

<style>
  .onyx-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 200px;
  }

  .onyx-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--background-modifier-border);
    border-top-color: var(--interactive-accent);
    border-radius: 50%;
    animation: onyx-spin 0.8s linear infinite;
  }

  @keyframes onyx-spin {
    to { transform: rotate(360deg); }
  }

  .onyx-error {
    text-align: center;
    padding: 2rem;
    color: var(--text-muted);
  }

  .onyx-error h2 {
    color: var(--text-error, var(--text-normal));
    margin-bottom: 0.5rem;
  }

  .onyx-app {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  .onyx-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
  }

  .onyx-title {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-normal);
    margin: 0;
    letter-spacing: -0.02em;
  }

  .hamburger {
    display: flex;
    align-items: center;
    justify-content: center;
    background: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 4px;
    border-radius: 6px;
    transition: color 0.15s, background 0.15s;
  }

  .hamburger:hover {
    color: var(--text-normal);
    background: var(--background-modifier-hover);
  }

  .tier-section {
    margin-bottom: 2rem;
  }

  .tier-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 20px;
  }

  .tier-card {
    display: flex;
    flex-direction: column;
    background: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: 12px;
    padding: 24px;
    transition: border-color 0.2s;
  }

  .tier-card:hover {
    border-color: var(--interactive-accent);
  }

  .tier-card-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 8px;
  }

  .tier-type {
    font-weight: 600;
    font-size: 1.05rem;
    color: var(--text-normal);
  }

  .tier-badge {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--interactive-accent);
    background: color-mix(in srgb, var(--interactive-accent) 14%, transparent);
    padding: 2px 10px;
    border-radius: 20px;
    text-transform: uppercase;
    letter-spacing: 0.03em;
  }

  .tier-desc {
    font-size: 0.85rem;
    color: var(--text-muted);
    margin: 0 0 auto 0;
    line-height: 1.5;
  }

  .tier-btn {
    margin-top: 16px;
    padding: 10px 0;
    width: 100%;
    background: var(--interactive-normal);
    color: var(--text-normal);
    border: none;
    border-radius: 8px;
    font-size: 0.85rem;
    font-weight: 600;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
    font-family: inherit;
    text-align: center;
  }

  .tier-btn:hover {
    background: var(--interactive-accent);
    color: var(--text-on-accent, #fff);
  }

  .projects-section {
    border-top: 1px solid var(--background-modifier-border);
    padding-top: 1.5rem;
    margin-top: 0.5rem;
  }

  .section-title {
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin: 0 0 1rem 0;
  }

  .projects-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
    gap: 12px;
  }

  .project-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 14px 16px;
    background: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: 10px;
    cursor: pointer;
    transition: border-color 0.15s;
  }

  .project-card:hover {
    border-color: var(--interactive-accent);
  }

  .project-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .project-name {
    font-weight: 500;
    font-size: 0.9rem;
    color: var(--text-normal);
  }

  .project-type {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .open-btn {
    padding: 6px 14px;
    background: var(--interactive-normal);
    color: var(--text-normal);
    border: none;
    border-radius: 6px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    transition: background 0.15s, color 0.15s;
    font-family: inherit;
  }

  .open-btn:hover {
    background: var(--interactive-accent);
    color: var(--text-on-accent, #fff);
  }
</style>
