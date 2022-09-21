import { hub } from 'dom-native';


const route_hub = hub("Route");

/** Flat based route state */
interface Route {
  project_id?: string
}

class Router {

  #current_route: Route = {};

  update_state(state: Route) {
    Object.assign(this.#current_route, state);
    route_hub.pub("change", null);
  }

  get_current(): Route {
    // clone for safety (shallow enough as route is designed to be flat)
    return { ...this.#current_route };
  }


}

export const router = new Router();