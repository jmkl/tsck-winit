import { action, app } from "photoshop";
import { getTokenHelperState } from "../../token/tokenContext";
import { Template, TemplateLine } from "@tsockee/frontend";
import { logger } from "../addLog";
import { FontAnton, fontHeight, FontUniSans } from "../dataset";
import { tryCatchAsync } from "../sandbox";
import { setLoadingState } from "../setLoadingState";
import { appendLinkedObject } from "./appendLinkedObject";
import { Alignment, distributeTextLayers } from "./distributeTextLayers";
import { findLayers } from "./findLayers";
import { modalExecutor } from "./modalExecutor";
import { selectLayerById } from "./selectLayerById";
import { toggleLayer } from "./toggleLayers";
// pixel to point equation
//const pxToPt = (px: number) => 4.3155 * px + 2.797;
const TEXT_SIZE = [80, 90, 98, 105, 115, 120];
export async function initiateTemplate(template: Template) {
	setLoadingState(true);
	const templateName = template.name;
	// const content = template.content.filter((t) => t.include);

	try {
		const templateFile =
			await getTokenHelperState().getTemplateFor(templateName);
		await appendTemplate(templateFile, template);
	} catch (e) {
		console.log(e);
	}
}
async function moveTopPos(padding: number, ypos: number) {
	const activeLayer = app.activeDocument.activeLayers[0];
	const { top: currentTop, left: currentLeft } = activeLayer.boundsNoEffects;
	await modalExecutor("translate", async () => {
		await activeLayer.translate(-currentLeft + padding, ypos - currentTop);
	});
}
//doing in rust side
function calculateTotalHeight(
	lines: TemplateLine[],
	gap: number = 10,
	KERENCADAS: boolean,
) {
	const l = lines.map((l) =>
		fontHeight(TEXT_SIZE[l.scale - 1], KERENCADAS ? FontAnton : FontUniSans),
	);
	const addGap = l.flatMap((n, i) => (i < l.length - 1 ? [n, gap] : [n]));
	const sum = addGap.reduce((a, b) => a + b, 0);

	return sum;
}
async function setFontItalic(isItalic: boolean) {
	let fontShape = isItalic ? "UniSansHeavyItalicCAPS" : "UniSansHeavyCAPS";
	modalExecutor("italiztion", async () => {
		await action.batchPlay(
			[
				{
					_obj: "set",
					_target: [
						{
							_ref: "property",
							_property: "textStyle",
						},
						{
							_ref: "textLayer",
							_enum: "ordinal",
							_value: "targetEnum",
						},
					],
					to: {
						_obj: "textStyle",
						textOverrideFeatureName: 808465457,
						typeStyleOperationType: 3,
						fontPostScriptName: fontShape,
						fontName: "Uni Sans",
						fontStyleName: "Heavy Italic CAPS",
						fontScript: 0,
						fontTechnology: 0,
						fontAvailable: true,
					},
				},
			],
			{},
		);
	});
}
const BLOCKLIST = ["KERENCADAS", "GRUPAG", "NAUFAL"];

async function appendTemplate(templateFile: Entry, template: Template) {
	tryCatchAsync(
		async () => {
			setLoadingState(true);
			logger.info("preparing...", {});
			const templateLines = template.content.filter((l) => l.include);
			const start = performance.now();
			logger.info("appending...", templateFile.name);
			await appendLinkedObject(templateFile, templateFile.name, true);
			//find dcsmstextlayer
			const layers = app.activeDocument.layers;
			const layer = findLayers(layers, "dcsmstext");

			const isBlocked = BLOCKLIST.some((key) => template.name.includes(key));
			let gap = template.gap;
			let padding = template.padding;
			let ypos = 720 - (template.total_height ?? 0) - padding;

			if (layer && layer.length > 0) {
				const l = layer[0];

				await modalExecutor("", async () => await selectLayerById(l.id));

				for await (const [index, line] of templateLines.entries()) {
					if (!line.include) return;
					logger.info("==>", line.text);

					const activeLayer = app.activeDocument.activeLayers[0];
					await modalExecutor("Appending Text", async () => {
						const textlayer = activeLayer.textItem;
						const fontsize = line.font_size ?? 0;
						const layerHeight = line.font_height ?? 0;
						textlayer.characterStyle.size = fontsize;
						textlayer.contents = line.text;
						activeLayer.name = "dcsmstext_tamper";
						if (!isBlocked) await setFontItalic(line.italic);
						await moveTopPos(padding, ypos);
						ypos += layerHeight + gap;
					});
					if (index < templateLines.length - 1)
						await modalExecutor("", async () => {
							await activeLayer.duplicate();
						});
				}
			}
			// await toggleLayer(true);
			await distributeTextLayers(
				template.padding,
				isBlocked ? Alignment.Center : Alignment.Left,
			);
			setLoadingState(false);
			const end = performance.now();
			logger.debug("Execution time:", (end - start).toFixed(2));
		},
		(e) => {
			logger.error("Template Apply Error", e);
		},
	);
}

// @deprecated
async function appendTemplate_old(templateFile: Entry, template: Template) {
	tryCatchAsync(
		async () => {
			setLoadingState(true);
			logger.info("preparing...", {});
			const templateLines = template.content.filter((l) => l.include);
			const start = performance.now();
			logger.info("appending...", templateFile.name);
			await appendLinkedObject(templateFile, templateFile.name, true);
			//find dcsmstextlayer
			const layers = app.activeDocument.layers;
			const layer = findLayers(layers, "dcsmstext");
			const KERENCADAS = template.name.includes("KERENCADAS");
			let gap = 10;
			let padding = 30;
			let ypos =
				720 - calculateTotalHeight(templateLines, gap, KERENCADAS) - padding;

			if (layer && layer.length > 0) {
				const l = layer[0];

				await modalExecutor("", async () => await selectLayerById(l.id));

				for await (const [index, line] of templateLines.entries()) {
					if (!line.include) return;
					logger.info("==>", line.text);

					const activeLayer = app.activeDocument.activeLayers[0];
					await modalExecutor("Appending Text", async () => {
						const textlayer = activeLayer.textItem;
						const fontsize = TEXT_SIZE[line.scale - 1];
						const layerHeight = fontHeight(
							fontsize,
							KERENCADAS ? FontAnton : FontUniSans,
						);
						textlayer.characterStyle.size = fontsize;
						textlayer.contents = line.text;
						activeLayer.name = "dcsmstext_tamper";
						if (!KERENCADAS) await setFontItalic(line.italic);
						await moveTopPos(padding, ypos);
						ypos += layerHeight + gap;
					});
					if (index < templateLines.length - 1)
						await modalExecutor("", async () => {
							await activeLayer.duplicate();
						});
				}
			}
			const end = performance.now();
			logger.error("Execution time:", (end - start).toFixed(2));
			toggleLayer(true);
			setLoadingState(false);
		},
		(e) => {
			logger.error("Template Apply Error", e);
		},
	);
}
