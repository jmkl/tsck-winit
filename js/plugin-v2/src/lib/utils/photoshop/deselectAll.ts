import { action } from "photoshop";
import { modalExecutor } from "./modalExecutor";

export async function deselectAll() {
  await modalExecutor("Deselect", async () => {
    await action.batchPlay(
      [
        {
          _obj: "selectNoLayers",
          _target: [
            {
              _ref: "layer",
              _enum: "ordinal",
              _value: "targetEnum",
            },
          ],
        },
      ],
      {},
    );
  });
}
