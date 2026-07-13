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
    </header>

    <section class="tier-section">
      <div class="tier-cards">
        {#each tiers as tier}
          <button class="tier-card" onclick={() => handleAddWorkspace(tier.type)}>
            <span class="tier-indicator tier-{tier.type.toLowerCase()}"></span>
            <span class="tier-type">{tier.type}</span>
            <span class="tier-label">{tier.label}</span>
            <span class="tier-desc">{tier.desc}</span>
          </button>
        {/each}
      </div>
    </section>

    <section class="action-section">
      <div class="add-workspace-group">
        {#each tiers as tier}
          <button
            class="add-btn"
            onclick={() => handleAddWorkspace(tier.type)}
          >
            New {tier.type} Workspace
          </button>
        {/each}
      </div>
    </section>

    {#if workspaces.length > 0}
      <section class="projects-section">
        <h2 class="section-title">Existing Projects</h2>
        <div class="projects-list">
          {#each workspaces as ws}
            <div class="project-item">
              <div class="project-info">
                <span class="project-name">{ws.name}</span>
                <span class="project-type">{ws.type}</span>
              </div>
              <button class="open-btn" onclick={() => handleOpenWorkspace(ws.id)}>
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
    padding: 1.5rem;
    max-width: 640px;
    margin: 0 auto;
  }

  .onyx-header {
    text-align: center;
    margin-bottom: 2rem;
  }

  .onyx-title {
    font-size: 1.75rem;
    font-weight: 700;
    color: var(--text-normal);
    margin: 0;
    letter-spacing: -0.02em;
  }

  .tier-section {
    margin-bottom: 1.5rem;
  }

  .tier-cards {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .tier-card {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 1rem 1.25rem;
    background: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: 8px;
    cursor: pointer;
    transition: border-color 0.15s, box-shadow 0.15s;
    text-align: left;
    font-family: inherit;
    color: var(--text-normal);
    width: 100%;
  }

  .tier-card:hover {
    border-color: var(--interactive-accent);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.1);
  }

  .tier-indicator {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .tier-personal {
    background: #84cc16;
  }

  .tier-freelance {
    background: #eab308;
  }

  .tier-business {
    background: #3b82f6;
  }

  .tier-type {
    font-weight: 600;
    font-size: 0.95rem;
    min-width: 80px;
  }

  .tier-label {
    font-size: 0.8rem;
    font-weight: 500;
    color: var(--interactive-accent);
    background: color-mix(in srgb, var(--interactive-accent) 12%, transparent);
    padding: 0.15rem 0.5rem;
    border-radius: 4px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .tier-desc {
    font-size: 0.8rem;
    color: var(--text-muted);
    margin-left: auto;
    display: none;
  }

  @media (min-width: 500px) {
    .tier-desc {
      display: inline;
    }
  }

  .action-section {
    margin-bottom: 1.5rem;
  }

  .add-workspace-group {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .add-btn {
    padding: 0.7rem 1.25rem;
    background: var(--interactive-accent);
    color: var(--text-on-accent, #fff);
    border: none;
    border-radius: 6px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: opacity 0.15s;
    font-family: inherit;
  }

  .add-btn:hover {
    opacity: 0.85;
  }

  .projects-section {
    margin-top: 0.5rem;
  }

  .section-title {
    font-size: 0.85rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-muted);
    margin: 0 0 0.75rem 0;
  }

  .projects-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .project-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    background: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: 6px;
  }

  .project-info {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
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
    padding: 0.4rem 0.9rem;
    background: var(--interactive-accent);
    color: var(--text-on-accent, #fff);
    border: none;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 500;
    cursor: pointer;
    transition: opacity 0.15s;
    font-family: inherit;
  }

  .open-btn:hover {
    opacity: 0.85;
  }
</style>
