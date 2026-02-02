import { action, app, core } from "photoshop";
import { Layer } from "photoshop/dom/Layer";
import { Bounds } from "photoshop/dom/types/SharedTypes";
import { storage } from "uxp";

import { getSocketServerState } from "../../socket/socketContext";
import { getTokenHelperState } from "../../token/tokenContext";
import { tryCatchAsync } from "../sandbox";
import { modalExecutor } from "./modalExecutor";
import { selectedToImageLayer } from "./selectedToImageLayer";
import { undoModalExecutor } from "./UndoModalExecutor";
import { AppState } from "../../AppState.svelte";
const formats = storage.formats;

export async function currentLayerToImage() {
  const comfyui = await getTokenHelperState().getComfyUIFolder();
  const input = await comfyui.getEntry("input");
  const name = (await selectedToImageLayer(input)) as string;
  if (!name) return;
  const base64image = (await toBase64Image(name, input)) as string;
  if (base64image)
    getSocketServerState().sendFacerestorePreviewImage([name, base64image]);
}

export async function pickCropImage(appstate: AppState) {
  const comfyui = await getTokenHelperState().getComfyUIFolder();
  const input = await comfyui.getEntry("input");
  const result = await selectAndCropImage(input, appstate.selectionBounds);
  const name = result.name;
  const base64image = (await toBase64Image(name, input)) as string;
  if (base64image)
    getSocketServerState().sendFacerestorePreviewImage([name, base64image]);
}

// !FIXME GOTTA REMOVE THIS
async function toBase64Image(filename: string, input_folder: Entry) {
  return new Promise(async (resolve, reject) => {
    resolve("data:image/png;base64,");

    tryCatchAsync(
      async () => {
        setTimeout(async () => {
          const comfyui = await getTokenHelperState().getComfyUIFolder();
          const input = await comfyui.getEntry("input");
          const image = await input.getEntry(filename);
          const base64image = await base64ImageFromEntry(image);
          await modalExecutor("deleting old layer", async () => {
            const layer = app.activeDocument.activeLayers[0];
            layer.delete();
          });
          resolve(base64image);
        }, 1000);
      },
      (error) => {
        reject(error);
      },
    );
  });
}

async function base64ImageFromEntry(file_entry: Entry): Promise<string> {
  return new Promise(async (resolve, reject) => {
    try {
      // Step 2: Get file contents as base64
      const arrayBuffer = await file_entry.read({ format: formats.binary });
      const uint8Array = new Uint8Array(arrayBuffer);
      // Convert ArrayBuffer → Base64
      let binary = "";
      const len = uint8Array.byteLength;
      for (let i = 0; i < len; i++) {
        binary += String.fromCharCode(uint8Array[i]);
      }
      const base64 = btoa(binary);
      resolve(`data:image/png;base64,${base64}`);
    } catch (error) {
      reject(0);
    }
  });
}

type CroppedResult = {
  name: string;
  layer: Layer;
};
async function selectAndCropImage(
  folder: Entry,
  selection: Bounds,
): Promise<CroppedResult> {
  const node_name = "cropped_";

  return new Promise(async (resolve, reject) => {
    await core.executeAsModal(
      async (ctx, desc) => {
        let hostControl = ctx.hostControl;
        let documentID = app.activeDocument.id;
        let susId1 = await hostControl.suspendHistory({
          documentID: documentID,
          name: "saveSelectionToImage",
        });
        await action
          .batchPlay(
            [
              {
                _obj: "flattenImage",
              },
            ],
            {},
          )
          .catch((e) => console.log(e));

        await action
          .batchPlay(
            [
              {
                _obj: "copyEvent",
                copyHint: "pixels",
              },
            ],
            {},
          )
          .catch((e) => console.log(e));
        await hostControl.resumeHistory(susId1, true);

        await undoModalExecutor();

        await action
          .batchPlay(
            [
              {
                _obj: "paste",
                inPlace: true,
                antiAlias: {
                  _enum: "antiAliasType",
                  _value: "antiAliasNone",
                },
                as: {
                  _class: "pixel",
                },
                _isCommand: true,
              },
            ],
            {},
          )
          .catch((e) => console.log(e));
        await action.batchPlay(
          [
            {
              _obj: "set",
              _target: [
                {
                  _ref: "channel",
                  _property: "selection",
                },
              ],
              to: {
                _ref: "channel",
                _enum: "channel",
                _value: "transparencyEnum",
              },
              _isCommand: true,
            },
          ],
          {},
        );

        let selectedLayer = app.activeDocument.activeLayers[0];

        let random_name = generateRandomName(null, true);
        if (node_name) random_name = node_name + "_" + random_name;

        selectedLayer.name = random_name;

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
                destFolder: folder.nativePath, //destFolder.nativePath,
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

        try {
          //await sel.delete();
        } catch (error) {
          console.log(error);
        }

        resolve({ name: resultname, layer: selectedLayer });
      },
      { commandName: "pickrop" },
    );
  });
}

function generateRandomName(
  filetype: string | null,
  without_extension: boolean,
) {
  if (filetype)
    return (
      (Math.random() + 1).toString(36).substring(7) +
      filetype.replace("image/", ".")
    );
  else
    return (
      (Math.random() + 1).toString(36).substring(7) +
      (without_extension ? "" : ".png")
    );
}
