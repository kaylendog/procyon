module.exports = {
	"*.{css,scss,md,json}": "prettier --write",
	"*.m?{ts,js}x?": ["prettier --write", "eslint --fix"],
	"*.rs": ["cargo fmt --", "cargo clippy --fix --allow-staged"],
};
