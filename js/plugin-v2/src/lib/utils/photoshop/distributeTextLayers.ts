import { action, app } from "photoshop";
import { Layer } from "photoshop/dom/Layer";
import { tryCatchAsync } from "../sandbox";
import { deselectAll } from "./deselectAll";
import { findLayers } from "./findLayers";
import { modalExecutor } from "./modalExecutor";
import { selectLayerByIds } from "./selectLayerByIds";

export const Alignment = {
  Right: "ADSRights",
  Left: "ADSLefts",
  Center: "ADSCentersH",
} as const;

export type AlignmentType = (typeof Alignment)[keyof typeof Alignment];

/**
 * distributeTextLayers
 *
 * Find and distribute text layers marked with the "dcsmstext_tamper" tag.
 * This function:
 * - Locates matching layers in the active document
 * - Selects those layers
 * - Prepares them for alignment by scaling/translating using the provided outer padding
 * - Applies the requested alignment/distribution
 *
 * @param outer_padding - number of pixels to use as outer margin when preparing alignment
 * @param align - AlignmentType value indicating how layers should be aligned/distributed
 * @returns Promise<void> that resolves when the distribution operation completes
 */
async function distributeTextLayers(
  outer_padding: number,
  align: AlignmentType,
): Promise<void> {
  const layers = findLayers(app.activeDocument.layers, "dcsmstext_tamper");
  await modalExecutor("translate", async () => {
    tryCatchAsync(
      async () => {
        await deselectAll();
        await selectLayerByIds(layers.map((lyr) => lyr.id));
        await prepareAlignLayers(outer_padding, align);
      },
      (error) => {},
    );
  });
}

type LayersBounds = {
  width: number;
  height: number;
  left: number;
  top: number;
  widestLayer: Layer | null;
};

/**
 * getSelectedLayersBounds
 *
 * Calculate the combined bounding box for the currently selected layers in the
 * active document. The bounds are computed using each layer's `boundsNoEffects`
 * (i.e. the geometric bounds excluding layer effects).
 *
 * Returns an object with:
 * - width:  combined width (right - left)
 * - height: combined height (bottom - top)
 * - top:    smallest top value among selected layers
 * - left:   smallest left value among selected layers

 */
function getSelectedLayersBounds(): LayersBounds {
  const layers = app.activeDocument.activeLayers;

  // Single pass to find all bounds and widest layer
  let top = Infinity;
  let bottom = -Infinity;
  let left = Infinity;
  let right = -Infinity;
  let widestLayer = null;
  let maxWidth = -Infinity;

  for (const layer of layers) {
    const bounds = layer.boundsNoEffects;
    if (bounds.top < top) top = bounds.top;
    if (bounds.bottom > bottom) bottom = bounds.bottom;
    if (bounds.left < left) left = bounds.left;
    if (bounds.right > right) right = bounds.right;

    const layerWidth = bounds.right - bounds.left;
    if (layerWidth > maxWidth) {
      maxWidth = layerWidth;
      widestLayer = layer;
    }
  }

  return {
    width: right - left,
    height: bottom - top,
    top,
    left,
    widestLayer,
  };
}

async function alignTextLayer(align: AlignmentType, to_canvas: boolean) {
  await tryCatchAsync(
    async () => {
      await modalExecutor("alignTextLayer", async () => {
        let result = await action.batchPlay(
          [
            {
              _obj: "align",
              _target: [
                {
                  _ref: "layer",
                  _enum: "ordinal",
                  _value: "targetEnum",
                },
              ],
              using: {
                _enum: "alignDistributeSelector",
                _value: align,
              },
              alignToCanvas: to_canvas,
            },
          ],
          {},
        );
      });
    },
    (error) => console.error(error),
  );
}

async function prepareAlignLayers(outer_padding: number, align: AlignmentType) {
  const DOC_WIDTH = 1280;
  const DOC_HEIGHT = 720;
  const MARGIN = outer_padding;

  await scaleLayers(DOC_WIDTH, MARGIN);
  await translateLayers(MARGIN, DOC_HEIGHT);
  await alignTextLayer(align, false);
}

async function translateLayers(margin: number, docHeight: number) {
  await modalExecutor("translateLayers", async () => {
    const scaledBounds = getSelectedLayersBounds();
    const layer = scaledBounds.widestLayer;
    const toplayer = app.activeDocument.activeLayers[0];
    if (!layer) return;
    const translateX = -(layer.boundsNoEffects.left - margin);
    const translateY =
      -toplayer.boundsNoEffects.top +
      (docHeight - scaledBounds.height - margin);
    await layer.translate(translateX, translateY);
  });
}

async function scaleLayers(docWidth: number, margin: number) {
  const bounds = getSelectedLayersBounds();
  const scale = ((docWidth - margin * 2) / bounds.width) * 100;
  await modalExecutor("scaleLayers", async () => {
    const layer = bounds.widestLayer;
    if (layer) await layer.scale(scale, scale);
  });
}

export { alignTextLayer, distributeTextLayers };
