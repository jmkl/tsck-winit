// auto_gen for ImageUpscaleWithModel Node
import { type NodeDefinition, type Node } from '../nodes';

export class ImageUpscaleWithModelNode implements Node {
	public id: string;

	public upscale_model: [string, number] = ['7', 0];
	public image: [string, number] = ['8', 0];

	constructor(id: string) {
		this.id = id;
	}

	setUpscale_model(upscale_model: [string, number]): this {
		this.upscale_model = upscale_model;
		return this;
	}

	setImage(image: [string, number]): this {
		this.image = image;
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
