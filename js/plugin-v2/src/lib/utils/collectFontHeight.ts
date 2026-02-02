import { app } from "photoshop";
import { modalExecutor } from "./photoshop/modalExecutor";

export async function collectFontHeight() {
  const layer = app.activeDocument.activeLayers[0];
  const data = [];
  for (let i = 50; i < 200; i++) {
    let fontsize = i;
    let height = 0;

    await modalExecutor("fontsize", async () => {
      const textitem = layer.textItem;
      textitem.characterStyle.size = i;
      const l = app.activeDocument.activeLayers[0];
      height = l.boundsNoEffects.bottom - l.boundsNoEffects.top;
    });
    data.push({ fontsize, height });
  }
}
