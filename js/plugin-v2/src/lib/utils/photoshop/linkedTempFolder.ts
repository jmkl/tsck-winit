import { action, app } from "photoshop";
import { storage } from "uxp";
import { getTokenHelperState } from "../../token/tokenContext";
import { tryCatchAsync } from "../sandbox";
import { modalExecutor } from "./modalExecutor";

export async function linkedTempFolder() {
  tryCatchAsync(
    async () => {
      const layers = app.activeDocument.activeLayers;
      const randomName = (Math.random() + 1).toString(36).substring(7);
      const tempFolder = await getTokenHelperState().getTempFolder();
      const newFile = await tempFolder.createFile(randomName, {
        overwrite: true,
      });
      const token = storage.localFileSystem.createSessionToken(newFile);

      const shouldCreateSmartObject =
        layers.length !== 1 || layers[0].kind !== "smartObject";
      modalExecutor("Linked", async () => {
        if (shouldCreateSmartObject) {
          await action.batchPlay([{ _obj: "newPlacedLayer" }], {});
        }

        await action.batchPlay(
          [
            {
              _obj: "placedLayerConvertToLinked",
              _target: [
                { _ref: "layer", _enum: "ordinal", _value: "targetEnum" },
              ],
              using: { _path: token, _kind: "local" },
            },
          ],
          {},
        );
      });
    },
    (error) => console.error(error),
  );
}
