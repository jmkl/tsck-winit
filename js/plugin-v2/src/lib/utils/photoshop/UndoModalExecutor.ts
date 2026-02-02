import { action, core } from "photoshop";
export async function undoModalExecutor() {
  return new Promise(async (resolve, reject) => {
    await core.executeAsModal(
      async () => {
        await action
          .batchPlay(
            [
              {
                _obj: "select",
                _target: [
                  {
                    _ref: "historyState",
                    _enum: "ordinal",
                    _value: "previous",
                  },
                ],
              },
            ],
            {},
          )
          .catch((e) => console.log(e));
        resolve(true);
      },
      { commandName: "undo" },
    );
  });
}
