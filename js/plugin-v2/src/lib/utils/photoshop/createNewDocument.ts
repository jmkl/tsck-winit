import { app, core } from "photoshop";

export async function createNewDocument() {
  core.executeAsModal(
    async (ctx, desc) => {
      await app.batchPlay(
        [
          {
            _obj: "make",
            new: {
              _obj: "document",
              artboard: false,
              autoPromoteBackgroundLayer: false,
              preset: "Thumbnail",
            },
          },
        ],
        {},
      );
    },
    { commandName: "Create new Document" },
  );
}
