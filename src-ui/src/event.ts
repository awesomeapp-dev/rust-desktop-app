import { Event as TauriEvent, listen } from '@tauri-apps/api/event';
import { hub } from 'dom-native';
import type { HubEvent } from './bindings/index.js';

// --- Bridge Tauri HubEvent events to dom-native hub/pub/sub event
//     (optional, but allows to use hub("Data").sub(..) or
//      @onHub("Data", topic, label) on BaseHTMLElement custom elements)
listen("HubEvent", function (evt: TauriEvent<HubEvent<any>>) {
	const hubEvent = evt.payload;

	// Get or create the Hub by name (from dom-native)
	//   (a Hub is a event bus namespace silo)
	let _hub = hub(hubEvent.hub);

	// Publish event to the given Hub
	if (hubEvent.label != null) {
		_hub.pub(hubEvent.topic, hubEvent.label, hubEvent.data);
	} else {
		_hub.pub(hubEvent.topic, hubEvent.data);
	}
})