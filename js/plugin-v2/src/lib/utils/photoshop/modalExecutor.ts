import { core } from "photoshop";

// Define the modal context types (adjust based on Photoshop's actual types)
type ExecutionContext = Parameters<
  Parameters<typeof core.executeAsModal>[0]
>[0];
type DescriptorPlayOptions = Parameters<
  Parameters<typeof core.executeAsModal>[0]
>[1];

export async function modalExecutor<T extends any[] = []>(
  tag: string,
  fn: (
    ctx: ExecutionContext,
    ev: DescriptorPlayOptions,
    ...args: T
  ) => Promise<void>,
  ...args: T
) {
  await core.executeAsModal(
    async (ctx, ev) => {
      await fn(ctx, ev, ...args);
    },
    { commandName: tag },
  );
}
// import { core } from "photoshop";

// // Option 3: More flexible - fn can optionally use the parameters
// export async function modalExecutor<T extends any[] = []>(
//   tag: string,
//   fn: (...args: T) => Promise<void>,
//   ...args: T
// ) {
//   await core.executeAsModal(
//     async (ctx, ev) => {
//       // let hostControl = ctx.hostControl;
//       // let documentID = app.activeDocument.id;
//       // let susId1 = await hostControl.suspendHistory({
//       //   documentID: documentID,
//       //   name: randomName + "_" + tag,
//       // });
//       await fn(...args);
//       // await hostControl.resumeHistory(susId1, true);
//     },
//     { commandName: tag },
//   );
// }
