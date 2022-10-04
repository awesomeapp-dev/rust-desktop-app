import { BaseHTMLElement, customElement, elem, onDoc, onEvent, OnEvent, trigger } from 'dom-native';

type Options = { [k: string]: string | HTMLElement };


@customElement('menu-c')
export class MenuComponent extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	// This data is disposable, no need to keep, and the key is stored as children attribute.
	set options(v: Options) { this.update(v) }
	// #endregion --- Data

	// #region    --- UI Events
	@onEvent('pointerup', 'li')
	onLiClick(evt: OnEvent) {
		const key = evt.selectTarget.getAttribute("data-key");
		trigger(this, "SELECT", { detail: key });
		this.remove();
	}

	@onDoc('pointerup', { nextFrame: true })
	onDocClick(evt: Event) {
		if (!this.contains(evt.target as Node)) {
			this.remove();
		}
	}
	// #endregion --- UI Events

	// Note: For this component, no need to check if same data, just refresh.
	//       And the key is stored in the data-key, so, nothing else to store. 
	//       Less is simpler.
	//       The frozen is not really needed here as we do not store it.
	//       However, just for consistency.
	update(options: Options) {
		// and replace the content
		const els = Object.entries(options).map(([k, v]) => {
			const el = elem('li', { "data-key": k });
			if (typeof v == "string") {
				el.textContent = v;
			} else {
				el.appendChild(v);
			}
			return el;
		});
		this.replaceChildren(...els);
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'menu-c': MenuComponent;
	}
}
