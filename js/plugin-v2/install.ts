//@ts-ignore
import fs, { readdirSync } from "fs";
//@ts-ignore
import unzipper from "unzipper";

const path = "C:/Users/jmkl/AppData/Roaming/Adobe/UXP/Plugins/External";
const uxpPath = path + "/hello.dcsms.leplug_0.0.1";

extractCCX(uxpPath);
console.log("INSTALLED");
function extractCCX(outputPath: string) {
  fs.createReadStream("ccx//hello.dcsms.leplug-veetwoo_PS.ccx")
    .pipe(unzipper.Extract({ path: outputPath }))
    .on("close", () => console.log("✅ Unzipped successfully"));
}
