import { action, app } from "photoshop";
import { randomName } from "../genRand";
import { tryCatchAsync } from "../sandbox";
import { modalExecutor } from "./modalExecutor";
import { undoModalExecutor } from "./UndoModalExecutor";

const toggleLayer = {
  _obj: "show",
  null: [
    {
      _ref: "layer",
      _enum: "ordinal",
      _value: "targetEnum",
    },
  ],
  toggleOptionsPalette: true,
};
const selectAll = {
  _obj: "set",
  _target: [
    {
      _ref: "channel",
      _property: "selection",
    },
  ],
  to: {
    _enum: "ordinal",
    _value: "allEnum",
  },
};
const flatten = {
  _obj: "flattenImage",
};
const moveUp = {
  _obj: "move",
  _target: [
    {
      _ref: "layer",
      _enum: "ordinal",
      _value: "targetEnum",
    },
  ],
  to: {
    _ref: "layer",
    _enum: "ordinal",
    _value: "front",
  },
};
const copy = {
  _obj: "copyEvent",
  copyHint: "pixels",
};
const paste = {
  _obj: "paste",
  antiAlias: {
    _enum: "antiAliasType",
    _value: "antiAliasNone",
  },
  as: {
    _class: "pixel",
  },
};
export async function selectedToImageLayer(folder: Entry) {
  return new Promise(async (resolve, reject) => {
    const node_name = "SELECTED_";
    tryCatchAsync(
      async () => {
        const sequece = [toggleLayer, selectAll, flatten, copy];
        await modalExecutor("", async (ctx) => {
          let hostControl = ctx.hostControl;
          let documentID = app.activeDocument.id;
          let susId1 = await hostControl.suspendHistory({
            documentID: documentID,
            name: "Toggling Layers",
          });

          for await (const s of sequece) {
            await action.batchPlay([s], {}).catch((e) => console.error(e));
          }
          await hostControl.resumeHistory(susId1, true);
        });
        await undoModalExecutor();
        await modalExecutor("", async () => {
          await action.batchPlay([paste, moveUp], {});
        });
        let selectedLayer = app.activeDocument.activeLayers[0];
        let random_name = randomName();
        await modalExecutor("", async () => {
          if (node_name) random_name = node_name + "_" + random_name;
          selectedLayer.name = random_name;
        });

        await modalExecutor("saving", async () => {
          const result = await action
            .batchPlay(
              [
                {
                  _obj: "exportSelectionAsFileTypePressed",
                  _target: {
                    _ref: "layer",
                    _enum: "ordinal",
                    _value: "targetEnum",
                  },
                  fileType: "png",
                  quality: 32,
                  metadata: 0,
                  destFolder: folder.nativePath,
                  sRGB: true,
                  openWindow: false,
                  _options: { dialogOptions: "dontDisplay" },
                },
              ],
              {},
            )
            .catch((e) => console.log(e));

          const resultname = random_name + ".png";
          let sel = app.activeDocument.activeLayers[0];
          resolve(resultname);
        });
      },
      (error) => {
        reject(error);
      },
    );
  });
}
