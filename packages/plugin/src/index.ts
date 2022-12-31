/**
 * Type alias for dependencies.
 */
type Dependency =
	| string
	| {
			name: string;
			version: string;
	  };

/**
 * The plugin manifest type.
 */
export interface Manifest {
	name: string;
	version: string;
	dependencies: Dependency[];
}

type RequiredManifestKeys = "name" | "version";
type PartialManifest = Pick<Manifest, RequiredManifestKeys> & Partial<Manifest>;

/**
 * A generic logging interface.
 */
interface Logger {
	debug: (...args: unknown[]) => void;
	info: (...args: unknown[]) => void;
	warn: (...args: unknown[]) => void;
	error: (...args: unknown[]) => void;
}

/**
 * A generic plugin.
 */
export abstract class Plugin {
	constructor(readonly logger: Logger, readonly manifest: Manifest) {}

	/**
	 * Called when the plugin is initialized.
	 */
	onInitialize() {
		// noop
	}

	/**
	 * Called when Procyon is shutting down.
	 */
	onShutdown() {
		// noop
	}
}

/**
 * Type helper to make it easier to use TypeScript with Procyon.
 */
export function definePlugin(manifest: PartialManifest, constructor: typeof Plugin) {
	return { manifest, constructor };
}
