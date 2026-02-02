import { storage } from "uxp";
import ScriptExecutor from "../utils/script-executor";
export class PhotoshopHelper extends EventTarget {
	scriptExecutor: ScriptExecutor;
	rootFolder: storage.Entry | undefined;
	constructor() {
		super();
		this.scriptExecutor = new ScriptExecutor();
	}
	initRootFolder() {}
}
