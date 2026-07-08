import { ItemView } from 'obsidian';
import App from './App.svelte';

export const VIEW_TYPE_ONYX = 'onyx-ledger-view';

export class OnyxView extends ItemView {
  component: any;

  constructor(leaf: WorkspaceLeaf) {
    super(leaf);
  }

  getViewType() {
    return VIEW_TYPE_ONYX;
  }

  getDisplayText() {
    return 'Onyx Ledger';
  }

  async onOpen() {
    this.component = new App({ target: this.contentEl });
  }

  onClose() {
    this.component.$destroy();
  }
}
