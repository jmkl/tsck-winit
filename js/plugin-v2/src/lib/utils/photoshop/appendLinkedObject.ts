import { action } from "photoshop";
import { storage } from "uxp";
import { logger } from "../addLog";
import { tryCatchAsync } from "../sandbox";
import { modalExecutor } from "./modalExecutor";

export async function appendLinkedObject(
  obj: Entry,
  convert_to_layer: boolean = false,
  linked: boolean = true,
) {
  return new Promise(async (resolve, reject) => {
    await modalExecutor("", async () => {
      //append linked image in
      tryCatchAsync(
        async () => {
          await action.batchPlay(
            [
              {
                _obj: "placeEvent",
                null: {
                  _path: storage.localFileSystem.createSessionToken(obj),
                  _kind: "local",
                },
                linked: linked,
              },
            ],
            {},
          );
        },
        (error) => {},
      );
    });
    if (convert_to_layer) {
      await modalExecutor("", async () => {
        logger.info("ranstedied", {});
        await action.batchPlay(
          [
            {
              _obj: "placedLayerConvertToLayers",
            },
          ],
          {},
        );
      });
    }
    resolve(0);
  });
}
