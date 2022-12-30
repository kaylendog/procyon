module.exports = {
	"*.{css,scss,md,json}": "prettier --write",
	"*.m?{ts,js}x?": ["prettier --write", "eslint --fix"],
	"*.rs": (files) => [
		`cargo fmt -- ${files.join(" ")}`,
		"cargo clippy --fix --allow-dirty --allow-staged --",
	],
};
