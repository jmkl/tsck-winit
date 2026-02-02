import { action } from "photoshop";
import { modalExecutor } from "./modalExecutor";

export async function selectLayerByIds(ids: number[]) {
  let code = ids.map((id) => {
    return {
      _ref: "layer",
      _id: id,
    };
  });
  const all_code = {
    _obj: "select",
    _target: code,
  };
  await modalExecutor("", async () => {
    await action.batchPlay([all_code], {});
  });
}
