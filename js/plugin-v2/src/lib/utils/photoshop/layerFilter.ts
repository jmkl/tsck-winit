import { getSocketServerState } from "../../socket/socketContext";
import { getTokenHelperState } from "../../token/tokenContext";
import {
  AdjustmentLayer,
  performAdjustmentLayer,
} from "./performAdjustmentLayer";
import {
  AlignLayer,
  performAlignSelectedLayers,
} from "./performAlignSelectedLayers";
import { performSavingFile } from "./performSavingFile";

export function newDoc() {
  console.log("function call newDoc");
}
export async function Save() {
  console.log("function call Save");
  performSavingFile(
    await getTokenHelperState().getRootFolder(),
    getSocketServerState(),
  );
}
export function curve() {
  performAdjustmentLayer(AdjustmentLayer.CURVES);
}
export function exposure() {
  console.log("function call exposure");
  performAdjustmentLayer(AdjustmentLayer.EXPOSURE);
}
export function colorBalance() {
  console.log("function call colorBalance");
  performAdjustmentLayer(AdjustmentLayer.COLORBALANCE);
}
export function hueSaturation() {
  console.log("function call hueSaturation");
  performAdjustmentLayer(AdjustmentLayer.HUESATURATION);
}
export function alignLeft() {
  performAlignSelectedLayers(AlignLayer.LEFT, false);
}
export function alignCenter() {
  performAlignSelectedLayers(AlignLayer.CENTERHORIZONTAL, false);
  console.log("function call alignCenter");
}
export function alignRight() {
  performAlignSelectedLayers(AlignLayer.RIGHT, false);
  console.log("function call alignRight");
}
export function justifyCenter() {
  performAlignSelectedLayers(AlignLayer.CENTERHORIZONTAL, true);
  console.log("function call justifyCenter");
}
