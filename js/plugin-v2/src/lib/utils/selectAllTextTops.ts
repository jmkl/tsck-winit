import { TextLayerInfo } from "@tsockee/frontend";
import { mapTo4096 } from "./mapRange";
import { multiGet } from "./photoshop/multiGet";

export async function getPipRangeAndTexts(): Promise<
	[number[], TextLayerInfo[]]
> {
	const layers = await multiGet(["boundsNoEffects", "textKey"]);
	const list = layers[0]["list"];
	const boundsTop = list
		.filter((x: any) => x.name == "dcsmstext_tamper")
		.map((x: any) => {
			return {
				top: x.boundsNoEffects.top._value,
				text: x.textKey.textKey,
				id: x.layerID,
			};
		});
	boundsTop.sort((a: any, b: any) => a.top - b.top);

	const pips = boundsTop.map((a: any) => mapTo4096(a.top));
	const texts: TextLayerInfo[] = boundsTop.map((a: any, idx: number) => {
		return { id: idx, content: a.text, layer_id: a.id };
	});
	return [pips, texts];

	// const layers = findLayers(app.activeDocument.layers, "dcsmstext_tamper");
	// const data = layers.map((l) => {
	//   return {
	//     id: l.id,

	//     text: l.textItem.contents,
	//     top: l.boundsNoEffects.top,
	//   };
	// });

	// const data = ["Hello", "World"];
	// const l = await multiGetExt(["textKey", "boundsNoEffects"]);
	// const list = l[0]["list"];
	// const txts = list.filter((x) => x.name == "dcsmstext_tamper");
	// const nt = txts.map((t) => {
	//   return {
	//     id: t.layerID,
	//     text: t.textKey.textKey,
	//     top: t.boundsNoEffects.top._value,
	//   };
	// });
	// data.sort((a, b) => a.top - b.top);
}
