export function randomSeed(): number {
	const array = new Uint32Array(1);
	crypto.getRandomValues(array);
	return array[0] % (Number.MAX_SAFE_INTEGER + 1);
}
