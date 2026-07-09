<script>
  import init, { inicializar_motor } from 'onyx-core';
  // Importamos el binario empaquetado en base64
  import wasmBase64 from 'onyx-core/onyx_core_bg.wasm';

  let engineState = $state('Loading Iron Core...');

  $effect(() => {
    // Decodificamos el base64 a un arreglo de bytes puros
    const wasmBytes = Uint8Array.from(atob(wasmBase64), c => c.charCodeAt(0));

    // Inicializamos el motor pasando los bytes directamente
    init({ module_or_path: wasmBytes }).then(() => {
      engineState = inicializar_motor();
    }).catch(err => {
      engineState = "Core catastrophic failure: " + err;
      console.error(err);
    });
  });
</script>

<div class="onyx-container" style="text-align: center; padding: 20px;">
  <h1 style="color: var(--interactive-accent);">Onyx Ledger</h1>
  <p style="color: var(--text-muted); font-family: monospace; font-size: 1.1em;">
    {engineState}
  </p>
</div>
