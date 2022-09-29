//! Main Application View which will initialize the application and display the appropriate 
//!
//! Notes:
//!   - Will listen to Route.change event, and update the main view
//!   - The Nav View `nav-v` will manage it's routing update.
//!
//! TODO: Needs to implement the menu click (min-nav action)
//!

import { BaseHTMLElement, customElement, elem, first, getFirst, html, onEvent, onHub } from 'dom-native';
import { projectFmc } from '../model';
import { router } from '../router';

// dom-native JS Tagged templates to create a DocumentFragment (parse once)
const HTML = html`
  <header>
  <d-ico class="menu action" name="ico-menu"></d-ico>
  <h1>Awesome App</h1>
  </header>
  <nav-v>nav-v</nav-v>
  <main>MAIN</main>
`

@customElement('app-v')
export class AppView extends BaseHTMLElement { // extends HTMLElement
  #mainEl!: HTMLElement

  // #region    --- App Events
  @onHub("Route", "change") // @onHub(hubName, topic, label?)
  async onRouteChange() {
    const { project_id } = router.get_current();
    if (project_id != null) {
      const project = await projectFmc.get(project_id);
      const projectEl = elem('project-v', { $: { project } });
      this.#mainEl.replaceChildren(projectEl);
    } else {
      this.#mainEl.textContent = "Welcome select project";
    }
  }
  // #endregion --- App Events


  // #region    --- UI Events
  @onEvent("pointerup", "header > c-ico.menu") // @onEvent(eventType, elementSelectorFromThis)
  onMenuClick(evt: PointerEvent) {
    this.classList.toggle("min-nav");
  }
  // #endregion --- UI Events


  init() { // Will be called by BaseHTMLElement once on first connectedCallback
    // clone the HTML documentFragment and get the key elements (to be used later)
    let content = document.importNode(HTML, true);

    this.#mainEl = getFirst(content, "main");

    // beautify the header h1
    const h1 = first(content, 'header > h1');
    if (h1) {
      if (h1.firstElementChild == null) {
        const text = h1.textContent?.split(/[-_ ](.+)/) ?? ["NO", "NAME"];
        h1.replaceChildren(html`<span>${text[0]}</span><span class="prime">${text[1]}</span>`)
      }
    }

    // replace the children
    this.replaceChildren(content);
  }

}
declare global { // trick to augment the global TagName with this component
  interface HTMLElementTagNameMap {
    'app-v': AppView;
  }
}
