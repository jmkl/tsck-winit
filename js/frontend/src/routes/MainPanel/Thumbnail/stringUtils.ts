export const ScaleRange = [17.85, 20.25, 22, 23.8, 26, 27.1];
export const RANGE_UNISANS = [17.85, 20.25, 22, 23.8, 26, 27.1];
export const RANGE_ANTON = [18.8, 21.6, 23.3, 25.2, 27.8, 29.1];
export type TYPEFACE = 'font-unisans' | 'font-anton';
export function scaleRangeByFontName(index: number, typeface: string) {
	return typeface == 'font-unisans' ? RANGE_UNISANS[index] : RANGE_ANTON[index];
}
export function cicleRange(value: number, min: number, max: number): number {
	const range = max - min + 1;
	return ((((value - min) % range) + range) % range) + min;
}
export type TemplateLine = {
	id: number;
	text: string;
	scale: number;
	font_size?: number;
	font_height?: number;
	italic: boolean;
	include: boolean;
};
export function setCaretPos(el: HTMLElement, index: number) {
	el.focus();

	const range = document.createRange();
	const selection = window.getSelection();

	if (!selection) return;

	let charCount = 0;
	let found = false;

	function traverse(node: Node) {
		if (found) return;

		if (node.nodeType === Node.TEXT_NODE) {
			const textNode = node as Text;
			const nextCount = charCount + textNode.length;

			if (index <= nextCount) {
				range.setStart(textNode, index - charCount);
				found = true;
			}

			charCount = nextCount;
		} else {
			node.childNodes.forEach(traverse);
		}
	}

	traverse(el);

	if (!found) {
		// if index exceeds total length, move to end
		range.selectNodeContents(el);
		range.collapse(false);
	}

	selection.removeAllRanges();
	selection.addRange(range);
}
export function breakWord(text: string, maxLen: number = 27): string {
	if (!text) return '';

	let result: string[] = [];
	let start = 0;

	while (start < text.length) {
		// Remaining text shorter than maxLen → push and break
		if (text.length - start <= maxLen) {
			result.push(text.slice(start).trim());
			break;
		}

		// Find last space before maxLen
		const end = start + maxLen;
		let spaceIndex = text.lastIndexOf(' ', end);

		// If no space found in range, hard break
		if (spaceIndex <= start) {
			spaceIndex = end;
		}

		// Extract and trim line
		const line = text.slice(start, spaceIndex).trim();
		result.push(line);

		// Move start to next character after the break
		start = spaceIndex + 1;
	}

	return result.join('\n');
}
/**
 * Extract URLs from text and return both the cleaned text and URLs.
 * @param text The input string.
 * @returns An object with { text, urls }.
 */
export function extractUrls(text: string): { text: string; urls: string[] } {
	const urlRegex = /\b((https?:\/\/|www\.)[^\s<>"]+[^.,;:!?()\[\]{}<>\s])/gi;

	const urls: string[] = [];
	const cleanedText = text.replace(urlRegex, (match) => {
		let normalized = match;
		if (!/^https?:\/\//i.test(match)) {
			normalized = 'https://' + match;
		}
		urls.push(normalized);
		return ''; // remove the URL from the text
	});

	// Normalize spacing after URL removal
	const finalText = cleanedText.replace(/\s{2,}/g, ' ').trim();

	return { text: finalText, urls };
}
