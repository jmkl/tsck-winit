import { appendLinkedObject } from "./photoshop/appendLinkedObject";
import { arrangeTextLayer } from "./photoshop/arrangeTextLayers";
import { createNewDocument } from "./photoshop/createNewDocument";
import { currentLayerToLinkedSmartObject } from "./photoshop/currentLayerToLinkedSmartObject";
import { alignTextLayer } from "./photoshop/distributeTextLayers";
import { findLayers } from "./photoshop/findLayers";
import {
  alignCenter,
  alignLeft,
  alignRight,
  colorBalance,
  curve,
  exposure,
  hueSaturation,
  justifyCenter,
  Save,
} from "./photoshop/layerFilter";
import { linkedTempFolder } from "./photoshop/linkedTempFolder";
import { modalExecutor } from "./photoshop/modalExecutor";
import { multiGet } from "./photoshop/multiGet";
import { selectLayerById } from "./photoshop/selectLayerById";
import { toggleLayer } from "./photoshop/toggleLayers";

export type FnCallType = {
  fn: string;
  args?: unknown[];
};

export async function fnCall<T = unknown>(
  fc: string,
  params?: unknown[],
): Promise<T | undefined> {
  try {
    const fn = fnMap[fc];
    if (!fn) {
      return undefined;
    }

    const args = Array.isArray(params)
      ? params
      : params !== undefined
        ? [params]
        : [];

    return await fn(...args);
  } catch (e) {
    throw e; // Re-throw for caller to handle
  }
}

const fnMap: Record<string, Function> = {
  arrangeTextLayer,
  findLayers,
  modalExecutor,
  selectLayerById,
  createNewDocument,
  appendLinkedObject,
  currentLayerToLinkedSmartObject,
  multiGet,
  toggleLayer,
  alignTextLayer,
  Save,
  curve,
  exposure,
  colorBalance,
  hueSaturation,
  alignLeft,
  alignCenter,
  alignRight,
  linkedTempFolder,
  justifyCenter,
};

type FnMap = typeof fnMap;
export type FnKeys = keyof FnMap;
// Usage:
// const jsonPayload = JSON.stringify({
//   fn: "findLayers",
//   args: [myLayers, "Background"]
// });

// try {
//   const layers = await fnCall<Layer[]>(jsonPayload);
//   console.log("Found layers:", layers);
// } catch (error) {
//   console.error("Failed:", error);
// }
