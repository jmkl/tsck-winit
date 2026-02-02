use crate::{
    event::{TS_PATH, UserEvent},
    ts_struct,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_struct! {path = TS_PATH,
    struct TemplateLine {
        id: i32,
        text: String,
        scale: i32,
        #[ts(optional)]
        font_size: Option<i32>,
        #[ts(optional)]
        font_height: Option<u32>,
        italic: bool,
        include: bool,
    }
}
const TEXT_SIZE: [i32; 6] = [80, 90, 98, 105, 115, 120];
ts_struct! {path = TS_PATH,
        pub struct Template {
            name: String,
            #[ts(optional)]
            total_height: Option<u32>,
            #[serde(default)]
            gap: u32,
            #[serde(default)]
            padding: u32,
            content: Vec<TemplateLine>,
        }
}

impl Default for Template {
    fn default() -> Self {
        Self {
            name: Default::default(),
            total_height: None,
            gap: 10,
            padding: 30,
            content: Default::default(),
        }
    }
}

impl Template {
    pub fn modify(mut self) -> Template {
        let keren_cadas = self.name.contains("KERENCADAS");
        let gap = self.gap;
        for t in &mut self.content {
            if let Some(&fnt_size) = TEXT_SIZE.get(t.scale as usize) {
                t.font_size = Some(fnt_size);
                let which_font = match keren_cadas {
                    true => FONT_ANTON,
                    false => FONT_UNISANS,
                };
                t.font_height = Some(font_height(fnt_size as u32, which_font));
            }
        }
        let total_height =
            self.content
                .iter()
                .filter(|c| c.include)
                .enumerate()
                .fold(0, |acc, (i, line)| {
                    let height = line.font_height.unwrap_or_default();
                    let gap_amount = if i < self.content.len() - 1 { gap } else { 0 };
                    acc + height + gap_amount
                });
        self.total_height = Some(total_height);

        self
    }
    #[allow(unused)]
    pub fn stringify(self) -> String {
        serde_json::to_string_pretty(&UserEvent::Template { template: self }).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FontData {
    pub fontsize: u32,
    pub height: u32,
}
pub const FONT_UNISANS: &[FontData] = &[
    FontData {
        fontsize: 50,
        height: 37,
    },
    FontData {
        fontsize: 51,
        height: 38,
    },
    FontData {
        fontsize: 52,
        height: 39,
    },
    FontData {
        fontsize: 53,
        height: 40,
    },
    FontData {
        fontsize: 54,
        height: 40,
    },
    FontData {
        fontsize: 55,
        height: 41,
    },
    FontData {
        fontsize: 56,
        height: 42,
    },
    FontData {
        fontsize: 57,
        height: 42,
    },
    FontData {
        fontsize: 58,
        height: 43,
    },
    FontData {
        fontsize: 59,
        height: 44,
    },
    FontData {
        fontsize: 60,
        height: 44,
    },
    FontData {
        fontsize: 61,
        height: 45,
    },
    FontData {
        fontsize: 62,
        height: 46,
    },
    FontData {
        fontsize: 63,
        height: 47,
    },
    FontData {
        fontsize: 64,
        height: 47,
    },
    FontData {
        fontsize: 65,
        height: 48,
    },
    FontData {
        fontsize: 66,
        height: 49,
    },
    FontData {
        fontsize: 67,
        height: 49,
    },
    FontData {
        fontsize: 68,
        height: 50,
    },
    FontData {
        fontsize: 69,
        height: 51,
    },
    FontData {
        fontsize: 70,
        height: 52,
    },
    FontData {
        fontsize: 71,
        height: 52,
    },
    FontData {
        fontsize: 72,
        height: 53,
    },
    FontData {
        fontsize: 73,
        height: 54,
    },
    FontData {
        fontsize: 74,
        height: 54,
    },
    FontData {
        fontsize: 75,
        height: 55,
    },
    FontData {
        fontsize: 76,
        height: 56,
    },
    FontData {
        fontsize: 77,
        height: 57,
    },
    FontData {
        fontsize: 78,
        height: 57,
    },
    FontData {
        fontsize: 79,
        height: 58,
    },
    FontData {
        fontsize: 80,
        height: 59,
    },
    FontData {
        fontsize: 81,
        height: 59,
    },
    FontData {
        fontsize: 82,
        height: 60,
    },
    FontData {
        fontsize: 83,
        height: 61,
    },
    FontData {
        fontsize: 84,
        height: 61,
    },
    FontData {
        fontsize: 85,
        height: 62,
    },
    FontData {
        fontsize: 86,
        height: 63,
    },
    FontData {
        fontsize: 87,
        height: 64,
    },
    FontData {
        fontsize: 88,
        height: 64,
    },
    FontData {
        fontsize: 89,
        height: 65,
    },
    FontData {
        fontsize: 90,
        height: 66,
    },
    FontData {
        fontsize: 91,
        height: 66,
    },
    FontData {
        fontsize: 92,
        height: 67,
    },
    FontData {
        fontsize: 93,
        height: 68,
    },
    FontData {
        fontsize: 94,
        height: 69,
    },
    FontData {
        fontsize: 95,
        height: 69,
    },
    FontData {
        fontsize: 96,
        height: 70,
    },
    FontData {
        fontsize: 97,
        height: 71,
    },
    FontData {
        fontsize: 98,
        height: 71,
    },
    FontData {
        fontsize: 99,
        height: 72,
    },
    FontData {
        fontsize: 100,
        height: 73,
    },
    FontData {
        fontsize: 101,
        height: 75,
    },
    FontData {
        fontsize: 102,
        height: 74,
    },
    FontData {
        fontsize: 103,
        height: 76,
    },
    FontData {
        fontsize: 104,
        height: 77,
    },
    FontData {
        fontsize: 105,
        height: 77,
    },
    FontData {
        fontsize: 106,
        height: 77,
    },
    FontData {
        fontsize: 107,
        height: 79,
    },
    FontData {
        fontsize: 108,
        height: 80,
    },
    FontData {
        fontsize: 109,
        height: 80,
    },
    FontData {
        fontsize: 110,
        height: 81,
    },
    FontData {
        fontsize: 111,
        height: 81,
    },
    FontData {
        fontsize: 112,
        height: 82,
    },
    FontData {
        fontsize: 113,
        height: 83,
    },
    FontData {
        fontsize: 114,
        height: 84,
    },
    FontData {
        fontsize: 115,
        height: 84,
    },
    FontData {
        fontsize: 116,
        height: 85,
    },
    FontData {
        fontsize: 117,
        height: 86,
    },
    FontData {
        fontsize: 118,
        height: 87,
    },
    FontData {
        fontsize: 119,
        height: 87,
    },
    FontData {
        fontsize: 120,
        height: 88,
    },
    FontData {
        fontsize: 121,
        height: 89,
    },
    FontData {
        fontsize: 122,
        height: 89,
    },
    FontData {
        fontsize: 123,
        height: 90,
    },
    FontData {
        fontsize: 124,
        height: 91,
    },
    FontData {
        fontsize: 125,
        height: 93,
    },
    FontData {
        fontsize: 126,
        height: 93,
    },
    FontData {
        fontsize: 127,
        height: 93,
    },
    FontData {
        fontsize: 128,
        height: 95,
    },
    FontData {
        fontsize: 129,
        height: 94,
    },
    FontData {
        fontsize: 130,
        height: 95,
    },
    FontData {
        fontsize: 131,
        height: 97,
    },
    FontData {
        fontsize: 132,
        height: 96,
    },
    FontData {
        fontsize: 133,
        height: 97,
    },
    FontData {
        fontsize: 134,
        height: 98,
    },
    FontData {
        fontsize: 135,
        height: 99,
    },
    FontData {
        fontsize: 136,
        height: 99,
    },
    FontData {
        fontsize: 137,
        height: 100,
    },
    FontData {
        fontsize: 138,
        height: 101,
    },
    FontData {
        fontsize: 139,
        height: 101,
    },
    FontData {
        fontsize: 140,
        height: 102,
    },
    FontData {
        fontsize: 141,
        height: 104,
    },
    FontData {
        fontsize: 142,
        height: 104,
    },
    FontData {
        fontsize: 143,
        height: 104,
    },
    FontData {
        fontsize: 144,
        height: 106,
    },
    FontData {
        fontsize: 145,
        height: 106,
    },
    FontData {
        fontsize: 146,
        height: 106,
    },
    FontData {
        fontsize: 147,
        height: 107,
    },
    FontData {
        fontsize: 148,
        height: 108,
    },
    FontData {
        fontsize: 149,
        height: 109,
    },
    FontData {
        fontsize: 150,
        height: 111,
    },
    FontData {
        fontsize: 151,
        height: 112,
    },
    FontData {
        fontsize: 152,
        height: 113,
    },
    FontData {
        fontsize: 153,
        height: 113,
    },
    FontData {
        fontsize: 154,
        height: 115,
    },
    FontData {
        fontsize: 155,
        height: 115,
    },
    FontData {
        fontsize: 156,
        height: 116,
    },
    FontData {
        fontsize: 157,
        height: 116,
    },
    FontData {
        fontsize: 158,
        height: 117,
    },
    FontData {
        fontsize: 159,
        height: 119,
    },
    FontData {
        fontsize: 160,
        height: 118,
    },
    FontData {
        fontsize: 161,
        height: 119,
    },
    FontData {
        fontsize: 162,
        height: 120,
    },
    FontData {
        fontsize: 163,
        height: 120,
    },
    FontData {
        fontsize: 164,
        height: 122,
    },
    FontData {
        fontsize: 165,
        height: 122,
    },
    FontData {
        fontsize: 166,
        height: 123,
    },
    FontData {
        fontsize: 167,
        height: 124,
    },
    FontData {
        fontsize: 168,
        height: 124,
    },
    FontData {
        fontsize: 169,
        height: 125,
    },
    FontData {
        fontsize: 170,
        height: 125,
    },
    FontData {
        fontsize: 171,
        height: 126,
    },
    FontData {
        fontsize: 172,
        height: 127,
    },
    FontData {
        fontsize: 173,
        height: 127,
    },
    FontData {
        fontsize: 174,
        height: 128,
    },
    FontData {
        fontsize: 175,
        height: 128,
    },
    FontData {
        fontsize: 176,
        height: 130,
    },
    FontData {
        fontsize: 177,
        height: 131,
    },
    FontData {
        fontsize: 178,
        height: 130,
    },
    FontData {
        fontsize: 179,
        height: 132,
    },
    FontData {
        fontsize: 180,
        height: 132,
    },
    FontData {
        fontsize: 181,
        height: 132,
    },
    FontData {
        fontsize: 182,
        height: 135,
    },
    FontData {
        fontsize: 183,
        height: 136,
    },
    FontData {
        fontsize: 184,
        height: 135,
    },
    FontData {
        fontsize: 185,
        height: 136,
    },
    FontData {
        fontsize: 186,
        height: 138,
    },
    FontData {
        fontsize: 187,
        height: 137,
    },
    FontData {
        fontsize: 188,
        height: 138,
    },
    FontData {
        fontsize: 189,
        height: 138,
    },
    FontData {
        fontsize: 190,
        height: 140,
    },
    FontData {
        fontsize: 191,
        height: 140,
    },
    FontData {
        fontsize: 192,
        height: 141,
    },
    FontData {
        fontsize: 193,
        height: 141,
    },
    FontData {
        fontsize: 194,
        height: 142,
    },
    FontData {
        fontsize: 195,
        height: 143,
    },
    FontData {
        fontsize: 196,
        height: 143,
    },
    FontData {
        fontsize: 197,
        height: 144,
    },
    FontData {
        fontsize: 198,
        height: 145,
    },
    FontData {
        fontsize: 199,
        height: 145,
    },
];

pub static FONT_ANTON: &[FontData] = &[
    FontData {
        fontsize: 50,
        height: 43,
    },
    FontData {
        fontsize: 51,
        height: 44,
    },
    FontData {
        fontsize: 52,
        height: 45,
    },
    FontData {
        fontsize: 53,
        height: 45,
    },
    FontData {
        fontsize: 54,
        height: 46,
    },
    FontData {
        fontsize: 55,
        height: 48,
    },
    FontData {
        fontsize: 56,
        height: 49,
    },
    FontData {
        fontsize: 57,
        height: 49,
    },
    FontData {
        fontsize: 58,
        height: 50,
    },
    FontData {
        fontsize: 59,
        height: 51,
    },
    FontData {
        fontsize: 60,
        height: 52,
    },
    FontData {
        fontsize: 61,
        height: 52,
    },
    FontData {
        fontsize: 62,
        height: 53,
    },
    FontData {
        fontsize: 63,
        height: 56,
    },
    FontData {
        fontsize: 64,
        height: 58,
    },
    FontData {
        fontsize: 65,
        height: 58,
    },
    FontData {
        fontsize: 66,
        height: 59,
    },
    FontData {
        fontsize: 67,
        height: 60,
    },
    FontData {
        fontsize: 68,
        height: 60,
    },
    FontData {
        fontsize: 69,
        height: 61,
    },
    FontData {
        fontsize: 70,
        height: 62,
    },
    FontData {
        fontsize: 71,
        height: 64,
    },
    FontData {
        fontsize: 72,
        height: 64,
    },
    FontData {
        fontsize: 73,
        height: 65,
    },
    FontData {
        fontsize: 74,
        height: 66,
    },
    FontData {
        fontsize: 75,
        height: 67,
    },
    FontData {
        fontsize: 76,
        height: 67,
    },
    FontData {
        fontsize: 77,
        height: 68,
    },
    FontData {
        fontsize: 78,
        height: 69,
    },
    FontData {
        fontsize: 79,
        height: 70,
    },
    FontData {
        fontsize: 80,
        height: 71,
    },
    FontData {
        fontsize: 81,
        height: 72,
    },
    FontData {
        fontsize: 82,
        height: 73,
    },
    FontData {
        fontsize: 83,
        height: 74,
    },
    FontData {
        fontsize: 84,
        height: 74,
    },
    FontData {
        fontsize: 85,
        height: 75,
    },
    FontData {
        fontsize: 86,
        height: 76,
    },
    FontData {
        fontsize: 87,
        height: 77,
    },
    FontData {
        fontsize: 88,
        height: 77,
    },
    FontData {
        fontsize: 89,
        height: 79,
    },
    FontData {
        fontsize: 90,
        height: 80,
    },
    FontData {
        fontsize: 91,
        height: 80,
    },
    FontData {
        fontsize: 92,
        height: 81,
    },
    FontData {
        fontsize: 93,
        height: 82,
    },
    FontData {
        fontsize: 94,
        height: 83,
    },
    FontData {
        fontsize: 95,
        height: 83,
    },
    FontData {
        fontsize: 96,
        height: 84,
    },
    FontData {
        fontsize: 97,
        height: 86,
    },
    FontData {
        fontsize: 98,
        height: 87,
    },
    FontData {
        fontsize: 99,
        height: 87,
    },
    FontData {
        fontsize: 100,
        height: 88,
    },
    FontData {
        fontsize: 101,
        height: 89,
    },
    FontData {
        fontsize: 102,
        height: 90,
    },
    FontData {
        fontsize: 103,
        height: 90,
    },
    FontData {
        fontsize: 104,
        height: 91,
    },
    FontData {
        fontsize: 105,
        height: 93,
    },
    FontData {
        fontsize: 106,
        height: 94,
    },
    FontData {
        fontsize: 107,
        height: 94,
    },
    FontData {
        fontsize: 108,
        height: 95,
    },
    FontData {
        fontsize: 109,
        height: 96,
    },
    FontData {
        fontsize: 110,
        height: 97,
    },
    FontData {
        fontsize: 111,
        height: 97,
    },
    FontData {
        fontsize: 112,
        height: 98,
    },
    FontData {
        fontsize: 113,
        height: 100,
    },
    FontData {
        fontsize: 114,
        height: 101,
    },
    FontData {
        fontsize: 115,
        height: 101,
    },
    FontData {
        fontsize: 116,
        height: 102,
    },
    FontData {
        fontsize: 117,
        height: 103,
    },
    FontData {
        fontsize: 118,
        height: 103,
    },
    FontData {
        fontsize: 119,
        height: 104,
    },
    FontData {
        fontsize: 120,
        height: 105,
    },
    FontData {
        fontsize: 121,
        height: 106,
    },
    FontData {
        fontsize: 122,
        height: 106,
    },
    FontData {
        fontsize: 123,
        height: 108,
    },
    FontData {
        fontsize: 124,
        height: 109,
    },
    FontData {
        fontsize: 125,
        height: 110,
    },
    FontData {
        fontsize: 126,
        height: 110,
    },
    FontData {
        fontsize: 127,
        height: 111,
    },
    FontData {
        fontsize: 128,
        height: 112,
    },
    FontData {
        fontsize: 129,
        height: 113,
    },
    FontData {
        fontsize: 130,
        height: 113,
    },
    FontData {
        fontsize: 131,
        height: 115,
    },
    FontData {
        fontsize: 132,
        height: 116,
    },
    FontData {
        fontsize: 133,
        height: 117,
    },
    FontData {
        fontsize: 134,
        height: 117,
    },
    FontData {
        fontsize: 135,
        height: 118,
    },
    FontData {
        fontsize: 136,
        height: 119,
    },
    FontData {
        fontsize: 137,
        height: 120,
    },
    FontData {
        fontsize: 138,
        height: 120,
    },
    FontData {
        fontsize: 139,
        height: 122,
    },
    FontData {
        fontsize: 140,
        height: 123,
    },
    FontData {
        fontsize: 141,
        height: 124,
    },
    FontData {
        fontsize: 142,
        height: 124,
    },
    FontData {
        fontsize: 143,
        height: 125,
    },
    FontData {
        fontsize: 144,
        height: 126,
    },
    FontData {
        fontsize: 145,
        height: 126,
    },
    FontData {
        fontsize: 146,
        height: 127,
    },
    FontData {
        fontsize: 147,
        height: 129,
    },
    FontData {
        fontsize: 148,
        height: 130,
    },
    FontData {
        fontsize: 149,
        height: 130,
    },
    FontData {
        fontsize: 150,
        height: 131,
    },
    FontData {
        fontsize: 151,
        height: 132,
    },
    FontData {
        fontsize: 152,
        height: 133,
    },
    FontData {
        fontsize: 153,
        height: 133,
    },
    FontData {
        fontsize: 154,
        height: 134,
    },
    FontData {
        fontsize: 155,
        height: 136,
    },
    FontData {
        fontsize: 156,
        height: 137,
    },
    FontData {
        fontsize: 157,
        height: 137,
    },
    FontData {
        fontsize: 158,
        height: 138,
    },
    FontData {
        fontsize: 159,
        height: 139,
    },
    FontData {
        fontsize: 160,
        height: 140,
    },
    FontData {
        fontsize: 161,
        height: 140,
    },
    FontData {
        fontsize: 162,
        height: 141,
    },
    FontData {
        fontsize: 163,
        height: 142,
    },
    FontData {
        fontsize: 164,
        height: 144,
    },
    FontData {
        fontsize: 165,
        height: 144,
    },
    FontData {
        fontsize: 166,
        height: 145,
    },
    FontData {
        fontsize: 167,
        height: 146,
    },
    FontData {
        fontsize: 168,
        height: 146,
    },
    FontData {
        fontsize: 169,
        height: 147,
    },
    FontData {
        fontsize: 170,
        height: 148,
    },
    FontData {
        fontsize: 171,
        height: 149,
    },
    FontData {
        fontsize: 172,
        height: 149,
    },
    FontData {
        fontsize: 173,
        height: 151,
    },
    FontData {
        fontsize: 174,
        height: 152,
    },
    FontData {
        fontsize: 175,
        height: 153,
    },
    FontData {
        fontsize: 176,
        height: 153,
    },
    FontData {
        fontsize: 177,
        height: 154,
    },
    FontData {
        fontsize: 178,
        height: 155,
    },
    FontData {
        fontsize: 179,
        height: 156,
    },
    FontData {
        fontsize: 180,
        height: 156,
    },
    FontData {
        fontsize: 181,
        height: 158,
    },
    FontData {
        fontsize: 182,
        height: 159,
    },
    FontData {
        fontsize: 183,
        height: 160,
    },
    FontData {
        fontsize: 184,
        height: 160,
    },
    FontData {
        fontsize: 185,
        height: 161,
    },
    FontData {
        fontsize: 186,
        height: 162,
    },
    FontData {
        fontsize: 187,
        height: 163,
    },
    FontData {
        fontsize: 188,
        height: 163,
    },
    FontData {
        fontsize: 189,
        height: 165,
    },
    FontData {
        fontsize: 190,
        height: 166,
    },
    FontData {
        fontsize: 191,
        height: 167,
    },
    FontData {
        fontsize: 192,
        height: 167,
    },
    FontData {
        fontsize: 193,
        height: 168,
    },
    FontData {
        fontsize: 194,
        height: 169,
    },
    FontData {
        fontsize: 195,
        height: 169,
    },
    FontData {
        fontsize: 196,
        height: 170,
    },
    FontData {
        fontsize: 197,
        height: 171,
    },
    FontData {
        fontsize: 198,
        height: 173,
    },
    FontData {
        fontsize: 199,
        height: 173,
    },
];

// fn calculate_total_height(lines: &[TemplateLine], gap: u32, kerencadas: bool) -> u32 {
//     let font_data = if kerencadas {
//         &FONT_ANTON
//     } else {
//         &FONT_UNI_SANS
//     };

//     lines.iter().enumerate().fold(0, |acc, (i, line)| {
//         let height = font_height(TEXT_SIZE[line.scale - 1], font_data);
//         let gap_amount = if i < lines.len() - 1 { gap } else { 0 };
//         acc + height + gap_amount
//     })
// }

fn font_height(fontsize: u32, data: &[FontData]) -> u32 {
    data.windows(2)
        .find_map(|window| {
            let curr = &window[0];
            let next = &window[1];

            if fontsize >= curr.fontsize && fontsize <= next.fontsize {
                let ratio =
                    (fontsize - curr.fontsize) as f32 / (next.fontsize - curr.fontsize) as f32;
                let height = curr.height as f32 + ratio * (next.height - curr.height) as f32;
                Some(height.round() as u32)
            } else {
                None
            }
        })
        .unwrap_or_else(|| data.last().map(|d| d.height).unwrap_or(0))
}
#[allow(unused)]
fn px_to_pt(px: f32) -> f32 {
    4.315_f32 * px + 2.797_f32
}

#[allow(unused)]
fn font_height_unisans(fontsize: u32) -> u32 {
    font_height(fontsize, FONT_UNISANS)
}
