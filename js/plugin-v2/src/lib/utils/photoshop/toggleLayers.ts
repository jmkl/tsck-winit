import { action } from "photoshop";
import { tryCatchAsync } from "../sandbox";
import { deselectAll } from "./deselectAll";
import { modalExecutor } from "./modalExecutor";
import { multiGet } from "./multiGet";
import { selectLayerById } from "./selectLayerById";
const LAYER = new Set([
  "IMAGES",
  "BG-EFFECT",
  "BG-EFFECT1",
  "BG-EFFECT2",
  "BG-EFFECT3",
  "BG-EFFECT4",
  "BACKGROUNDHERE",
  "BACKGROUNDHEREROOT",
  "FLARE",
]);

async function toggleLayer(show: boolean = false) {
  await tryCatchAsync(
    async () => {
      const layers = await multiGet();
      const layer_list = layers[0]["list"];
      const shadow_layer = layer_list.filter((l: any) => l.name == "shadow2");
      await selectShadowLayer(shadow_layer);
      const layers_strip = layer_list
        .filter((item: any) => LAYER.has(item.name))
        .map((item: any) => item.name);
      for await (const f of layers_strip) {
        await modalExecutor(
          "",
          async () => await toggle(show, f, shadow_layer),
        );
      }

      show = !show;
      await deselectAll();
    },
    (e) => {
      console.log(e);
    },
  );
}
async function selectShadowLayer(shadow_layer: any) {
  if (shadow_layer && shadow_layer.length > 0) {
    const id = shadow_layer[0].layerID;
    await modalExecutor("", async () => {
      await selectLayerById(id);
    });
  }
}
async function toggle(show: boolean, layername: string, shadow_layer: any) {
  await action.batchPlay(
    [
      {
        _obj: show ? "show" : "hide",
        null: [
          {
            _ref: "layer",
            _name: layername,
          },
        ],
      },
      ...shadow(show),
    ],
    {},
  );
}

const shadow = (dd: boolean) => {
  const show = dd ? "show" : "hide";
  return [
    {
      _obj: show,
      null: [
        {
          _ref: [
            {
              _ref: "layerEffects",
            },
            {
              _ref: "layer",
              _name: "shadow2",
            },
          ],
        },
      ],
    },
    {
      _obj: show,
      null: [
        {
          _ref: [
            {
              _ref: "layerEffects",
            },
            {
              _ref: "layer",
              _name: "shadow3",
            },
          ],
        },
      ],
    },
    {
      _obj: show,
      null: [
        {
          _ref: [
            {
              _ref: "layerEffects",
            },
            {
              _ref: "layer",
              _name: "core",
            },
          ],
        },
      ],
    },
  ];
};
export { toggleLayer };
