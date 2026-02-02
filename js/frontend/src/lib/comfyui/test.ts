// test.ts (example usage)
import {
  AddPixelNode,
  DuplicateIdError,
  EmptyIdError,
  ImageUpscaleWithModelNode,
  LoadResizeImageMask512Node,
  ReActorRestoreFaceNode,
  RMBGNode,
  SaveImageNode,
  UpscaleModelLoaderNode,
  workflow,
} from "./nodes";

// Example usage - exact same API as Rust!
const node2 = new LoadResizeImageMask512Node("2")
  .setImage("cropped__036q.png")
  .setValue(512);

const node8 = new UpscaleModelLoaderNode("8");

const node7 = new ReActorRestoreFaceNode("7").setImage(node2.id, 0);

const node6 = new ImageUpscaleWithModelNode("6")
  .setUpscaleModel(node8.id, 0)
  .setImage(node7.id, 0);

const node1 = new RMBGNode("1").setImage(node6.id, 0);

const node3 = new AddPixelNode("3").setImage(node1.id, 0).setMask(node1.id, 1);

const node4 = new SaveImageNode("4")
  .setFilenamePrefix("RMBG")
  .setImages(node3.id, 0);

const node5 = new SaveImageNode("5")
  .setFilenamePrefix("RMBG")
  .setImages(node6.id, 0);

// Create workflow - throws error if duplicate IDs!
try {
  const wf = workflow(node1, node2, node3, node4, node5, node6, node7, node8);
} catch (error) {
  if (error instanceof DuplicateIdError) {
    console.error("Duplicate ID:", error.duplicateId);
  } else if (error instanceof EmptyIdError) {
    console.error("Empty ID found");
  }
}

// Test duplicate detection
try {
  const duplicate1 = new RMBGNode("1");
  const duplicate2 = new RMBGNode("1"); // Same ID!
  const wf = workflow(duplicate1, duplicate2);
} catch (error) {
  console.log(
    "✓ Caught duplicate ID:",
    (error as DuplicateIdError).duplicateId,
  );
}
