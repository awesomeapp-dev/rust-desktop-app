// NOTE - Some TypeScript type enhancements. 
//        Does not have to be imported anywhere, TypeScript will pick it up.

export { }; // make this file a module

declare global {
	// Cloning a DocumentFragment returns a DocumentFragment
	// Note: This is not needed in this code base as we use importNode, 
	//       but this is showing how the global types can be extends/tuned.
	interface DocumentFragment {
		cloneNode(deep?: boolean): DocumentFragment;
	}
}