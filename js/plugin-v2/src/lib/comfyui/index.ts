export class ComfyUIAPI {
  serverAddress = "127.0.0.1:8888";
  clientId = crypto.randomUUID();
  constructor() {}
  prompt(upscale_model: string, input_name: string, with_bg: boolean) {
    const withbg = {
      16: {
        inputs: {
          filename_prefix: "RemBG",
          images: ["15", 0],
        },
        class_type: "SaveImage",
        _meta: {
          title: "MASK",
        },
      },
    };
    const nobg = {
      10: {
        inputs: {
          image: input_name,
          value: 512,
        },
        class_type: "LoadResizeImageMask512",
        _meta: {
          title: "Load Image n Resize to 512",
        },
      },
      11: {
        inputs: {
          facedetection: "retinaface_resnet50",
          model: "GFPGANv1.4.pth",
          visibility: 1,
          codeformer_weight: 0.5,
          image: ["10", 0],
        },
        class_type: "ReActorRestoreFace",
        _meta: {
          title: "Restore Face 🌌 ReActor",
        },
      },
      12: {
        inputs: {
          upscale_model: ["13", 0],
          image: ["11", 0],
        },
        class_type: "ImageUpscaleWithModel",
        _meta: {
          title: "Upscale Image (using Model)",
        },
      },
      13: {
        inputs: {
          model_name: upscale_model,
        },
        class_type: "UpscaleModelLoader",
        _meta: {
          title: "Load Upscale Model",
        },
      },
      14: {
        inputs: {
          filename_prefix: "ComfyUI",
          images: ["12", 0],
        },
        class_type: "SaveImage",
        _meta: {
          title: "Image",
        },
      },
      15: {
        inputs: {
          torchscript_jit: "default",
          image: ["12", 0],
        },
        class_type: "InspyrenetRembg",
        _meta: {
          title: "Inspyrenet Rembg",
        },
      },
    };
    return with_bg ? { ...nobg, ...withbg } : nobg;
  }

  async queuePrompt(prompt: string, promptId: string) {
    const p = {
      prompt,
      client_id: this.clientId,
      prompt_id: promptId,
    };

    await fetch(`http://${this.serverAddress}/prompt`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(p),
    });
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

  async getImages(prompt: any): Promise<any> {
    return new Promise((resolve, reject) => {
      const ws = new WebSocket(
        `ws://${this.serverAddress}/ws?clientId=${this.clientId}`,
      );
      const promptId = crypto.randomUUID();

      ws.addEventListener("open", async () => {
        try {
          await this.queuePrompt(prompt, promptId);
        } catch (err) {
          reject(err);
        }
      });

      ws.addEventListener("message", async (event) => {
        if (typeof event.data === "string") {
          const message = JSON.parse(event.data);
          if (message.type === "executing") {
            const data = message.data;
            if (data.node === null && data.prompt_id === promptId) {
              // Execution finished
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
                      //   const imageData = await this.getImage(
                      //     image.filename,
                      //     image.subfolder,
                      //     image.type
                      //   );
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
          // Binary previews (optional)
          // Example: new Uint8Array(event.data.slice(8))
        }
      });

      ws.addEventListener("error", (err) => reject(err));
    });
  }
}
