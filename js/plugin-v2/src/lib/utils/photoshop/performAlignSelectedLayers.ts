import { action } from "photoshop";
import { modalExecutor } from "./modalExecutor";
export enum AlignLayer {
  LEFT = "ADSLefts",
  RIGHT = "ADSRights",
  CENTERHORIZONTAL = "ADSCentersH",
  TOP = "ADSTops",
  BOTTOM = "ADSBottoms",
  CENTERVERTICAL = "ADSCentersV",
}
export async function performAlignSelectedLayers<T>(
  alignto: T,
  toCanvas: boolean,
) {
  modalExecutor("Align Selected", async () => {
    await action.batchPlay(
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
            _value: alignto,
          },
          alignToCanvas: toCanvas,
        },
      ],
      {},
    );
  });
}
