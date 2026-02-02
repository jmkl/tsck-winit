import { type Layers } from "photoshop/dom/collections/Layers";
import { type Layer } from "photoshop/dom/Layer";

/**
 * Recursively finds a layer group by name.
 * @param layers The Layers collection to search.
 * @param groupName The name of the group to find.
 * @returns The found Layer (group) or undefined if not found.
 */
 export function findLayers(layers: Layers, layerName: string): Layer[] {
   const result: Layer[] = [];

   for (const layer of layers) {
     if (layer.name === layerName) {
       result.push(layer);
     }

     // Recurse into groups
     if (layer.kind === "group" && layer.layers) {
       result.push(...findLayers(layer.layers, layerName));
     }
   }

   return result;
 }
