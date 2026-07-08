import { Plugin } from 'obsidian';
import { VIEW_TYPE_ONYX, OnyxView } from './OnyxView';

export default class OnyxLedgerPlugin extends Plugin {
  async onload() {
    console.log('Onyx Ledger inicializado');

    this.registerView(
      VIEW_TYPE_ONYX,
      (leaf) => new OnyxView(leaf)
    );

    this.addCommand({
      id: 'abrir-panel-onyx',
      name: 'Abrir Panel de Onyx',
      callback: () => {
        this.activateView();
      }
    });
  }

  async activateView() {
    const { workspace } = this.app;
    
    let leaf: any = null;
    const leaves = workspace.getLeavesOfType(VIEW_TYPE_ONYX);

    if (leaves.length > 0) {
      leaf = leaves[0];
    } else {
      leaf = workspace.getLeaf('tab');
      await leaf.setViewState({ type: VIEW_TYPE_ONYX, active: true });
    }

    workspace.revealLeaf(leaf);
  }
}
