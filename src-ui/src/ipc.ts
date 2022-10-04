import { invoke } from "@tauri-apps/api";
import { deepFreeze } from 'utils-min';

/** 
 * Small wrapper on top of tauri api invoke
 * 
 * best-practice: Light and narrow external api abstraction. 
 */
export async function ipc_invoke(method: string, params?: object): Promise<any> {
	const response: any = await invoke(method, { params });
	if (response.error != null) {
		console.log('ERROR - ipc_invoke - ipc_invoke error', response);
		throw new Error(response.error);
	} else {
		return deepFreeze(response.result);
	}
}
