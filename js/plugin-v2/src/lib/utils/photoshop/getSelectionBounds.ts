import { action } from "photoshop";
import { SelectionBound } from "@tsockee/frontend";

export async function getSelectionBounds(): Promise<SelectionBound> {
	const result = await action.batchPlay(
		[
			{
				_obj: "get",

				_target: [
					{
						_property: "selection",
					},

					{
						_ref: "document",

						_enum: "ordinal",

						_value: "targetEnum",
					},
				],
			},
		],
		{},
	);

	if (Object.hasOwn(result[0], "selection")) {
		const s = result[0].selection;
		return {
			selection_mode: s.right._value - s.left._value > 0,
			bounds: {
				top: s.top._value,
				bottom: s.bottom._value,
				right: s.right._value,
				left: s.left._value,
			},
		};
	} else {
		return {
			selection_mode: false,
			bounds: { top: 0, bottom: 0, left: 0, right: 0 },
		};
	}
}
