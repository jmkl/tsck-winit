export const RAWFILTER_WHITELIST = ['texture', 'clarity', 'dehaze', 'sharpen', 'noise_reduction'];

export const RAWFILTER_DATA = [
	{ name: 'temp', min: -100, max: 100, value: 0, step: 1 },
	{ name: 'tint', min: -100, max: 100, value: 0, step: 1 },
	{ name: 'texture', min: -100, max: 100, value: 0, step: 1 },
	{ name: 'clarity', min: -100, max: 100, value: 0, step: 1 },
	{ name: 'dehaze', min: -100, max: 100, value: 0, step: 1 },
	{ name: 'sharpen', min: 0, max: 150, value: 0, step: 1 },
	{ name: 'sharpen_radius', min: 0.5, max: 3.0, value: 1.0, step: 0.1 },
	{ name: 'sharpen_detail', min: 0, max: 100, value: 25, step: 1 },
	{ name: 'noise_reduction', min: 0, max: 100, value: 0, step: 1 },
	{
		name: 'noise_reduction_detail',
		min: 0,
		max: 100,
		value: 50,
		step: 1
	}
];

export const RAWFILTER_VALUE_DEFAULT = [
	{ name: 'temp', value: 0 },
	{ name: 'tint', value: 0 },
	{ name: 'texture', value: 0 },
	{ name: 'clarity', value: 0 },
	{ name: 'dehaze', value: 0 },
	{ name: 'sharpen', value: 0 },
	{ name: 'sharpen_radius', value: 1.0 },
	{ name: 'sharpen_detail', value: 25 },
	{ name: 'noise_reduction', value: 0 },
	{ name: 'noise_reduction_detail', value: 50 }
];

interface CameraRawFilterParam {
	id: number;
	key: string;
	name: string;
	min: number;
	max: number;
	value: number;
	step: number;
}

export const cameraRawFilterDefaultData: CameraRawFilterParam[] = [
	{
		id: 1,
		key: '$Ex12',
		name: 'light_exposure',
		min: -5.0,
		max: 5.0,
		value: 0.0,
		step: 0.1
	},
	{
		id: 2,
		key: '$Cr12',
		name: 'light_contrast',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 3,
		key: '$Hi12',
		name: 'light_highlights',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 4,
		key: '$Sh12',
		name: 'light_shadows',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 5,
		key: '$Wh12',
		name: 'light_whites',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 6,
		key: '$Bk12',
		name: 'light_blacks',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 7,
		key: '$Temp',
		name: 'color_temp',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 8,
		key: '$Tint',
		name: 'color_tint',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 9,
		key: '$Vibr',
		name: 'color_vibrance',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 10,
		key: 'saturation',
		name: 'color_saturation',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 11,
		key: '$CrTx',
		name: 'effects_texture',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 12,
		key: '$Cl12',
		name: 'effects_clarity',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 13,
		key: '$Dhze',
		name: 'effects_dehaze',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 14,
		key: '$PCVA',
		name: 'effects_vignette',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 15,
		key: '$PCVM',
		name: 'effects_vignette_midpoint',
		min: 0,
		max: 100,
		value: 50,
		step: 1
	},
	{
		id: 16,
		key: '$PCVR',
		name: 'effects_vignette_roundness',
		min: -100,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 17,
		key: '$PCVF',
		name: 'effects_vignette_feather',
		min: 0,
		max: 100,
		value: 50,
		step: 1
	},
	{
		id: 18,
		key: '$PCVS',
		name: 'effects_vignette_highlights',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 19,
		key: '$GRNA',
		name: 'effects_grain',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 20,
		key: '$GRNS',
		name: 'effects_grain_size',
		min: 0,
		max: 100,
		value: 25,
		step: 1
	},
	{
		id: 21,
		key: '$GRNF',
		name: 'effects_grain_roughness',
		min: 0,
		max: 100,
		value: 50,
		step: 1
	},
	{
		id: 22,
		key: 'sharpen',
		name: 'detail_sharpen',
		min: 0,
		max: 150,
		value: 0,
		step: 1
	},
	{
		id: 23,
		key: '$ShpR',
		name: 'detail_sharpen_radius',
		min: 0.5,
		max: 3.0,
		value: 1.0,
		step: 0.1
	},
	{
		id: 24,
		key: '$ShpD',
		name: 'detail_sharpen_detail',
		min: 0,
		max: 100,
		value: 25,
		step: 1
	},
	{
		id: 25,
		key: '$ShpM',
		name: 'detail_sharpen_masking',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 26,
		key: '$LNR',
		name: 'detail_nr',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 27,
		key: '$LNRD',
		name: 'detail_nr_detail',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 28,
		key: '$LNRC',
		name: 'detail_nr_contrast',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 29,
		key: '$CNR',
		name: 'detail_colornr',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 30,
		key: '$CNRD',
		name: 'detail_colornr_detail',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	},
	{
		id: 31,
		key: '$CNRS',
		name: 'detail_colornr_smooth',
		min: 0,
		max: 100,
		value: 0,
		step: 1
	}
];

const filterFX = {
	filterFX: [
		{
			_obj: 'filterFX',
			name: 'Camera Raw Filter',
			filter: {
				_obj: 'Adobe Camera Raw Filter',
				$CrVe: '18.1.1',
				$PrVN: 6,
				$PrVe: 251920384,
				$WBal: {
					_enum: '$WBal',
					_value: 'customEnum'
				},
				$Temp: -13,
				$Tint: -17,
				$Ex12: 0.2,
				$Sh12: 67,
				$Wh12: -54,
				$Bk12: 44,
				$CrTx: 100,
				$Cl12: 52,
				$Dhze: 41,
				sharpen: 150,
				$ShpR: 1.8,
				$ShpD: 64,
				$ShpM: 29,
				$LNR: 99,
				$LNRD: 69,
				$LNRC: 69,
				$CNR: 38,
				$CNRD: 50,
				$CNRS: 50,
				$GRNA: 39,
				$GRNS: 25,
				$GRNF: 50,
				$PCVA: -32,
				$PCVM: 50,
				$PCVF: 50,
				$PCVR: 0,
				$PCVS: 1,
				$PCVH: 0,
				$TMMs: 0,
				$PGTM: 0,
				RGBSetupClass: 0
			},
			filterID: 2739
		}
	]
};
