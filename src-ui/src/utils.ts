

/** 
 * Narrow utility to make a string css class "compatible". 
 * TODO: Need to make it more exhaustive.
 */
export function classable(str: string): string {
	return str.replace(":", "_");
}