import { action, app, core } from "photoshop";
import { RawFilterDataType } from "@tsockee/frontend";
import { AppState } from "../../AppState.svelte";
export const ADOBE_CAMERA_RAW_FILTER = "Adobe Camera Raw Filter";
export const PatchRawfilterData = (rd: RawFilterDataType) => ({
  _obj: "Adobe Camera Raw Filter",
  $CrVe: "16.4",
  $PrVN: 6,
  $PrVe: 251920384,
  $WBal: { _enum: "$WBal", _value: "customEnum" },
  $Temp: rd.temp,
  $Tint: rd.tint,
  $CrTx: rd.texture,
  $Cl12: rd.clarity,
  $Dhze: rd.dehaze,
  sharpen: rd.sharpen,
  $ShpR: rd.sharpen_radius,
  $ShpD: rd.sharpen_detail,
  $ShpM: 0,
  $LNR: rd.noise_reduction,
  $LNRD: rd.noise_reduction_detail,
  $LNRC: 0,
  $TMMs: 0,
  $PGTM: 0,
  RGBSetupClass: 0,
  _isCommand: true,
});

async function isApplied() {
  const id = app.activeDocument.activeLayers[0].id;

  const [data] = await app.batchPlay(
    [
      {
        _obj: "get",
        _target: [
          { _ref: "layer", _id: id },
          { _ref: "document", _id: app.activeDocument.id },
        ],
      },
    ],
    {},
  );

  const so = data.smartObject;
  const fx = so?.filterFX;

  return {
    applied: Boolean(fx?.length),
    filters: fx ?? [],
  };
}

function buildFilterFX(idx: number, filterType: string, highPass: boolean) {
  const base: any = {
    _obj: "set",
    _target: [
      { _ref: "filterFX", _index: idx + 1 },
      { _ref: "layer", _enum: "ordinal", _value: "targetEnum" },
    ],
    filterFX: {
      _obj: "filterFX",
      filter: filterType,
    },
  };

  if (highPass) {
    base.filterFX.blendOptions = {
      _obj: "blendOptions",
      opacity: { _unit: "percentUnit", _value: 100 },
      mode: { _enum: "blendMode", _value: "overlay" },
    };
  }

  return base;
}

async function currentLayerToSmartObject() {
  const layer = app.activeDocument.activeLayers[0];
  if (layer.kind !== "smartObject") {
    await action
      .batchPlay(
        [
          {
            _obj: "newPlacedLayer",
          },
        ],
        {},
      )
      .catch((e) => {});
  }
}

export async function performRawFilterEffects(
  appstate: AppState,
  filter: any,
  filterObject: string,
): Promise<string> {
  const isHighPass = filterObject === "highPass";

  await core.executeAsModal(
    async () => {
      try {
        let { applied, filters } = await isApplied();
        let index = filters.findIndex(
          (f: any) => f.filter._obj === filterObject,
        );

        await currentLayerToSmartObject();

        // If filter already exists → update it
        if (applied && index > -1) {
          await action.batchPlay(
            [buildFilterFX(index, filter, isHighPass)],
            {},
          );
          appstate.applyingRawFilter = false;
          return;
        }

        // Else apply new filter
        await action.batchPlay([filter], {});

        // If highPass, we must immediately set blendOptions
        if (isHighPass) {
          // get updated list
          ({ filters } = await isApplied());
          index = filters.findIndex((f: any) => f.filter._obj === filterObject);

          if (index > -1) {
            await action.batchPlay([buildFilterFX(index, filter, true)], {});
          }
        }
      } catch (e) {
        appstate.applyingRawFilter = false;
      }
    },
    { commandName: "Raw Filter" },
  );
  appstate.applyingRawFilter = false;
  return "Done";
}

export function isLayerHasRawFilterEffects(
  returnRawFilter: boolean,
): Promise<RawFilterDataType> {
  return new Promise(async (resolve) => {
    const { applied, filters } = await isApplied();

    const defaultFilter: RawFilterDataType = {
      temp: 0,
      tint: 0,
      texture: 0,
      clarity: 0,
      dehaze: 0,
      sharpen: 0,
      sharpen_radius: 1,
      sharpen_detail: 25,
      noise_reduction: 0,
      noise_reduction_detail: 50,
    };

    if (applied) {
      const raw = filters.find(
        (fx: any) => fx.filter._obj === "Adobe Camera Raw Filter",
      );
      if (raw) {
        const s = raw.filter;
        defaultFilter.temp = s.$Temp;
        defaultFilter.tint = s.$Tint;
        defaultFilter.texture = s.$CrTx;
        defaultFilter.clarity = s.$Cl12;
        defaultFilter.dehaze = s.$Dhze;
        defaultFilter.sharpen = s.sharpen;
        defaultFilter.sharpen_radius = s.$ShpR;
        defaultFilter.sharpen_detail = s.$ShpD;
        defaultFilter.noise_reduction = s.$LNR;
        defaultFilter.noise_reduction_detail = s.$LNRD;
      }
    }

    resolve(returnRawFilter ? defaultFilter : { ...defaultFilter });
  });
}
