import { app, constants } from "photoshop";
import { storage } from "uxp";
import { Bounds } from "@tsockee/frontend";
import { appendLinkedObject } from "./appendLinkedObject";
import { modalExecutor } from "./modalExecutor";

export async function appendLinkedObjectResize(
	entry: storage.Entry,
	selection_bounds: Bounds,
) {
	let resize_me = selection_bounds.right - selection_bounds.left > 0;
	await appendLinkedObject(entry);

	let sourceheight = 720;
	if (resize_me) {
		sourceheight = selection_bounds.bottom - selection_bounds.top;
	}
	const newlayer = app.activeDocument.activeLayers[0];
	const layer = newlayer.boundsNoEffects;
	const curheight = layer.height;
	let percentage = (sourceheight / curheight) * 100;
	await modalExecutor("", async () => {
		await app.activeDocument.activeLayers[0].scale(
			percentage,
			percentage,
			constants.AnchorPosition.MIDDLECENTER,
		);
	});

	await modalExecutor("", async () => {
		const layer = app.activeDocument.activeLayers[0];
		const t = layer.boundsNoEffects.top;
		const l = layer.boundsNoEffects.left;
		layer.translate(selection_bounds.left - l, selection_bounds.top - t);
	});
}
