import { hub } from 'dom-native';

const route_hub = hub("Route");

/** 
 * Route states for the whole application. 
 * 
 * Currently, the best practice is to keep the Route states as simple
 * as possible, meaning, flat and just "ids" like names/values.
 * 
 **/
interface Route {
	project_id?: string
}

class Router {

	#current_route: Route = {};

	update_state(state: Partial<Route>) {
		// Note: DeepClone when Route state cannot be assumed to be flat anymore.
		Object.assign(this.#current_route, state);
		route_hub.pub("change", null);
	}

	get_current(): Route {
		// clone for safety (shallow enough as route is designed to be flat)
		return { ...this.#current_route };
	}


}

export const router = new Router();