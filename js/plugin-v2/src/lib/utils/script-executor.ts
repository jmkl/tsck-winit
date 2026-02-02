import Sval from "sval";
import { storage } from "uxp";
import { getTokenHelperState } from "../token/tokenContext";
import { tryCatchAsync } from "./sandbox";

export default class ScriptExecutor {
  interpreter: Sval;
  constructor() {
    const API_FUNC = {
      //add local function later
    };
    this.interpreter = new Sval({
      ecmaVer: 9,
      sandBox: false,
    });
    this.interpreter.import({
      uxp: require("uxp"),
      os: require("os"),
      fs: require("fs"),
      photoshop: require("photoshop"),
      app: require("photoshop").app,
      core: require("photoshop").core,
      batchPlay: require("photoshop").action.batchPlay,
      executeAsModal: require("photoshop").core.executeAsModal,
      ...API_FUNC,
    });

    setTimeout(async () => {
      const rootFolder = getTokenHelperState().getRootFolder();
      const customscrpipt = await rootFolder.getEntry("customscripts");
      await this.initHelper(customscrpipt);
    }, 3000);
  }
  async initHelper(folder: Entry) {
    const helper = (await folder.getEntry("HELPER.js")) as Entry;
    const helperscript = await helper.read({ format: storage.formats.utf8 });
    this.interpreter.run(`
     ${helperscript}
    `);
  }

  async runScript(scriptContent: string) {
    return new Promise(async (resolve, reject) => {
      tryCatchAsync(
        async () => {
          this.interpreter.run(`
                    "use strict";
                    async function userCode(){${scriptContent}};
                    exports.returnValue = userCode();
                    `);
          const res = await this.interpreter.exports.returnValue;
          resolve({ ok: true, data: res });
        },
        (error) => console.error(error),
      );
    });
  }
}
