import '@dom-native/ui';
import { loadDefaultIcons } from '@dom-native/ui';
import { html } from 'dom-native';
import './event.js';
import { SYMBOLS } from './svg-symbols.js';

// important, this will load the customElements
import './view/index.js';

// load the default icons from @dom-native/ui
loadDefaultIcons();

// --- Initialize some assets on DOMContentLoaded
document.addEventListener("DOMContentLoaded", async function (event) {

	// Append the app custom icons 
	// (similar to what loadDefaultIcons does for @dom-native/ui icons)
	// (this allows to use the <use xlink:href="#symbol_id" ...> and update fill from css)
	const svgEl = html(SYMBOLS).firstElementChild!;
	svgEl.setAttribute('style', 'display: none'); // in case dom engine move it to body
	document.head.appendChild(svgEl);
});


