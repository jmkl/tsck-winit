// ─────────────────────────────────────────────
// Singleton + Context Helpers

import { getContext, setContext } from 'svelte';
import type { TemplateLine } from '../routes/MainPanel/Thumbnail/stringUtils';
import { DUMMY_TEMPLATELINE } from './temp';
import { TodoHelper, type TodoType } from '../routes/MainPanel/Thumbnail/TodoHelper';

// ─────────────────────────────────────────────
let contextInstance: Apps | null = null;
const MAINCTX = Symbol('AppsContext');

function getInstance() {
	if (!contextInstance) contextInstance = new Apps();
	return contextInstance;
}

export function SetAppsState() {
	setContext(MAINCTX, getInstance());
}

export function GetAppsState() {
	return getContext<Apps>(MAINCTX);
}

export const HERO_PAGE = {
	THUMBNAIL: 0,
	SMARTOBJECT: 1,
	TEXTURES: 2,
	FACERESTORE: 3,
	RAWFILTER: 4,
	YOUTUBETHUMBNAIL: 5,
	LOG: 6,
	CLASSGEN: 7
};

class Apps {
	IsWindowFocus = $state(false);
	globalCompactMode = $state(true);
	thumbnailTypeface = $state<'font-unisans' | 'font-anton'>('font-unisans');
	todoTemplateLines: TemplateLine[] = $state(DUMMY_TEMPLATELINE);
	todoList: TodoType[] = $state([]);
	todoHelper: TodoHelper | undefined = $state();
	showSnippet = $state(true);
	globalActivePage: number = $state(HERO_PAGE.THUMBNAIL);
	constructor() {
		this.todoInit();
	}

	//Todo
	async todoInit() {
		this.todoHelper = new TodoHelper('http://127.0.0.1:6969');
		this.todoUpdate();
	}
	async todoUpdate() {
		if (this.todoHelper) this.todoList = await this.todoHelper?.fetchTodo();
	}
	setWindowFocus(focus: boolean) {
		this.IsWindowFocus = focus;
	}
	resetShadowLayer() {}
}
