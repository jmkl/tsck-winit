import Logger from "$lib/Logger";
import { type NodeDefinition } from "./nodes";

export class ComfyUIAPI {
  serverAddress;
  clientId = crypto.randomUUID();
  upscaleModel = [];
  faceRestoreModel = [];
  rmbgModel = [];
  constructor(api_url: string) {
    this.serverAddress = api_url;
    this.initModel();
  }
  async initModel(): Promise<boolean> {
    await this.getUpscaleModel();
    await this.getRMBGModel();
    await this.getFaceRestoreModel();
    return this.checkSanity();
  }
  checkSanity() {
    if (
      this.upscaleModel.length <= 0 ||
      this.faceRestoreModel.length <= 0 ||
      this.rmbgModel.length <= 0
    )
      return false;
    else return true;
  }

  async getUpscaleModel() {
    const response = await fetch(
      `http://${this.serverAddress}/object_info/UpscaleModelLoader`,
    );
    const result = await response.json();
    console.log(result);
    this.upscaleModel =
      result.UpscaleModelLoader.input.required.model_name[1].options;
  }
  async getFaceRestoreModel() {
    const response = await fetch(
      `http://${this.serverAddress}/object_info/ReActorRestoreFace`,
    );
    const result = await response.json();
    this.faceRestoreModel = result.ReActorRestoreFace.input.required.model[0];
  }
  async checkServer() {
    const response = await fetch(`http://${this.serverAddress}/prompt`);
    return response.ok;
  }
  async getRMBGModel() {
    const response = await fetch(
      `http://${this.serverAddress}/object_info/RMBG`,
    );
    const result = await response.json();
    this.rmbgModel = result.RMBG.input.required.model[0];
  }

  async getImage(filename: string, subfolder: string, folderType: string) {
    const params = new URLSearchParams({
      filename,
      subfolder,
      type: folderType,
    });

    const response = await fetch(`http://${this.serverAddress}/view?${params}`);
    return await response.arrayBuffer(); // raw bytes
  }

  async getHistory(promptId: string) {
    const response = await fetch(
      `http://${this.serverAddress}/history/${promptId}`,
    );
    return await response.json();
  }

  //TODO
  async deployComfyUIPrompt(
    prompt_mapped: Record<string, NodeDefinition>,
  ): Promise<any> {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket(
        `ws://${this.serverAddress}/ws?clientId=${this.clientId}`,
      );
      const promptId = crypto.randomUUID();

      ws.addEventListener("open", async () => {
        const payload = JSON.stringify(
          {
            prompt: prompt_mapped,
            client_id: this.clientId,
            prompt_id: promptId,
          },
          null,
          2,
        );

        Logger.warn(payload);

        await fetch(`http://${this.serverAddress}/prompt`, {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: payload,
        });
      });

      ws.addEventListener("message", async (event) => {
        if (typeof event.data === "string") {
          const message = JSON.parse(event.data);
          if (message.type === "executing") {
            const data = message.data;
            if (data.node === null && data.prompt_id === promptId) {
              ws.close();

              try {
                const history = await this.getHistory(promptId);

                const historyData = history[promptId];
                const outputImages: any = {};

                for (const nodeId of Object.keys(historyData.outputs)) {
                  const nodeOutput = historyData.outputs[nodeId];
                  const imagesOutput = [];

                  if ("images" in nodeOutput) {
                    for (const image of nodeOutput.images) {
                      imagesOutput.push(image.filename);
                    }
                  }

                  outputImages[nodeId] = imagesOutput;
                }
                resolve(outputImages);
              } catch (err) {
                reject(err);
              }
            }
          }
        } else {
          //const data = new Uint8Array(event.data.slice(8))
        }
      });

      ws.addEventListener("error", (err) => reject(err));
    });
  }
}
