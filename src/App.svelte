<script lang="ts">
  import { onMount } from 'svelte';
  import { inicializar_motor } from 'onyx-core';

  let estadoMotor: string = $state('Cargando núcleo...');

  onMount(async () => {
    const resultado = await inicializar_motor();
    estadoMotor = resultado;
  });
</script>

<main>
  <h1>Sistemas en línea. Motor a la espera.</h1>
  <p>{$estadoMotor}</p>
</main>
