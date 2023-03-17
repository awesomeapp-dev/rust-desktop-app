import { DInputElement } from '@dom-native/ui';
import { BaseHTMLElement, customElement, elem, getFirst, html, onEvent, OnEvent } from 'dom-native';
import { Project } from '../bindings/index.js';
import { taskFmc } from '../model/index.js';

const HTML = html`
<header>
<h1></h1>
<d-input class="new-task" placeholder="Enter new task (press enter)"></d-input>
</header>
<d-input class="search-task" placeholder="Search your task"></d-input>
<section></section>
`;

@customElement('project-v')
export class ProjectView extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	#project!: Project
	set project(p: Project) { this.#project = p; this.update(); }
	// #endregion --- Data

	// #region    --- Key Els
	#titleEl!: HTMLElement
	#contentEl!: HTMLElement
	#newTaskDInputEl!: DInputElement
	#searchTaskDInputEl!: DInputElement
	// #endregion --- Key Els

	// #region    --- UI Events
	@onEvent("CHANGE", "d-input.new-task")
	onNewTaskInput(evt: OnEvent) {
		let title = (<DInputElement>evt.selectTarget).value.trim();
		if (title.length > 0) {

			// Create the task
			const project_id = this.#project.id;
			taskFmc.create({ project_id, title });

			// Clear the input
			// Note: Here we could also do an await on create, before clearing the input. 
			//       Or listening the create event back on task (which is debetable).
			this.#newTaskDInputEl.value = '';
		}
	}

	@onEvent("CHANGE", "d-input.search-task")
	onSearchChange(evt: OnEvent) {
		let search = (<DInputElement>evt.selectTarget).value.trim() as string;
		if (search.length > 0) {
			this.update({ title: { $contains: search } });
		} else {
			this.update();
		}
	}

	@onEvent("EMPTY", "tasks-dt")
	onTasksIsEmpty() {
		this.#newTaskDInputEl.focus();
	}
	// #endregion --- UI Events

	init() {
		const content = document.importNode(HTML, true);

		[this.#titleEl, this.#contentEl, this.#newTaskDInputEl, this.#searchTaskDInputEl] =
			getFirst(content, "h1", "section", "d-input.new-task", "d-input.search-task") as [HTMLHeadingElement, HTMLElement, DInputElement, DInputElement];

		this.replaceChildren(content);

		this.update()
	}

	async update(filter?: any) {
		if (this.#contentEl && this.#titleEl) {
			this.#titleEl.textContent = this.#project.name;

			const taskDt = elem('tasks-dt', { $: { project_id: this.#project.id, filter } });
			this.#contentEl.replaceChildren(taskDt);
		}
	}
}
declare global {
	interface HTMLElementTagNameMap {
		'project-v': ProjectView;
	}
}
