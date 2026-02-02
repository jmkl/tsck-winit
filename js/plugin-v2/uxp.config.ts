import { UXP_Manifest, UXP_Config } from "vite-uxp-plugin";
import { name as package_name, version } from "./package.json";
const id = "hello.dcsms." + package_name;
const name = package_name;

const manifest: UXP_Manifest = {
  id,
  name,
  version,
  main: "index.html",
  manifestVersion: 5,
  host: [
    {
      app: "PS",
      minVersion: "23.0.0",
    },
  ],
  requiredPermissions: {
    localFileSystem: "fullAccess",
    clipboard: "readAndWrite",
    network: {
      domains: "all",
    },
    webview: {
      allow: "yes",
      domains: "all",
      allowLocalRendering: "yes",
      enableMessageBridge: "localAndRemote",
    },
    allowCodeGenerationFromStrings: true,
    launchProcess: {
      schemes: ["http", "https"],
      extensions: [".svg", ".png"],
    },
  },
  entrypoints: [
    {
      type: "panel",
      id: "main",
      label: {
        default: "Face Restore",
      },
      minimumSize: {
        width: 230,
        height: 200,
      },
      maximumSize: {
        width: 2000,
        height: 2000,
      },
      preferredDockedSize: {
        width: 230,
        height: 300,
      },
      preferredFloatingSize: {
        width: 230,
        height: 300,
      },
      icons: [
        {
          width: 23,
          height: 23,
          path: "icons/light.png",
          scale: [1, 2],
          theme: ["lightest", "light", "darkest", "dark", "medium"],
        },
      ],
    },
  ],
  icons: [
    {
      width: 48,
      height: 48,
      path: "icons/light.png",
      scale: [1, 2],
      theme: ["dark", "darkest", "medium", "lightest", "light", "all"],
      species: ["pluginList"],
    },
  ],
};
export const config: UXP_Config = {
  manifest,
  hotReloadPort: 8087,
  copyZipAssets: ["pub/*"],
};
