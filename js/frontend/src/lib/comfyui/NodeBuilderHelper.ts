import { ImageUpscaleWithModelNode } from './nodeclass/ImageUpscaleWithModelNode';
import { ReActorRestoreFaceAdvancedNode } from './nodeclass/ReActorRestoreFaceAdvanced';
import { RMBGNode } from './nodeclass/RMBG';
import {
	AddPixelNode,
	DuplicateIdError,
	EmptyIdError,
	LoadResizeImageMask512Node,
	SaveImageNode,
	UpscaleModelLoaderNode,
	workflow,
	type NodeDefinition
} from './nodes';

export function BuildFaceRestoreNode(
	image: string,
	upscaleModel: string,
	facerestoreModel: string,
	removeBackground: boolean,
	REMBGModel: string
): Promise<Record<string, NodeDefinition>> {
	return new Promise((resolve, reject) => {
		const loadImageNode = new LoadResizeImageMask512Node('2').setImage(image).setValue(512);

		const upscale_loader = new UpscaleModelLoaderNode('8').setModelName(upscaleModel);

		// const reactor = new ReActorRestoreFaceNode('7')
		// 	.setModel(facerestoreModel)
		// 	.setImage(loadImageNode.id, 0);
		const reactor = new ReActorRestoreFaceAdvancedNode('7')
			.setModel(facerestoreModel)
			.setImage([loadImageNode.id, 0]);

		const upscaleImagewModel = new ImageUpscaleWithModelNode('6')
			.setUpscale_model([upscale_loader.id, 0])
			.setImage([reactor.id, 0]);
		// const upscaleImagewModel = new ImageUpscaleWithModelNode('6')
		// 	.setUpscaleModel(upscale_loader.id, 0)
		// 	.setImage(reactor.id, 0);

		const RMBG = new RMBGNode('1').setModel(REMBGModel).setImage([upscaleImagewModel.id, 0]);

		const adPixel = new AddPixelNode('3').setImage(RMBG.id, 0).setMask(RMBG.id, 1);

		const saveImage = new SaveImageNode('4')
			.setFilenamePrefix('[WITHBG] ')
			.setImages(upscaleImagewModel.id, 0);

		const saveImageRemBG = new SaveImageNode('5')
			.setFilenamePrefix('[REMBG] ')
			.setImages(adPixel.id, 0);
		try {
			if (!removeBackground) {
				let wf = workflow(loadImageNode, saveImage, upscaleImagewModel, reactor, upscale_loader);
				resolve(wf.toMap());
			} else {
				let wf = workflow(
					RMBG,
					loadImageNode,
					adPixel,
					saveImage,
					saveImageRemBG,
					upscaleImagewModel,
					reactor,
					upscale_loader
				);
				resolve(wf.toMap());
			}
		} catch (error) {
			if (error instanceof DuplicateIdError) {
				console.error('Duplicate ID:', error.duplicateId);
				reject('Duplicate ID: ' + error.duplicateId);
			} else if (error instanceof EmptyIdError) {
				reject('Empty ID found');
			}
		}
	});
}
