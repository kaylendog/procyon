import Axios from "axios";

import { definePlugin, Plugin } from "@procyon/plugin";

class EDDNPlugin extends Plugin {
	readonly http = Axios.create({
		baseURL: "https://eddn.edcd.io:4430/upload",
	});

	onInitialize(): void {
		this.logger.info("EDDN data service initialized");
	}
}

export default definePlugin(
	{
		name: "eddn",
		version: "1.0.0",
	},
	EDDNPlugin
);
