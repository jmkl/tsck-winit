function inferTsType(value: unknown): string {
	if (typeof value === 'string') return 'string';
	if (typeof value === 'number') return 'number';
	if (typeof value === 'boolean') return 'boolean';
	if (Array.isArray(value)) return '[string,number]';
	if (value === null) return 'any';
	return 'any';
}
function serializeTsValue(value: unknown): string {
	if (typeof value === 'string') return `"${value}"`;
	if (typeof value === 'number') return String(value);
	if (typeof value === 'boolean') return String(value);
	if (Array.isArray(value)) {
		return `[${value.map(serializeTsValue).join(', ')}]`;
	}
	if (value === null) return 'null';
	if (typeof value === 'object') {
		return JSON.stringify(value, null, 2);
	}
	return 'undefined';
}

export function generateNodeClassFromJson(
	className: string,
	node: {
		inputs: Record<string, any>;
		class_type: string;
		_meta: { title: string };
	}
): string {
	const fields = Object.entries(node.inputs)
		.map(([key, value]) => {
			const type = inferTsType(value);
			const defaultValue = serializeTsValue(value);

			return `  public ${key}: ${type} = ${defaultValue};`;
		})
		.join('\n');

	const setters = Object.entries(node.inputs)
		.map(([key, value]) => {
			const type = inferTsType(value);
			const method = 'set' + key.charAt(0).toUpperCase() + key.slice(1);

			return `
  ${method}(${key}: ${type}): this {
    this.${key} = ${key};
    return this;
  }`;
		})
		.join('\n');

	const inputsObject = Object.keys(node.inputs)
		.map((key) => `        ${key}: this.${key},`)
		.join('\n');

	return `
// auto_gen for ${className} Node
import { type NodeDefinition, type Node } from '../nodes';

export class ${className}Node implements Node {
  public id: string;

${fields}

  constructor(id: string) {
    this.id = id;
  }
${setters}

  toDefinition(): NodeDefinition {
    return {
      inputs: {
${inputsObject}
      },
      class_type: "${node.class_type}",
      _meta: {
        title: "${node._meta.title}",
      },
    };
  }
}
`;
}
