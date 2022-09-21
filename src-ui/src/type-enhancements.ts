// NOTE - Some TypeScript type enhancements. 
//        Does not have to be imported anywhere, TypeScript will pick it up.

export { }; // make this file a module

declare global {
  // Cloning a DocumentFragment returns a DocumentFragment
  // Note: This is not needed in this code based as we use importNode, 
  //       but just as an example.
  interface DocumentFragment {
    cloneNode(deep?: boolean): DocumentFragment;
  }
}