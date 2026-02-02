// auto_gen for RMBG Node
import { type NodeDefinition, type Node } from '../nodes';

export class RMBGNode implements Node {
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
	public image: [string, number] = ['6', 0];

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

	setProcess_res(process_res: number): this {
		this.process_res = process_res;
		return this;
	}

	setMask_blur(mask_blur: number): this {
		this.mask_blur = mask_blur;
		return this;
	}

	setMask_offset(mask_offset: number): this {
		this.mask_offset = mask_offset;
		return this;
	}

	setInvert_output(invert_output: boolean): this {
		this.invert_output = invert_output;
		return this;
	}

	setRefine_foreground(refine_foreground: boolean): this {
		this.refine_foreground = refine_foreground;
		return this;
	}

	setBackground(background: string): this {
		this.background = background;
		return this;
	}

	setBackground_color(background_color: string): this {
		this.background_color = background_color;
		return this;
	}

	setImage(image: [string, number]): this {
		this.image = image;
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
