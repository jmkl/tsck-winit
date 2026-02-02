// socketContext.ts
import {
  Bounds,
  UserEvent,
  FuncCallArgs,
  RawFilterTextPipRange,
  WsMessagePayload,
} from "@tsck/lib";
import ScriptExecutor from "./utils/script-executor";
import { getSocketServerState, initSocketServer } from "./socket/socketContext";
import { getTokenHelperState, initTokenState } from "./token/tokenContext";
import { action, app } from "photoshop";
import { ActionDescriptor } from "photoshop/dom/CoreModules";
import { getSelectionBounds } from "./utils/photoshop/getSelectionBounds";
import {
  ADOBE_CAMERA_RAW_FILTER,
  isLayerHasRawFilterEffects,
  PatchRawfilterData,
  performRawFilterEffects,
} from "./utils/photoshop/RawFilter";
import { getPipRangeAndTexts } from "./utils/selectAllTextTops";
import { logger } from "./utils/addLog";
import { SocketServer } from "./socket/socketServer";
import { initiateTemplate } from "./utils/photoshop/initiateTemplate";
import { appendLinkedObjectResize } from "./utils/photoshop/appendLinkedObjectResize";
import {
  currentLayerToImage,
  pickCropImage,
} from "./utils/photoshop/layerToImage";
import { tryCatch } from "./utils/sandbox";
import { applyTriColor } from "./utils/photoshop/applyTriColor";
import { fnCall } from "./utils/fnCall";
import { getContext, setContext } from "svelte";

const ALLOWED_LAYER = ["smartObject", "pixel", "text"];
const APPSTATE_KEY = Symbol("AppStateContext");

// ─────────────────────────────────────────────
// Singleton + Context Helpers
// ─────────────────────────────────────────────
let contextInstance: AppState | null = null;

function getInstance() {
  if (!contextInstance) contextInstance = new AppState();
  return contextInstance;
}

export function SetAppsState() {
  setContext(APPSTATE_KEY, getInstance());
}

export function GetAppsState() {
  return getContext<AppState>(APPSTATE_KEY);
}

export class AppState {
  selectionBounds: Bounds = $state({ top: 0, bottom: 0, left: 0, right: 0 });
  applyingRawFilter = $state(false);
  scriptExecutor: ScriptExecutor;
  onOff: boolean = $state(false);
  cycleImage: number = $state(0);
  constructor() {
    this.init();
    this.scriptExecutor = new ScriptExecutor();
  }
  init() {
    initSocketServer();
    initTokenState();
    action.addNotificationListener(["set", "select"], (action, desc) => {
      this.handlePhotoshopEvent(action, desc);
    });
    const ws = getSocketServerState();
    this.handleWebsocketEvent(ws);
  }
  debugState() {
    console.log("AppState", [this.selectionBounds, this.applyingRawFilter]);
  }
  updateWebsocketState(online: boolean) {
    this.onOff = online;
  }
  handleWebsocketEvent(ws: SocketServer) {
    ws.on("open", (e: any) => {
      logger.info("WS Open", e.type);
      this.onOff = true;
      this.updateWebsocketState(this.onOff);
    });
    ws.on("close", (e: any) => {
      logger.info("WS Close", e.type);
      this.onOff = false;
      this.updateWebsocketState(this.onOff);
    });
    ws.on("payload|server", async (e: any) => {
      this.cycleImage = (this.cycleImage + 1) % 4;
      let payload = e.detail as UserEvent;
      switch (payload.type) {
        case "Template":
          await initiateTemplate(payload.value.template);
          break;
        case "GenerateImage":
          break;
        case "AppendComfyUIOutput":
          const value = payload.value;
          for await (const img of value.images) {
            let output = await getTokenHelperState().getComfyUiOutputFolder();
            let entry = await output.getEntry(img);
            await appendLinkedObjectResize(entry, value.bounds);
          }
          break;
        case "PerformSelectionToImage":
          await pickCropImage(this);
          break;
        case "PerformLayerToImage":
          await currentLayerToImage();
          break;
        case "ApplyRawFilter":
          tryCatch(
            () => {
              this.applyingRawFilter = true;
              performRawFilterEffects(
                this,
                PatchRawfilterData(payload.value),
                ADOBE_CAMERA_RAW_FILTER,
              );
            },
            (err) => {
              this.applyingRawFilter = false;

              console.log(err);
            },
          );
          break;
        case "ApplyTriColor":
          applyTriColor(payload.value);

          break;
        case "ExecuteScript":
          this.scriptExecutor.runScript(payload.value);
          break;

        //-------------------------------
        // FunCall section
        // ------------------------------
        case "FunctionCall":
          let fn = payload.value.func;
          let args: FuncCallArgs[] | undefined = payload.value.args;
          switch (payload.value.func) {
            case "appendLinkedObject":
              let argv = (args[0] as string).split("|");
              if (argv[0] == "smartobject") {
                const entry = await getTokenHelperState().getSmartObjectEntry(
                  argv[1],
                );
                await fnCall(fn, [entry]);
              } else if (argv[0] == "texture") {
                const entry = await getTokenHelperState().getTextureEntry(
                  argv[1],
                  argv[2],
                );
                await fnCall(fn, [entry]);
              }
              break;
            case "layerToSmartObject":
              fn = "currentLayerToLinkedSmartObject";
              const new_file = await fnCall<string>(fn, [
                getTokenHelperState().getSmartObjectFolder(),
                args[0] as string,
              ]);
              getSocketServerState().sendMessage<WsMessagePayload>({
                from_server: false,
                type: "create-thumb",
                content: new_file ?? "null",
              });
              break;
            default:
              console.log(fn);
              await fnCall(fn, args);
              break;
          }

          break;
      }

      //Handling websocket Message Here
    });
    ws.on("error", (e: any) => {
      logger.info("Error", [e.detail.type, e.detail.data]);
      this.onOff = false;
      this.updateWebsocketState(this.onOff);
    });
  }

  async handlePhotoshopEvent(name: string, descriptor: ActionDescriptor) {
    switch (name) {
      //when setting crop area
      case "set":
        if (this.applyingRawFilter) return;
        const bounds = await getSelectionBounds();
        if (bounds) {
          //sendtoWebsocket
          getSocketServerState().sendMessage<WsMessagePayload>({
            from_server: false,
            type: "selection-mode",
            content: bounds,
          });
        }
        break;
      //when selecting layer
      case "select":
        if (this.applyingRawFilter) return;
        const layer = app.activeDocument.activeLayers[0];
        if (!layer) return;
        const layerType = layer?.kind;
        if (!layerType) return;

        if (ALLOWED_LAYER.includes(layerType)) {
          const rawfilter_data = await isLayerHasRawFilterEffects(true);
          const [pip_ranges, text_layers_info] = await getPipRangeAndTexts();
          const payload: RawFilterTextPipRange = {
            rawfilter_data,
            text_layers_info,
            pip_ranges,
            layer_kind: layerType,
          };
          getSocketServerState().sendMessage<WsMessagePayload>({
            from_server: false,
            type: "raw-filter-text-pip-range",
            content: payload,
          });
        }

        break;
    }
  }
}
