// types.ts
export interface NodeMeta {
	title: string;
}

export interface NodeDefinition {
	inputs: Record<string, any>;
	class_type: string;
	_meta: NodeMeta;
}

export interface Node {
	id: string;
	toDefinition(): NodeDefinition;
}

// workflow.ts
export class WorkflowError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'WorkflowError';
	}
}

export class DuplicateIdError extends WorkflowError {
	constructor(public duplicateId: string) {
		super(`Duplicate node ID found: '${duplicateId}'`);
		this.name = 'DuplicateIdError';
	}
}

export class EmptyIdError extends WorkflowError {
	constructor() {
		super('Node with empty ID found');
		this.name = 'EmptyIdError';
	}
}

export class Workflow {
	private nodes: Node[];

	constructor(nodes: Node[]) {
		// Check for duplicate IDs
		const seenIds = new Set<string>();

		for (const node of nodes) {
			const id = node.id;

			// Check for empty ID
			if (!id || id.trim() === '') {
				throw new EmptyIdError();
			}

			// Check for duplicate
			if (seenIds.has(id)) {
				throw new DuplicateIdError(id);
			}

			seenIds.add(id);
		}

		this.nodes = nodes;
	}

	toJSON(): string {
		return JSON.stringify(this.toMap(), null, 2);
	}

	toMap(): Record<string, NodeDefinition> {
		const workflow: Record<string, NodeDefinition> = {};
		for (const node of this.nodes) {
			workflow[node.id] = node.toDefinition();
		}
		return workflow;
	}
}

// Helper function to create workflow
export function workflow(...nodes: Node[]): Workflow {
	return new Workflow(nodes);
}

// nodes/RMBGNode.ts
class RMBGNode implements Node {
	public id: string;
	public model: string = 'RMBG-2.0';
	public sensitivity: number = 1;
	public process_res: number = 1024;
	public mask_blur: number = 0;
	public mask_offset: number = 0;
	public invert_output: boolean = false;
	public refine_foreground: boolean = false;
	public background: string = 'Alpha';
	public background_color: string = '#222222';
	public image: [string, number] = ['', 0];

	constructor(id: string) {
		this.id = id;
	}

	setModel(model: string): this {
		this.model = model;
		return this;
	}

	setSensitivity(sensitivity: number): this {
		this.sensitivity = sensitivity;
		return this;
	}

	setProcessRes(processRes: number): this {
		this.process_res = processRes;
		return this;
	}

	setMaskBlur(maskBlur: number): this {
		this.mask_blur = maskBlur;
		return this;
	}

	setMaskOffset(maskOffset: number): this {
		this.mask_offset = maskOffset;
		return this;
	}

	setInvertOutput(invertOutput: boolean): this {
		this.invert_output = invertOutput;
		return this;
	}

	setRefineForeground(refineForeground: boolean): this {
		this.refine_foreground = refineForeground;
		return this;
	}

	setBackground(background: string): this {
		this.background = background;
		return this;
	}

	setBackgroundColor(backgroundColor: string): this {
		this.background_color = backgroundColor;
		return this;
	}

	setImage(nodeId: string, output: number): this {
		this.image = [nodeId, output];
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				model: this.model,
				sensitivity: this.sensitivity,
				process_res: this.process_res,
				mask_blur: this.mask_blur,
				mask_offset: this.mask_offset,
				invert_output: this.invert_output,
				refine_foreground: this.refine_foreground,
				background: this.background,
				background_color: this.background_color,
				image: this.image
			},
			class_type: 'RMBG',
			_meta: {
				title: 'Remove Background (RMBG)'
			}
		};
	}
}

// nodes/LoadResizeImageMask512Node.ts
export class LoadResizeImageMask512Node implements Node {
	public id: string;
	public image: string = '';
	public value: number = 512;

	constructor(id: string) {
		this.id = id;
	}

	setImage(image: string): this {
		this.image = image;
		return this;
	}

	setValue(value: number): this {
		this.value = value;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				image: this.image,
				value: this.value
			},
			class_type: 'LoadResizeImageMask512',
			_meta: {
				title: 'Load Image n Resize to 512'
			}
		};
	}
}

// nodes/AddPixelNode.ts
export class AddPixelNode implements Node {
	public id: string;
	public image: [string, number] = ['', 0];
	public mask: [string, number] = ['', 0];

	constructor(id: string) {
		this.id = id;
	}

	setImage(nodeId: string, output: number): this {
		this.image = [nodeId, output];
		return this;
	}

	setMask(nodeId: string, output: number): this {
		this.mask = [nodeId, output];
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				image: this.image,
				mask: this.mask
			},
			class_type: 'AddPixelNode',
			_meta: {
				title: 'Add Pixel Node'
			}
		};
	}
}

// nodes/SaveImageNode.ts
export class SaveImageNode implements Node {
	public id: string;
	public filename_prefix: string = 'ComfyUI';
	public images: [string, number] = ['', 0];

	constructor(id: string) {
		this.id = id;
	}

	setFilenamePrefix(prefix: string): this {
		this.filename_prefix = prefix;
		return this;
	}

	setImages(nodeId: string, output: number): this {
		this.images = [nodeId, output];
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				filename_prefix: this.filename_prefix,
				images: this.images
			},
			class_type: 'SaveImage',
			_meta: {
				title: 'Save Image'
			}
		};
	}
}

// nodes/ImageUpscaleWithModelNode.ts
class ImageUpscaleWithModelNode implements Node {
	public id: string;
	public upscale_model: [string, number] = ['', 0];
	public image: [string, number] = ['', 0];

	constructor(id: string) {
		this.id = id;
	}

	setUpscaleModel(nodeId: string, output: number): this {
		this.upscale_model = [nodeId, output];
		return this;
	}

	setImage(nodeId: string, output: number): this {
		this.image = [nodeId, output];
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				upscale_model: this.upscale_model,
				image: this.image
			},
			class_type: 'ImageUpscaleWithModel',
			_meta: {
				title: 'Upscale Image (using Model)'
			}
		};
	}
}

// nodes/ReActorRestoreFaceNode.ts
export class ReActorRestoreFaceNode implements Node {
	public id: string;
	public facedetection: string = 'retinaface_resnet50';
	public model: string = 'GFPGANv1.4.pth';
	public visibility: number = 1;
	public codeformer_weight: number = 1;
	public image: [string, number] = ['', 0];

	constructor(id: string) {
		this.id = id;
	}

	setFacedetection(facedetection: string): this {
		this.facedetection = facedetection;
		return this;
	}

	setModel(model: string): this {
		this.model = model;
		return this;
	}

	setVisibility(visibility: number): this {
		this.visibility = visibility;
		return this;
	}

	setCodeformerWeight(weight: number): this {
		this.codeformer_weight = weight;
		return this;
	}

	setImage(nodeId: string, output: number): this {
		this.image = [nodeId, output];
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				facedetection: this.facedetection,
				model: this.model,
				visibility: this.visibility,
				codeformer_weight: this.codeformer_weight,
				image: this.image
			},
			class_type: 'ReActorRestoreFace',
			_meta: {
				title: 'Restore Face 🌌 ReActor'
			}
		};
	}
}

// nodes/UpscaleModelLoaderNode.ts
export class UpscaleModelLoaderNode implements Node {
	public id: string;
	public model_name: string = 'RealESRGAN_x2plus.pth';

	constructor(id: string) {
		this.id = id;
	}

	setModelName(modelName: string): this {
		this.model_name = modelName;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				model_name: this.model_name
			},
			class_type: 'UpscaleModelLoader',
			_meta: {
				title: 'Load Upscale Model'
			}
		};
	}
}

// nodes/UpscaleModelLoaderNode.ts
export class _Name implements Node {
	public id: string;
	public model_name: string = 'RealESRGAN_x2plus.pth';

	constructor(id: string) {
		this.id = id;
	}

	setModelName(modelName: string): this {
		this.model_name = modelName;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				model_name: this.model_name
			},
			class_type: 'UpscaleModelLoader',
			_meta: {
				title: 'Load Upscale Model'
			}
		};
	}
}

// auto_gen for UltimateSDUpscale Node
export class UltimateSDUpscaleNode implements Node {
	public id: string;

	public upscale_by: number = 2;
	public seed: number = 1047116002591133;
	public steps: number = 6;
	public cfg: number = 1;
	public sampler_name: string = 'lcm';
	public scheduler: string = 'simple';
	public denoise: number = 0.1;
	public mode_type: string = 'Linear';
	public tile_width: number = 512;
	public tile_height: number = 512;
	public mask_blur: number = 8;
	public tile_padding: number = 32;
	public seam_fix_mode: string = 'None';
	public seam_fix_denoise: number = 1;
	public seam_fix_width: number = 64;
	public seam_fix_mask_blur: number = 8;
	public seam_fix_padding: number = 16;
	public force_uniform_tiles: boolean = true;
	public tiled_decode: boolean = false;
	public image: any[] = ['8', 0];
	public model: any[] = ['3', 0];
	public positive: any[] = ['4', 0];
	public negative: any[] = ['5', 0];
	public vae: any[] = ['3', 2];
	public upscale_model: any[] = ['6', 0];

	constructor(id: string) {
		this.id = id;
	}

	setUpscale_by(upscale_by: number): this {
		this.upscale_by = upscale_by;
		return this;
	}

	setSeed(seed: number): this {
		this.seed = seed;
		return this;
	}

	setSteps(steps: number): this {
		this.steps = steps;
		return this;
	}

	setCfg(cfg: number): this {
		this.cfg = cfg;
		return this;
	}

	setSampler_name(sampler_name: string): this {
		this.sampler_name = sampler_name;
		return this;
	}

	setScheduler(scheduler: string): this {
		this.scheduler = scheduler;
		return this;
	}

	setDenoise(denoise: number): this {
		this.denoise = denoise;
		return this;
	}

	setMode_type(mode_type: string): this {
		this.mode_type = mode_type;
		return this;
	}

	setTile_width(tile_width: number): this {
		this.tile_width = tile_width;
		return this;
	}

	setTile_height(tile_height: number): this {
		this.tile_height = tile_height;
		return this;
	}

	setMask_blur(mask_blur: number): this {
		this.mask_blur = mask_blur;
		return this;
	}

	setTile_padding(tile_padding: number): this {
		this.tile_padding = tile_padding;
		return this;
	}

	setSeam_fix_mode(seam_fix_mode: string): this {
		this.seam_fix_mode = seam_fix_mode;
		return this;
	}

	setSeam_fix_denoise(seam_fix_denoise: number): this {
		this.seam_fix_denoise = seam_fix_denoise;
		return this;
	}

	setSeam_fix_width(seam_fix_width: number): this {
		this.seam_fix_width = seam_fix_width;
		return this;
	}

	setSeam_fix_mask_blur(seam_fix_mask_blur: number): this {
		this.seam_fix_mask_blur = seam_fix_mask_blur;
		return this;
	}

	setSeam_fix_padding(seam_fix_padding: number): this {
		this.seam_fix_padding = seam_fix_padding;
		return this;
	}

	setForce_uniform_tiles(force_uniform_tiles: boolean): this {
		this.force_uniform_tiles = force_uniform_tiles;
		return this;
	}

	setTiled_decode(tiled_decode: boolean): this {
		this.tiled_decode = tiled_decode;
		return this;
	}

	setImage(image: any[]): this {
		this.image = image;
		return this;
	}

	setModel(model: any[]): this {
		this.model = model;
		return this;
	}

	setPositive(positive: any[]): this {
		this.positive = positive;
		return this;
	}

	setNegative(negative: any[]): this {
		this.negative = negative;
		return this;
	}

	setVae(vae: any[]): this {
		this.vae = vae;
		return this;
	}

	setUpscale_model(upscale_model: any[]): this {
		this.upscale_model = upscale_model;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				upscale_by: this.upscale_by,
				seed: this.seed,
				steps: this.steps,
				cfg: this.cfg,
				sampler_name: this.sampler_name,
				scheduler: this.scheduler,
				denoise: this.denoise,
				mode_type: this.mode_type,
				tile_width: this.tile_width,
				tile_height: this.tile_height,
				mask_blur: this.mask_blur,
				tile_padding: this.tile_padding,
				seam_fix_mode: this.seam_fix_mode,
				seam_fix_denoise: this.seam_fix_denoise,
				seam_fix_width: this.seam_fix_width,
				seam_fix_mask_blur: this.seam_fix_mask_blur,
				seam_fix_padding: this.seam_fix_padding,
				force_uniform_tiles: this.force_uniform_tiles,
				tiled_decode: this.tiled_decode,
				image: this.image,
				model: this.model,
				positive: this.positive,
				negative: this.negative,
				vae: this.vae,
				upscale_model: this.upscale_model
			},
			class_type: 'UltimateSDUpscale',
			_meta: {
				title: 'Ultimate SD Upscale'
			}
		};
	}
}

// auto_gen for CheckpointLoaderSimple Node
export class CheckpointLoaderSimpleNode implements Node {
	public id: string;

	public ckpt_name: string = 'dreamshaper_8LCM.safetensors';

	constructor(id: string) {
		this.id = id;
	}

	setCkpt_name(ckpt_name: string): this {
		this.ckpt_name = ckpt_name;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				ckpt_name: this.ckpt_name
			},
			class_type: 'CheckpointLoaderSimple',
			_meta: {
				title: 'Load Checkpoint'
			}
		};
	}
}

// auto_gen for CLIPTextEncode Node
export class CLIPTextEncodeNode implements Node {
	public id: string;
	public text: string = '';
	public clip: any[] = ['3', 1];

	constructor(id: string) {
		this.id = id;
	}

	setText(text: string): this {
		this.text = text;
		return this;
	}

	setClip(clip: any[]): this {
		this.clip = clip;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				text: this.text,
				clip: this.clip
			},
			class_type: 'CLIPTextEncode',
			_meta: {
				title: 'CLIP Text Encode (Prompt)'
			}
		};
	}
}

// auto_gen for ReActorFaceSwap Node
export class ReActorFaceSwapNode implements Node {
	public id: string;

	public enabled: boolean = true;
	public swap_model: string = 'inswapper_128.onnx';
	public facedetection: string = 'retinaface_resnet50';
	public face_restore_model: string = 'GFPGANv1.4.pth';
	public face_restore_visibility: number = 1;
	public codeformer_weight: number = 0.5;
	public detect_gender_input: string = 'no';
	public detect_gender_source: string = 'no';
	public input_faces_index: string = '0';
	public source_faces_index: string = '0';
	public console_log_level: number = 0;
	public input_image: any[] = ['1', 0];
	public source_image: any[] = ['12', 0];

	constructor(id: string) {
		this.id = id;
	}

	setEnabled(enabled: boolean): this {
		this.enabled = enabled;
		return this;
	}

	setSwap_model(swap_model: string): this {
		this.swap_model = swap_model;
		return this;
	}

	setFacedetection(facedetection: string): this {
		this.facedetection = facedetection;
		return this;
	}

	setFace_restore_model(face_restore_model: string): this {
		this.face_restore_model = face_restore_model;
		return this;
	}

	setFace_restore_visibility(face_restore_visibility: number): this {
		this.face_restore_visibility = face_restore_visibility;
		return this;
	}

	setCodeformer_weight(codeformer_weight: number): this {
		this.codeformer_weight = codeformer_weight;
		return this;
	}

	setDetect_gender_input(detect_gender_input: string): this {
		this.detect_gender_input = detect_gender_input;
		return this;
	}

	setDetect_gender_source(detect_gender_source: string): this {
		this.detect_gender_source = detect_gender_source;
		return this;
	}

	setInput_faces_index(input_faces_index: string): this {
		this.input_faces_index = input_faces_index;
		return this;
	}

	setSource_faces_index(source_faces_index: string): this {
		this.source_faces_index = source_faces_index;
		return this;
	}

	setConsole_log_level(console_log_level: number): this {
		this.console_log_level = console_log_level;
		return this;
	}

	setInput_image(input_image: any[]): this {
		this.input_image = input_image;
		return this;
	}

	setSource_image(source_image: any[]): this {
		this.source_image = source_image;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				enabled: this.enabled,
				swap_model: this.swap_model,
				facedetection: this.facedetection,
				face_restore_model: this.face_restore_model,
				face_restore_visibility: this.face_restore_visibility,
				codeformer_weight: this.codeformer_weight,
				detect_gender_input: this.detect_gender_input,
				detect_gender_source: this.detect_gender_source,
				input_faces_index: this.input_faces_index,
				source_faces_index: this.source_faces_index,
				console_log_level: this.console_log_level,
				input_image: this.input_image,
				source_image: this.source_image
			},
			class_type: 'ReActorFaceSwap',
			_meta: {
				title: 'ReActor 🌌 Fast Face Swap'
			}
		};
	}
}
