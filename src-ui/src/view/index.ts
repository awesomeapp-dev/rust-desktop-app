//! index of all of the views and components.
//!
//! Notes:
//!   - The responsibility of this module is to export/import all of the sub views/components
//!     as they must be preloaded to activate the HTML custom elements.
//!   - This file will be imported by `main.ts` without the need to know the specifics of the views and components.
//!   - Component notation follows "[domain_space]-[component_type]" where the domain_space is the entity or function of the components,
//!     for example, `app` or `tasks`, and the component_type reflect the type of the component, such as `v` for **view** or `c` for **component**
//!     or `dt` for **data table**.
//! 
//! The differences between "Views" and "Components" are more on the semantic side than implementations
//!   - Views are a bigger part of the application, usually big composites of components and light elements. 
//!     Typically manage the UI Events, Model Events, and routing as needed.
//!   - Components are smaller UI Elements, usually not model specific. They are designed to be as data "unintelligent" as possible. 
//!   - Composites are between views and components and tend to be used for medium sized reusable system parts. Like a "Task Data Table" (e.g., `tasks-dt`)
//!

export * from './app-v.js';
export * from './menu-c.js';
export * from './nav-v.js';
export * from './project-v.js';
export * from './tasks-dt.js';




