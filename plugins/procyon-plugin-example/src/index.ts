import { definePlugin, Plugin } from "@procyon/plugin";

class ExamplePlugin extends Plugin {
	onInitialize(): void {
		this.logger.info("Example plugin initialized!");
	}
	onShutdown(): void {
		this.logger.info("Example plugin shutdown!");
	}
}

export default definePlugin(
	{
		name: "example",
		version: "1.0.0",
	},
	ExamplePlugin
);
