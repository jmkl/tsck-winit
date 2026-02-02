function formatDate(date: Date): string {
	return date.toLocaleString("en-US", {
		day: "2-digit",
		month: "short",
		year: "numeric",
		hour: "2-digit",
		minute: "2-digit",
		second: "2-digit",
	});
}
setInterval(() => {
	console.log("Timer::", formatDate(new Date()));
}, 1000);
