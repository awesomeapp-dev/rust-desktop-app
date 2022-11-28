import { DCheckElement } from '@dom-native/ui';
import { all, BaseHTMLElement, customElement, elem, first, frag, html, on, OnEvent, onEvent, onHub, position, scanChild, trigger } from 'dom-native';
import { ModelMutateResultData, Task } from '../bindings/index.js';
import { taskFmc } from '../model/index.js';
import { classable } from '../utils.js';

const TASK_HEADER = html`
	<div class="th">Title </div>
	<div class="th">Info</div>
	<div class="th done">Done</div>
	<div class="th more">&nbsp;</div>
`

const TASK_ROW_HTML = html`
	<span class="title"></span>
	<span class="info"></span>
	<d-check class="done"></d-check>
	<d-ico class="show-more" name="ico-more"></d-ico>
`;

@customElement('tasks-dt')
export class TasksDataTable extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	#project_id!: string;
	set project_id(v: string) { this.#project_id = v; this.update() }

	#filter?: any
	set filter(f: any) { this.#filter = f; this.update() }
	// #endregion --- Data

	// #region    --- App Event
	// Create will refresh the full datagrid, in case of sort by name and such
	@onHub("Model", "task", "create")
	onTaskCreate() {
		this.update();
	}

	// Delete can be more selective in this case, will delete the row
	@onHub("Model", "task", "delete")
	onTaskDelete(data: ModelMutateResultData) {
		all(this, `task-row.${classable(data.id)}`).forEach(taskRowEl => {
			// Note: This will add the class in the taskRow, but the animations are on the cells
			//       as the task-row as the display: contents in the css 
			//       (to be transparent to the grid layout, hence, can't style it)
			taskRowEl.classList.add('anim-delete');

			// Note: Trick to start the dom deletion before the animation terminate to make it snapier 
			setTimeout(() => {
				taskRowEl.remove();
			}, 100);


			// Note: This is sementically correct way to delete it, on first transition end. 
			// taskRowEl.addEventListener('transitionend', (evt) => {
			//   // Note: Here we will get many events back (one per animated element and property)
			//   //       So, just delete on first.
			//   if (taskRowEl.isConnected) {
			//     taskRowEl.remove()
			//   }
			// });
		});
	}

	@onHub("Model", "task", "update")
	async onTaskUpdate(data: ModelMutateResultData) {
		const newTask = await taskFmc.get(data.id);
		all(this, `task-row.${classable(data.id)}`).forEach((taskEl) => (<TaskRow>taskEl).task = newTask);
	}
	// #endregion --- App Event

	// #region    --- UI Events
	@onEvent("pointerup", "task-row .show-more")
	onTaskShowMore(evt: OnEvent) {
		const MENU_CLASS = 'task-row-more-menu';

		// if already showing (will auto remove, but we do not want to popup it again)
		if (first(`body > menu-c.${MENU_CLASS}`)) return;

		const showMoreEl = evt.selectTarget;
		const task = showMoreEl.closest('task-row')!.task;

		const options = {
			'toggle': (task.done) ? "Mark Undone" : "Mark Done",
			'delete': elem("label", { class: "delete", $: { textContent: "Delete" } }),
		};

		// Show the meunu
		const menuEl = elem("menu-c", { "class": MENU_CLASS, $: { options } });
		document.body.appendChild(menuEl);
		on(menuEl, "SELECT", (evt: OnEvent<keyof typeof options>) => {
			if (evt.detail == 'delete') {
				taskFmc.delete(task.id);
			} else if (evt.detail == 'toggle') {
				taskFmc.update(task.id, { done: !task.done });
			}

		});
		position(menuEl, showMoreEl, { refPos: "BR", pos: "BL", gap: 4 });
	}

	@onEvent("CHANGE", "task-row d-check")
	onTaskCheckClick(evt: OnEvent<{ value: boolean }>) {
		let taskEl = evt.selectTarget.closest("task-row")!;
		let task_id = taskEl.task.id;
		let newDone = evt.detail.value;

		// Make sure to avoid infine loop 
		// (will get this event when changed by other mean as well)
		if (newDone !== taskEl.task.done) {
			taskFmc.update(task_id, { done: evt.detail.value });
		}
	}
	// #endregion --- UI Events

	postDisplay() {
		this.update();
	}

	async update() {
		if (this.initialized) {
			const filter = {
				project_id: this.#project_id,
				...this.#filter
			}
			const tasks = await taskFmc.list(filter);

			const content = frag(tasks, task => elem('task-row', { $: { task } }));

			content.prepend(document.importNode(TASK_HEADER, true));

			this.replaceChildren(content);

			if (tasks.length == 0) {
				trigger(this, "EMPTY");
			}
		}

	}
}
declare global {
	interface HTMLElementTagNameMap {
		'tasks-dt': TasksDataTable;
	}
}

// #region    --- task-row
@customElement('task-row')
export class TaskRow extends BaseHTMLElement { // extends HTMLElement
	// #region    --- Data
	#task!: Task;
	set task(newTask: Task) {
		const oldTask = this.#task as Task | undefined;
		if (oldTask !== newTask) {
			this.#task = newTask;
			this.update(newTask, oldTask);
		}
	}
	get task() { return this.#task }
	// #endregion --- Data

	// #region    --- Key Els
	#checkEl!: DCheckElement;
	#titleEl!: HTMLElement;
	#infoEl!: HTMLElement;
	// #endregion --- Key Els

	init() {

		super.init();
		let content = document.importNode(TASK_ROW_HTML, true);
		// Note: dom-native scanChild is a strict one fast pass child scanner. 
		//       Use all/first if needs to be more flexible. 
		[this.#titleEl, this.#infoEl, this.#checkEl] = scanChild(content, 'span', 'span', 'd-check');

		// FIXME: Check that order does not matter here.
		this.replaceChildren(content);
		this.update(this.#task);
	}

	update(newTask: Task, oldTask?: Task) {

		if (oldTask) {
			this.classList.remove(`${classable(oldTask.id)}`)
		}

		// if ready to be injected, we do the job
		if (newTask && this.#titleEl != null) {

			this.classList.add(`${classable(newTask.id)}`);
			this.#checkEl.checked = newTask.done;

			this.#titleEl.textContent = newTask.title;
			let info = newTask.ctime;
			info = info.substring(info.length - 5);
			this.#infoEl.textContent = `(ctime: ${info})`;
		}

	}
}
declare global {
	interface HTMLElementTagNameMap {
		'task-row': TaskRow;
	}
}
// #endregion --- task-row

