<script lang="ts">
  import init, { init_system } from 'onyx-core';
  import wasmBase64 from 'onyx-core/onyx_core_bg.wasm';

  // --- ESTADO (Runas de Svelte 5) ---
  let engineState = $state('Loading Iron Core...');
  let isCoreReady = $state(false);
  
  // Array temporal de prueba (Hasta que conectemos la persistencia física)
  let workspaces = $state([
    { id: '1', name: 'Personal Finances', type: 'Personal', tier: 'Free' },
    { id: '2', name: 'ACME Corp Accounting', type: 'Business', tier: 'Premium' }
  ]);

  // --- INICIALIZACIÓN DEL NÚCLEO WASM ---
  $effect(() => {
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));
    init({ module_or_path: wasmBytes }).then(() => {
      init_system();
      engineState = 'Iron Core ready';
      isCoreReady = true;
    }).catch(err => {
      engineState = "Core catastrophic failure: " + err;
      console.error(err);
    });
  });

  // --- FUNCIONES INTERFAZ ---
  function createWorkspace(type: string, tier: string) {
    if (!isCoreReady) return;
    // Esto es temporal: solo empuja un objeto falso a la vista
    workspaces.push({
      id: crypto.randomUUID(),
      name: `New ${type} Workspace`,
      type: type,
      tier: tier
    });
  }

  function openWorkspace(id: string) {
    console.log(`Abriendo espacio de trabajo: ${id}`);
    // Aquí conectaremos la lógica para cambiar la vista hacia el Ledger seleccionado
  }
</script>

<div class="onyx-container">
  
  <header class="onyx-header">
    <h1 class="onyx-title">Onyx Ledger</h1>
    <button class="onyx-icon-btn" aria-label="Menu" title="Settings">
      <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-menu"><line x1="4" x2="20" y1="12" y2="12"/><line x1="4" x2="20" y1="6" y2="6"/><line x1="4" x2="20" y1="18" y2="18"/></svg>
    </button>
  </header>

  {#if !isCoreReady}
    <div class="onyx-loading">
      <p>{engineState}</p>
    </div>
  {:else}
    
    <div class="onyx-tiers-grid">
      
      <div class="onyx-card">
        <div class="card-header">
          <h2>Personal</h2>
          <span class="badge badge-free">Free</span>
        </div>
        <p class="card-desc">Individual finance tracking with zero-based budgeting.</p>
        <button class="mod-cta" onclick={() => createWorkspace('Personal', 'Free')}>New Workspace</button>
      </div>

      <div class="onyx-card">
        <div class="card-header">
          <h2>Freelance</h2>
          <span class="badge badge-premium">Premium</span>
        </div>
        <p class="card-desc">Contractor management, invoicing, and tax estimations.</p>
        <button class="mod-cta" onclick={() => createWorkspace('Freelance', 'Premium')}>New Workspace</button>
      </div>

      <div class="onyx-card">
        <div class="card-header">
          <h2>Business</h2>
          <span class="badge badge-premium">Premium</span>
        </div>
        <p class="card-desc">Full business accounting suite for multiple entities.</p>
        <button class="mod-cta" onclick={() => createWorkspace('Business', 'Premium')}>New Workspace</button>
      </div>

    </div>

    <div class="onyx-existing-section">
      <h3>Existing Projects</h3>
      {#if workspaces.length === 0}
        <p class="empty-state">No workspaces created yet.</p>
      {:else}
        <div class="onyx-projects-list">
          {#each workspaces as ws}
            <button type="button" class="project-item" onclick={() => openWorkspace(ws.id)}>
              <div class="project-info">
                <h4>{ws.name}</h4>
                <span class="project-meta">{ws.type} • {ws.tier}</span>
              </div>
              <div class="project-action">Open</div>
            </button>
          {/each}
        </div>
      {/if}
    </div>

  {/if}
</div>

<style>
  /* === CSS NATIVO DE OBSIDIAN === */
  
  .onyx-container {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
    font-family: var(--font-interface);
    color: var(--text-normal);
  }

  .onyx-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 2rem;
    border-bottom: 1px solid var(--background-modifier-border);
    padding-bottom: 1rem;
  }

  .onyx-title {
    margin: 0;
    font-size: 2em;
    font-weight: 700;
    color: var(--text-normal);
  }

  .onyx-icon-btn {
    background: transparent;
    box-shadow: none;
    border: none;
    color: var(--text-muted);
    cursor: pointer;
    padding: 8px;
    border-radius: var(--radius-s);
    transition: color 0.2s ease, background 0.2s ease;
  }

  .onyx-icon-btn:hover {
    color: var(--text-normal);
    background: var(--background-modifier-hover);
  }

  .onyx-loading {
    text-align: center;
    padding: 3rem;
    color: var(--text-muted);
    font-family: var(--font-monospace);
  }

  /* --- Cuadrícula de Tarjetas --- */
  .onyx-tiers-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1.5rem;
    margin-bottom: 3rem;
  }

  .onyx-card {
    background-color: var(--background-secondary);
    border: 1px solid var(--background-modifier-border);
    border-radius: var(--radius-l);
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    transition: border-color 0.2s ease, transform 0.1s ease;
  }

  .onyx-card:hover {
    border-color: var(--interactive-accent);
    /* transform: translateY(-2px); Opcional: pequeño salto al pasar el mouse */
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 1rem;
  }

  .card-header h2 {
    margin: 0;
    font-size: 1.25em;
  }

  .badge {
    font-size: 0.75em;
    padding: 2px 8px;
    border-radius: var(--radius-s);
    font-weight: 600;
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

  .card-desc {
    color: var(--text-muted);
    font-size: 0.9em;
    line-height: 1.4;
    flex-grow: 1; /* Empuja el botón hacia abajo */
    margin-bottom: 1.5rem;
  }

  /* --- Lista de Proyectos Existentes --- */
  .onyx-existing-section {
    border-top: 1px solid var(--background-modifier-border);
    padding-top: 2rem;
  }

  .onyx-existing-section h3 {
    margin-top: 0;
    margin-bottom: 1rem;
    font-size: 1.2em;
    color: var(--text-normal);
  }

  .empty-state {
    color: var(--text-muted);
    font-style: italic;
  }

  .onyx-projects-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .project-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    background-color: var(--background-primary-alt);
    border: 1px solid var(--background-modifier-border);
    border-radius: var(--radius-m);
    padding: 1rem 1.5rem;
    cursor: pointer;
    text-align: left;
    width: 100%;
    transition: background-color 0.2s ease, border-color 0.2s ease;
  }

  .project-item:hover {
    background-color: var(--background-modifier-hover);
    border-color: var(--interactive-accent);
  }

  .project-info h4 {
    margin: 0;
    font-size: 1.1em;
    color: var(--text-normal);
  }

  .project-meta {
    font-size: 0.85em;
    color: var(--text-muted);
  }

  .project-action {
    font-size: 0.9em;
    font-weight: 600;
    color: var(--interactive-accent);
  }
</style>
