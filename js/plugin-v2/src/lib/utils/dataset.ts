type FontData = {
  fontsize: number;
  height: number;
};

export const FontUniSans: FontData[] = [
  {
    fontsize: 80,
    height: 59,
  },
  {
    fontsize: 81,
    height: 59,
  },
  {
    fontsize: 82,
    height: 60,
  },
  {
    fontsize: 83,
    height: 61,
  },
  {
    fontsize: 84,
    height: 61,
  },
  {
    fontsize: 85,
    height: 62,
  },
  {
    fontsize: 86,
    height: 63,
  },
  {
    fontsize: 87,
    height: 64,
  },
  {
    fontsize: 88,
    height: 64,
  },
  {
    fontsize: 89,
    height: 65,
  },
  {
    fontsize: 90,
    height: 66,
  },
  {
    fontsize: 91,
    height: 66,
  },
  {
    fontsize: 92,
    height: 67,
  },
  {
    fontsize: 93,
    height: 68,
  },
  {
    fontsize: 94,
    height: 69,
  },
  {
    fontsize: 95,
    height: 69,
  },
  {
    fontsize: 96,
    height: 70,
  },
  {
    fontsize: 97,
    height: 71,
  },
  {
    fontsize: 98,
    height: 71,
  },
  {
    fontsize: 99,
    height: 72,
  },
  {
    fontsize: 100,
    height: 73,
  },
  {
    fontsize: 101,
    height: 74,
  },
  {
    fontsize: 102,
    height: 75,
  },
  {
    fontsize: 103,
    height: 76,
  },
  {
    fontsize: 104,
    height: 76,
  },
  {
    fontsize: 105,
    height: 76,
  },
  {
    fontsize: 106,
    height: 78,
  },
  {
    fontsize: 107,
    height: 79,
  },
  {
    fontsize: 108,
    height: 79,
  },
  {
    fontsize: 109,
    height: 80,
  },
  {
    fontsize: 110,
    height: 81,
  },
  {
    fontsize: 111,
    height: 81,
  },
  {
    fontsize: 112,
    height: 81,
  },
  {
    fontsize: 113,
    height: 83,
  },
  {
    fontsize: 114,
    height: 84,
  },
  {
    fontsize: 115,
    height: 83,
  },
  {
    fontsize: 116,
    height: 85,
  },
  {
    fontsize: 117,
    height: 86,
  },
  {
    fontsize: 118,
    height: 86,
  },
  {
    fontsize: 119,
    height: 87,
  },
  {
    fontsize: 120,
    height: 88,
  },
];
export const FontAnton: FontData[] = [
  {
    fontsize: 80,
    height: 71,
  },
  {
    fontsize: 82,
    height: 73,
  },
  {
    fontsize: 84,
    height: 74,
  },
  {
    fontsize: 86,
    height: 76,
  },
  {
    fontsize: 88,
    height: 77,
  },
  {
    fontsize: 90,
    height: 80,
  },
  {
    fontsize: 92,
    height: 81,
  },
  {
    fontsize: 94,
    height: 83,
  },
  {
    fontsize: 96,
    height: 84,
  },
  {
    fontsize: 98,
    height: 87,
  },
  {
    fontsize: 100,
    height: 88,
  },
  {
    fontsize: 102,
    height: 90,
  },
  {
    fontsize: 104,
    height: 91,
  },
  {
    fontsize: 106,
    height: 94,
  },
  {
    fontsize: 108,
    height: 95,
  },
  {
    fontsize: 110,
    height: 97,
  },
  {
    fontsize: 112,
    height: 98,
  },
  {
    fontsize: 114,
    height: 101,
  },
  {
    fontsize: 116,
    height: 102,
  },
  {
    fontsize: 118,
    height: 103,
  },
];

export function fontHeight(fontsize: number, which: FontData[]) {
  for (let i = 0; i < which.length - 1; i++) {
    const curr = which[i];
    const next = which[i + 1];
    if (fontsize >= curr.fontsize && fontsize <= next.fontsize) {
      const ratio =
        (fontsize - curr.fontsize) / (next.fontsize - curr.fontsize);
      return Math.round(curr.height + ratio * (next.height - curr.height));
    }
  }
  return which[which.length - 1].height;
}

export function fontHeightUnisans(fontsize: number) {
  for (let i = 0; i < FontUniSans.length - 1; i++) {
    const curr = FontUniSans[i];
    const next = FontUniSans[i + 1];
    if (fontsize >= curr.fontsize && fontsize <= next.fontsize) {
      const ratio =
        (fontsize - curr.fontsize) / (next.fontsize - curr.fontsize);
      return Math.round(curr.height + ratio * (next.height - curr.height));
    }
  }
  return FontUniSans[FontUniSans.length - 1].height;
}
