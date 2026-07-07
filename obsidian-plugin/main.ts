import { Plugin } from 'obsidian';

export default class OnyxLedgerPlugin extends Plugin {
  onload() {
    console.log('Onyx Ledger inicializado');
    this.addCommand({
      id: 'abrir-panel-onyx',
      name: 'Abrir Panel de Onyx',
      callback: () => {
        new Notice('Interfaz de Svelte se está construyendo.');
      }
    });
  }
}
