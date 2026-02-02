import { app } from "photoshop";
import { fontHeightUnisans } from "../dataset";
import { findLayers } from "./findLayers";
import { modalExecutor } from "./modalExecutor";
import { selectLayerById } from "./selectLayerById";

export async function arrangeTextLayer(gap: number = 10, padding: number = 30) {
  const layers = findLayers(app.activeDocument.layers, "dcsmstext_tamper");
  if (!layers.length) return;
  layers.sort((a, b) => a.boundsNoEffects.top - b.boundsNoEffects.top);

  const totalHeight =
    layers
      .map(
        (layer) => fontHeightUnisans(layer.textItem.characterStyle.size) + gap,
      )
      .reduce((acc, curr) => acc + curr, 0) + padding;

  let ypos = 720 - totalHeight;
  const start = performance.now();
  for (const layer of layers) {
    const size = layer.textItem.characterStyle.size;
    const layerHeight = fontHeightUnisans(size);
    await modalExecutor("translate", async () => {
      await selectLayerById(layer.id);
    });
    const { top: currentTop, left: currentLeft } = layer.boundsNoEffects;
    await modalExecutor("translate", async () => {
      await layer.translate(-currentLeft + padding, ypos - currentTop);
    });
    ypos += layerHeight + gap;
  }
  const end = performance.now();
}
