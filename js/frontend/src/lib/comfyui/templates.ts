import { randomSeed } from './utils';

export const sdUpscaleTemplate = (source_image: string) => {
	const random_seed = randomSeed();
	return {
		'1': {
			inputs: {
				upscale_by: 1.2,
				seed: 239414931582311,
				steps: 6,
				cfg: 1,
				sampler_name: 'lcm',
				scheduler: 'simple',
				denoise: 0.1,
				mode_type: 'Linear',
				tile_width: 512,
				tile_height: 512,
				mask_blur: 8,
				tile_padding: 32,
				seam_fix_mode: 'None',
				seam_fix_denoise: 1,
				seam_fix_width: 64,
				seam_fix_mask_blur: 8,
				seam_fix_padding: 16,
				force_uniform_tiles: true,
				tiled_decode: false,
				image: ['8', 0],
				model: ['3', 0],
				positive: ['4', 0],
				negative: ['5', 0],
				vae: ['3', 2],
				upscale_model: ['6', 0]
			},
			class_type: 'UltimateSDUpscale',
			_meta: {
				title: 'Ultimate SD Upscale'
			}
		},
		'2': {
			inputs: {
				image: source_image,
				value: 512
			},
			class_type: 'LoadResizeImageMask512',
			_meta: {
				title: 'Load Image n Resize to 512'
			}
		},
		'3': {
			inputs: {
				ckpt_name: 'dreamshaper_8LCM.safetensors'
			},
			class_type: 'CheckpointLoaderSimple',
			_meta: {
				title: 'Load Checkpoint'
			}
		},
		'4': {
			inputs: {
				text: '',
				clip: ['3', 1]
			},
			class_type: 'CLIPTextEncode',
			_meta: {
				title: 'CLIP Text Encode (Prompt)'
			}
		},
		'5': {
			inputs: {
				text: '',
				clip: ['3', 1]
			},
			class_type: 'CLIPTextEncode',
			_meta: {
				title: 'CLIP Text Encode (Prompt)'
			}
		},
		'6': {
			inputs: {
				model_name: 'RealESRGAN_x2plus.pth'
			},
			class_type: 'UpscaleModelLoader',
			_meta: {
				title: 'Load Upscale Model'
			}
		},
		'8': {
			inputs: {
				facedetection: 'retinaface_resnet50',
				model: 'GFPGANv1.4.pth',
				visibility: 1,
				codeformer_weight: 0.5,
				image: ['2', 0]
			},
			class_type: 'ReActorRestoreFace',
			_meta: {
				title: 'Restore Face 🌌 ReActor'
			}
		},
		'9': {
			inputs: {
				enabled: true,
				swap_model: 'inswapper_128.onnx',
				facedetection: 'retinaface_resnet50',
				face_restore_model: 'GFPGANv1.4.pth',
				face_restore_visibility: 1,
				codeformer_weight: 0.5,
				detect_gender_input: 'no',
				detect_gender_source: 'no',
				input_faces_index: '0',
				source_faces_index: '0',
				console_log_level: 0,
				input_image: ['1', 0],
				source_image: ['12', 0]
			},
			class_type: 'ReActorFaceSwap',
			_meta: {
				title: 'ReActor 🌌 Fast Face Swap'
			}
		},
		'12': {
			inputs: {
				upscale_model: ['6', 0],
				image: ['8', 0]
			},
			class_type: 'ImageUpscaleWithModel',
			_meta: {
				title: 'Upscale Image (using Model)'
			}
		},
		'13': {
			inputs: {
				model: 'RMBG-2.0',
				sensitivity: 1,
				process_res: 512,
				mask_blur: 0,
				mask_offset: 0,
				invert_output: false,
				refine_foreground: false,
				background: 'Alpha',
				background_color: '#222222',
				image: ['1', 0]
			},
			class_type: 'RMBG',
			_meta: {
				title: 'Remove Background (RMBG)'
			}
		},
		'15': {
			inputs: {
				filename_prefix: '[SDUPSCALE]',
				images: ['1', 0]
			},
			class_type: 'SaveImage',
			_meta: {
				title: 'Save Image'
			}
		},
		'16': {
			inputs: {
				filename_prefix: '{RESTORE FACE]',
				images: ['9', 0]
			},
			class_type: 'SaveImage',
			_meta: {
				title: 'Save Image'
			}
		},
		'17': {
			inputs: {
				filename_prefix: '[REMBG]',
				images: ['18', 0]
			},
			class_type: 'SaveImage',
			_meta: {
				title: 'Save Image'
			}
		},
		'18': {
			inputs: {
				image: ['13', 0],
				mask: ['13', 1]
			},
			class_type: 'AddPixelNode',
			_meta: {
				title: 'Add Pixel Node'
			}
		}
	};
};
