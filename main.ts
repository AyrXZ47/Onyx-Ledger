import { Plugin, Notice } from 'obsidian';
import { VIEW_TYPE_ONYX, OnyxView } from './OnyxView';

export default class OnyxLedgerPlugin extends Plugin {
  onload() {
    console.log('Onyx Ledger inicializado');
    this.registerView(VIEW_TYPE_ONYX, (leaf) => new OnyxView(leaf));

    this.addCommand({
      id: 'abrir-panel-onyx',
      name: 'Abrir Panel de Onyx',
      callback: async () => {
        await this.activateView();
      }
    });
  }

  async activateView() {
    const leaves = this.app.workspace.getLeavesOfType(VIEW_TYPE_ONYX);
    if (leaves.length === 0) {
      const leaf = this.app.workspace.getLeaf('tab');
      await leaf.setViewState({ type: VIEW_TYPE_ONYX });
    }
    this.app.workspace.revealLeaf(leaves[0]);
  }
}
