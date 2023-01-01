import esbuild from "esbuild";

esbuild.build({
	entryPoints: ["./src/index.ts"],
	bundle: true,
	external: ["@procyon/plugin"],
	format: "cjs",
	target: "es2018",
	outfile: "./dist/main.js",
	sourcemap: false,
	treeShaking: true,
});
