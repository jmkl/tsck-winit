// ─────────────────────────────────────────────
// Singleton + Context Helpers

import { getContext, onDestroy, onMount, setContext } from "svelte";
import type { TemplateLine } from "../routes/MainPanel/Thumbnail/stringUtils";
import { DUMMY_TEMPLATELINE } from "./temp";
import {
  TodoHelper,
  type TodoType,
} from "../routes/MainPanel/Thumbnail/TodoHelper";
import type {
  AppConfig,
  Bounds,
  EventPayload,
  RawFilterTemplate,
  UnListen,
  UserEvent,
  WindowSize,
  WinLevel,
} from "@tsck/lib";
import { invokePayload, invokePayloadWithCallback, listen } from "$lib";
import { ComfyUIAPI } from "./comfyui/comfyuiapi";
import { RAWFILTER_DATA } from "../routes/MainPanel/RawFilter/data";

// ─────────────────────────────────────────────
let contextInstance: Apps | null = null;
const MAINCTX = Symbol("AppsContext");

function getInstance() {
  if (!contextInstance) contextInstance = new Apps();
  return contextInstance;
}

export function SetAppsState() {
  setContext(MAINCTX, getInstance());
}

export function GetAppsState() {
  return getContext<Apps>(MAINCTX);
}

const WINDOW_SIZE: WindowSize[] = [
  { width: 350, height: 25 },
  { width: 350, height: 560 },
];
const LOCAL_STORAGE_ITEMS = "local-storage-items";
export type LocalStorageItems = {
  selected_upscale_model: number;
  selected_facerestore_model: number;
  selected_rmbg_model: number;
  remove_background: boolean;
  selected_rawfilter_template: number;
};
export const HERO_PAGE = {
  THUMBNAIL: 0,
  SMARTOBJECT: 1,
  TEXTURES: 2,
  FACERESTORE: 3,
  RAWFILTER: 4,
  YOUTUBETHUMBNAIL: 5,
  COMMANDLOG: 6,
  CONFIG: 7,
  SETTINGUI: 8,
  HOTKEE: 9,
};
export const SHARED_HERO_PAGE = {
  THUMBNAIL: 0,
  SMARTOBJECT: 1,
  TEXTURES: 2,
  FACERESTORE: 3,
  RAWFILTER: 4,
  YOUTUBETHUMBNAIL: 5,
};
export type PageProps = {
  page: number;
  totalPages: number;
  imageCount: number;
};
export type TextLayerInfo = { content: string; layer_id: number; id: number };
class Apps {
  WindowLevel: WinLevel = $state("Normal");
  CompactMode = $state(false);
  globalShadowLayer = $state(false);
  IsWindowFocus = $state(false);
  LoadingPanel = $state(false);
  Pages: PageProps = $state({ page: 0, totalPages: 0, imageCount: 0 });
  ThumbnailTypeface = $state<"font-unisans" | "font-anton">("font-unisans");
  TodoTemplateLines: TemplateLine[] = $state(DUMMY_TEMPLATELINE);
  todoList: TodoType[] = $state([]);
  todoHelper: TodoHelper | undefined = $state();
  showSnippet = $state(true);
  transitionEnd = $state(false);
  globalActivePage: number = $state(HERO_PAGE.RAWFILTER);
  stamp: HTMLDivElement | undefined = $state();
  AppConfig: AppConfig | undefined = $state();
  ToolbarContent = $state();
  httpServerStaticUrl: string | undefined = $state();
  //FACERESTORE
  FacerestoreImageFileName: string | undefined = $state();
  comfyuiAPI: ComfyUIAPI | undefined = $state();
  facerestoreSelectionMode = $state(false);
  facerestoreSelectionBound: Bounds = $state({
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
  });
  youtubeThumbnailUrl: string | undefined = $state();
  textLayersInfo: TextLayerInfo[] = $state([]);
  facerestoreImageSource: string | undefined = $state();
  globalStorageItems: LocalStorageItems | undefined = $state();
  //RAWFILTER AND COLOR
  private rawfilterRawFilterData = $state(RAWFILTER_DATA);
  rawfilterTemplates: RawFilterTemplate[] = $state([]);
  rawfilterPipsRanges: number[] = $state([]);
  rawfilterColorRanges = $state([1365, 2730]);
  rawfilterColorList: string[] = $state([]);
  rawfilterTriColor = $state(["#ffff00", "#ffffff", "#ffff00"]);
  frontendeventListener: UnListen = $state();
  psEventListener: UnListen = $state();
  hotkeyListener: UnListen = $state();
  transformWindowTimeout: NodeJS.Timeout | undefined = $state();
  constructor() {
    this.loadLocalStorateItems();
    this.initConfig();

    onMount(() => {
      this.frontendeventListener = this.spawnFrontendListener();
      this.hotkeyListener = this.spawnHotkeyListener();
      this.psEventListener = this.spawnPSEventLister();
    });
    onDestroy(() => {
      if (this.frontendeventListener) this.frontendeventListener();
      if (this.hotkeyListener) this.hotkeyListener();
      if (this.psEventListener) this.psEventListener();
      if (this.transformWindowTimeout)
        clearTimeout(this.transformWindowTimeout);
    });
  }

  initConfig() {
    invokePayloadWithCallback<UserEvent, AppConfig>(
      { type: "GetAppConfig" },
      (error, result) => {
        if (error || !result) return;
        this.AppConfig = result;
        this.rawfilterColorList = this.AppConfig.color_list;
        this.rawfilterTemplates = this.AppConfig.rawfilter_template;
        this.httpServerStaticUrl = `http://127.0.0.1:${this.AppConfig.http_server_port}`;
        this.todoInit();
        this.setupComfyuiHelper();
      },
    );
  }
  //Todo
  async todoInit() {
    if (!this.AppConfig) return;
    this.todoHelper = new TodoHelper(
      `http://127.0.0.1:${this.AppConfig.whatsapp_bot_port}`,
    );
    this.todoUpdate();
  }
  private setupComfyuiHelper() {
    if (!this.AppConfig?.comfyui_url) return;
    this.comfyuiAPI = new ComfyUIAPI(this.AppConfig?.comfyui_url!);
    this.comfyuiAPI.initModel();
  }

  async todoUpdate() {
    if (this.todoHelper) this.todoList = await this.todoHelper?.fetchTodo();
  }
  setWindowFocus(focus: boolean) {
    this.IsWindowFocus = focus;
  }
  showLoadingPanel(show: boolean) {
    this.LoadingPanel = show;
  }
  toggleShadowLayer() {
    this.globalShadowLayer = !this.globalShadowLayer;
    const payload: UserEvent = {
      type: "FunctionCall",
      value: {
        func: "toggleLayer",
        args: [this.globalShadowLayer],
      },
    };

    invokePayload(payload);
  }
  resetShadowLayer() {}
  loadLocalStorateItems() {
    const def_state: LocalStorageItems = {
      selected_upscale_model: 0,
      selected_facerestore_model: 0,
      selected_rmbg_model: 0,
      remove_background: false,
      selected_rawfilter_template: 0,
    };
    const storage = localStorage.getItem(LOCAL_STORAGE_ITEMS);
    if (!storage) {
      this.globalStorageItems = def_state;
    } else {
      try {
        this.globalStorageItems = JSON.parse(storage);
      } catch (e) {
        this.globalStorageItems = def_state;
      }
    }
    this.updateAllLocalStorageItems();
  }
  private updateAllLocalStorageItems() {
    localStorage.setItem(
      LOCAL_STORAGE_ITEMS,
      JSON.stringify(this.globalStorageItems),
    );
  }
  getLocalStorageItem<K extends keyof LocalStorageItems>(
    key: K,
  ): number | boolean {
    if (!this.globalStorageItems) return 0;
    return this.globalStorageItems[key];
  }
  updateLocalStorageItem<K extends keyof LocalStorageItems>(
    key: K,
    value: LocalStorageItems[K],
  ) {
    if (!this.globalStorageItems) return;
    this.globalStorageItems[key] = value;
    localStorage.setItem(
      LOCAL_STORAGE_ITEMS,
      JSON.stringify(this.globalStorageItems),
    );
  }
  rawfilterGetRawFilterData() {
    return this.rawfilterRawFilterData;
  }
  rawfilterSaveRawFilterTemplate(data: RawFilterTemplate) {
    this.rawfilterTemplates.push(data);
    invokePayload<UserEvent>({
      type: "UpdateRawfilterTemplates",
      value: this.rawfilterTemplates,
    });
  }
  rawfilterUpdateRawfilterData(data: any) {
    this.rawfilterRawFilterData = data;
  }
  spawnHotkeyListener() {
    return listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::HOTKEE",
      (event) => {
        if (event) {
          switch (event.type) {
            default:
              break;
          }
        }
      },
    );
  }
  spawnPSEventLister() {
    return listen("tsockee:websocket|event", (event) => {});
  }

  spawnFrontendListener() {
    return listen<EventPayload, UserEvent>(
      "tsck::event|EVENTPAYLOAD::FRONTEND",
      (event) => {
        if (event) {
          switch (event.type) {
            case "ToggleShadow":
              console.log("ToggleShadow");
              this.toggleShadowLayer();
              break;
            case "ToggleWindowLevel":
              this.WindowLevel =
                this.WindowLevel === "Normal" ? "Top" : "Normal";
              invokePayload<UserEvent>({
                type: "SetWindowLevel",
                value: [this.WindowLevel, "main"],
              });
              break;
            case "FocusPage":
              this.globalActivePage = event.value - 1;
              break;
            case "ToggleCompactMode":
              this.CompactMode = !this.CompactMode;
              this.transformMainWindow();
              break;
            case "LoadingState":
              this.showLoadingPanel(event.value.loading);
              break;
            case "SmartobjectThumbnailUpdate":
              this.showLoadingPanel(false);
              break;
            case "FacerestorePreviewImage":
              setTimeout(() => {
                this.FacerestoreImageFileName = event.value[0];
                this.facerestoreImageSource = event.value[1];
              }, 1000);
              break;
            case "RawFilterTextPipRange":
              switch (event.value.layer_kind) {
                case "pixel":
                  this.globalActivePage = HERO_PAGE.FACERESTORE;
                  break;
                case "text":
                case "smartObject":
                  this.globalActivePage = HERO_PAGE.RAWFILTER;
                  break;
                default:
                  break;
              }

              const value = event.value;
              this.rawfilterRawFilterData?.forEach((item) => {
                if (item.name in value.rawfilter_data) {
                  item.value =
                    value.rawfilter_data[
                      item.name as keyof typeof value.rawfilter_data
                    ];
                }
              });
              this.rawfilterPipsRanges = value.pip_ranges;
              this.textLayersInfo = value.text_layers_info;
              break;
            case "RawFilterDataUpdate":
              this.rawfilterRawFilterData?.forEach((item) => {
                if (item.name in event.value) {
                  item.value =
                    event.value[item.name as keyof typeof event.value];
                }
              });
              break;
            case "SelectionChanged":
              const val = event.value;
              this.facerestoreSelectionMode = val.selection_mode;
              this.facerestoreSelectionBound = val.bounds;
              this.globalActivePage = HERO_PAGE.FACERESTORE;
              this.IsWindowFocus = true;

              break;
            case "CyclePages":
              this.IsWindowFocus = true;
              const len = Object.keys(SHARED_HERO_PAGE).length;
              this.globalActivePage =
                (this.globalActivePage + event.value + len) % len;
              break;
            case "WhatsappUpdate":
              switch (event.value.type) {
                case "connect":
                case "delete":
                case "edit":
                case "upsert":
                  this.todoUpdate();
                  break;
              }
              break;
            case "ActivateWorkSpace":
              break;
            case "WindowFocusChange":
              this.IsWindowFocus = event.value;
              if (!event.value) {
                //hide
                this.prepareTransformMainWindow();
              } else {
                //show
                this.CompactMode = false;
                this.transformMainWindow();
              }
              break;
          }
        }
      },
    );
  }

  transformMainWindow() {
    invokePayload<UserEvent>({
      type: "SetWindowSize",
      value: this.CompactMode ? WINDOW_SIZE[0] : WINDOW_SIZE[1],
    });
    // invokePayload<UserEvent>({
    //   type: "TransformWindow",
    //   value: {
    //     label: "main",
    //     easing: "EaseInQuad",
    //     toSize: SIZE[compactMode ? 0 : 1],
    //     duration: 200,
    //   },
    // });
  }
  prepareTransformMainWindow() {
    if (this.transformWindowTimeout) clearTimeout(this.transformWindowTimeout);
    this.transformWindowTimeout = setTimeout(() => {
      this.CompactMode = true;
      this.transformMainWindow();
    }, 150_000);
  }
}
