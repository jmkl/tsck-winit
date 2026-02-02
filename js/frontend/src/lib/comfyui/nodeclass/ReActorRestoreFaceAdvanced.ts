import { type NodeDefinition, type Node } from '../nodes';

// auto_gen for ReActorRestoreFaceAdvanced Node
export class ReActorRestoreFaceAdvancedNode implements Node {
	public id: string;

	public facedetection: string = 'retinaface_resnet50';
	public model: string = 'GFPGANv1.4.onnx';
	public visibility: number = 1;
	public codeformer_weight: number = 0.5;
	public face_selection: string = 'all';
	public sort_by: string = 'area';
	public reverse_order: boolean = false;
	public take_start: number = 0;
	public take_count: number = 1;
	public image: any[] = ['1', 0];

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

	setCodeformer_weight(codeformer_weight: number): this {
		this.codeformer_weight = codeformer_weight;
		return this;
	}

	setFace_selection(face_selection: string): this {
		this.face_selection = face_selection;
		return this;
	}

	setSort_by(sort_by: string): this {
		this.sort_by = sort_by;
		return this;
	}

	setReverse_order(reverse_order: boolean): this {
		this.reverse_order = reverse_order;
		return this;
	}

	setTake_start(take_start: number): this {
		this.take_start = take_start;
		return this;
	}

	setTake_count(take_count: number): this {
		this.take_count = take_count;
		return this;
	}

	setImage(image: [string, number]): this {
		this.image = image;
		return this;
	}

	toDefinition(): NodeDefinition {
		return {
			inputs: {
				facedetection: this.facedetection,
				model: this.model,
				visibility: this.visibility,
				codeformer_weight: this.codeformer_weight,
				face_selection: this.face_selection,
				sort_by: this.sort_by,
				reverse_order: this.reverse_order,
				take_start: this.take_start,
				take_count: this.take_count,
				image: this.image
			},
			class_type: 'ReActorRestoreFaceAdvanced',
			_meta: {
				title: 'Restore Face Advanced 🌌 ReActor'
			}
		};
	}
}
