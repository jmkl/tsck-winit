import { action, app, core } from "photoshop";
import { RawFilterDataType } from "@tsockee/frontend";
import { currentLayerToSmartObject } from "./currentLayerToSmartObject";
import { AppState } from "../../AppState.svelte";

// Constants "Adobe Camera Raw Filter"
export const ADOBE_CAMERA_RAW_FILTER = "Adobe Camera Raw Filter";
const HIGH_PASS_FILTER = "highPass";
const DEFAULT_RAW_FILTER: RawFilterDataType = {
  temp: 0,
  tint: 0,
  texture: 0,
  clarity: 0,
  dehaze: 0,
  sharpen: 0,
  sharpen_radius: 1.0,
  sharpen_detail: 25,
  noise_reduction: 0,
  noise_reduction_detail: 50,
};

export function PatchRawfilterData(rd: RawFilterDataType) {
  return {
    _obj: ADOBE_CAMERA_RAW_FILTER,
    $CrVe: "16.4",
    $PrVN: 6,
    $PrVe: 251920384,
    $WBal: {
      _enum: "$WBal",
      _value: "customEnum",
    },
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
  };
}

interface SmartObjectInfo {
  filterFX?: Array<{ filter: any }>;
}

interface LayerInfo {
  smartObject?: SmartObjectInfo;
}

async function getSmartObjectInfo(): Promise<[boolean, any[] | null]> {
  const activeLayer = app.activeDocument.activeLayers[0];

  const result = (await app.batchPlay(
    [
      {
        _obj: "get",
        _target: [
          { _ref: "layer", _id: activeLayer.id },
          { _ref: "document", _id: app.activeDocument.id },
        ],
      },
    ],
    {},
  )) as LayerInfo[];

  const so = result[0]?.smartObject;
  return so?.filterFX ? [so.filterFX.length > 0, so.filterFX] : [false, null];
}

const createFilterFX = (idx: number, filter: any, isHighPass: boolean) => {
  const baseConfig: any = {
    _obj: "set",
    _target: [
      { _ref: "filterFX", _index: idx + 1 },
      { _ref: "layer", _enum: "ordinal", _value: "targetEnum" },
    ],
    filterFX: {
      _obj: "filterFX",
      filter,
    },
  };

  if (isHighPass) {
    baseConfig.filterFX = {
      ...baseConfig.filterFX,
      blendOptions: {
        _obj: "blendOptions",
        opacity: { _unit: "percentUnit", _value: 100 },
        mode: { _enum: "blendMode", _value: "overlay" },
      },
    };
  }

  return baseConfig;
};

const findFilterIndex = (
  filters: any[] | null,
  filterObject: string,
): number => {
  return filters?.findIndex((e) => e.filter._obj === filterObject) ?? -1;
};

async function applyFilterFX(
  idx: number,
  filter: any,
  isHighPass: boolean,
): Promise<void> {
  try {
    await action.batchPlay([createFilterFX(idx, filter, isHighPass)], {});
  } catch (e) {
    console.error("Failed to apply filter FX:", e);
  }
}

export async function performRawFilterEffects(
  appstate: AppState,
  filter: any,
  filterObject: string,
): Promise<string> {
  await core.executeAsModal(
    async () => {
      appstate.applyingRawFilter = true;
      try {
        const [hasFilters, filterList] = await getSmartObjectInfo();
        const filterIndex = findFilterIndex(filterList, filterObject);
        const isHighPass = filterObject === HIGH_PASS_FILTER;

        if (hasFilters && filterIndex > -1) {
          await currentLayerToSmartObject();
          await applyFilterFX(filterIndex, filter, isHighPass);
          appstate.applyingRawFilter = false;
        } else {
          await currentLayerToSmartObject();

          try {
            await action.batchPlay([filter], {});
          } catch (e) {
            console.error("Failed to apply filter:", e);
          }

          if (isHighPass) {
            const [, updatedFilterList] = await getSmartObjectInfo();
            const updatedIndex = findFilterIndex(
              updatedFilterList,
              filterObject,
            );
            await applyFilterFX(updatedIndex, filter, isHighPass);
          }

          appstate.applyingRawFilter = false;
        }
      } catch (e) {
        console.error(e);
        appstate.applyingRawFilter = false;
      }
    },
    { commandName: "Raw Filter" },
  );

  return "Done";
}

function extractRawFilterData(filterFX: any): RawFilterDataType | null {
  for (const fltrFX of filterFX) {
    const s = fltrFX.filter;

    if (s._obj === ADOBE_CAMERA_RAW_FILTER) {
      return {
        temp: s.$Temp,
        tint: s.$Tint,
        texture: s.$CrTx,
        clarity: s.$Cl12,
        dehaze: s.$Dhze,
        sharpen: s.sharpen,
        sharpen_radius: s.$ShpR,
        sharpen_detail: s.$ShpD,
        noise_reduction: s.$LNR,
        noise_reduction_detail: s.$LNRD,
      };
    }
  }
  return null;
}

export async function isLayerRAWFiltered(
  rawfilter: boolean,
): Promise<RawFilterDataType> {
  const [hasFilters, filterList] = await getSmartObjectInfo();

  if (hasFilters && filterList) {
    const extractedData = extractRawFilterData(filterList);
    if (extractedData) {
      return extractedData;
    }
  }

  return { ...DEFAULT_RAW_FILTER };
}
