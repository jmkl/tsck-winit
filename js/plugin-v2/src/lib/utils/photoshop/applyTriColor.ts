import { action, app, constants } from "photoshop";
import { findLayers } from "./findLayers";
import { modalExecutor } from "./modalExecutor";

let removeGuideTimeout: NodeJS.Timeout | undefined;

type TriColorConfig = {
  tri_color: string[];
  position: number[];
};

type RGB = [number, number, number];

const CANVAS_HEIGHT = 720;
const GRADIENT_SCALE = 4096;

export async function applyTriColor(config: TriColorConfig): Promise<void> {
  const [topColor, midColor, bottomColor] = config.tri_color.map(hex2rgb);
  const [posA, posB] = config.position;

  await addGuide(posA, posB);

  await modalExecutor("", async () => {
    const layerId = findLayers(app.activeDocument.layers, "colorfill")[0].id;

    await modalExecutor("Apply Color", async () => {
      // Select and unlock layer
      await action.batchPlay([selectLayer(layerId), unlockLayer()], {});

      // Apply tri-color gradient
      await action.batchPlay(
        [createTriColorGradient(topColor, midColor, bottomColor, posA, posB)],
        {},
      );

      // Lock layer
      await action.batchPlay([lockLayer()], {});
    });
  });

  scheduleGuideRemoval();
}

function scheduleGuideRemoval(): void {
  if (removeGuideTimeout) {
    clearTimeout(removeGuideTimeout);
  }
  removeGuideTimeout = setTimeout(() => {
    modalExecutor("remove guide", async () => {
      app.activeDocument.guides.removeAll();
    });
  }, 2000);
}

async function addGuide(topPos: number, bottomPos: number): Promise<void> {
  await modalExecutor("AddGuide", async () => {
    const top = Math.floor((topPos / GRADIENT_SCALE) * CANVAS_HEIGHT);
    const bottom = Math.floor((bottomPos / GRADIENT_SCALE) * CANVAS_HEIGHT);

    const { guides } = app.activeDocument;
    guides.removeAll();
    guides.add(constants.Direction.HORIZONTAL, top);
    guides.add(constants.Direction.HORIZONTAL, bottom);
  });
}

function hex2rgb(hex: string): RGB {
  const cleaned = hex.replace(/^#/, "");

  // Expand shorthand hex (e.g., "03F" -> "0033FF")
  const fullHex =
    cleaned.length === 3
      ? cleaned
          .split("")
          .map((char) => char + char)
          .join("")
      : cleaned;

  return [
    parseInt(fullHex.slice(0, 2), 16),
    parseInt(fullHex.slice(2, 4), 16),
    parseInt(fullHex.slice(4, 6), 16),
  ];
}

function selectLayer(layerId: number) {
  return {
    _obj: "select",
    _target: { _ref: "layer", _id: layerId },
  };
}

function unlockLayer() {
  return {
    _obj: "applyLocking",
    _target: [{ _ref: "layer", _enum: "ordinal", _value: "targetEnum" }],
    layerLocking: { _obj: "layerLocking", protectAll: false },
  };
}

function lockLayer() {
  return {
    _obj: "applyLocking",
    _target: [{ _ref: "layer", _enum: "ordinal", _value: "targetEnum" }],
    layerLocking: { _obj: "layerLocking", protectAll: true },
  };
}

function createColorStop(color: RGB, location: number) {
  return {
    _obj: "colorStop",
    color: {
      _obj: "RGBColor",
      red: color[0],
      grain: color[1],
      blue: color[2],
    },
    type: { _enum: "colorStopType", _value: "userStop" },
    location,
    midpoint: 50,
  };
}

function createTriColorGradient(
  top: RGB,
  mid: RGB,
  bottom: RGB,
  posA: number,
  posB: number,
) {
  return {
    _obj: "set",
    _target: [{ _ref: "contentLayer", _enum: "ordinal", _value: "targetEnum" }],
    to: {
      _obj: "gradientLayer",
      gradientsInterpolationMethod: {
        _enum: "gradientInterpolationMethodType",
        _value: "perceptual",
      },
      angle: { _unit: "angleUnit", _value: -90 },
      type: { _enum: "gradientType", _value: "linear" },
      scale: { _unit: "percentUnit", _value: 100 },
      gradient: {
        _obj: "gradientClassEvent",
        name: "Custom",
        gradientForm: { _enum: "gradientForm", _value: "customStops" },
        interfaceIconFrameDimmed: 0,
        colors: [
          createColorStop(top, 0),
          createColorStop(top, posA - 1),
          createColorStop(mid, posA),
          createColorStop(mid, posB),
          createColorStop(bottom, posB + 1),
        ],
        transparency: [
          {
            _obj: "transferSpec",
            opacity: { _unit: "percentUnit", _value: 100 },
            location: 0,
            midpoint: 50,
          },
          {
            _obj: "transferSpec",
            opacity: { _unit: "percentUnit", _value: 100 },
            location: GRADIENT_SCALE,
            midpoint: 50,
          },
        ],
      },
    },
  };
}
