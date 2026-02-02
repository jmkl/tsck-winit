import { action, app } from "photoshop";
import { storage } from "uxp";
import { getTokenHelperState } from "../../token/tokenContext";
import { randomName } from "../genRand";
import { modalExecutor } from "./modalExecutor";

export async function currentLayerToSmartObject() {
  try {
    const layers = app.activeDocument.activeLayers;
    const rname = randomName();
    const tempFolder = await getTokenHelperState().getTempFolder();
    const newFile = await tempFolder.createFile(rname, {
      overwrite: true,
    });
    const token = storage.localFileSystem.createSessionToken(newFile);

    const shouldCreateSmartObject =
      layers.length !== 1 || layers[0].kind !== "smartObject";
    await modalExecutor("", async () => {
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
  } catch (err) {
    console.error("Link layer error:", err);
  }
}
