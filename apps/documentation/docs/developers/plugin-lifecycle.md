# Plugin Lifecycle

A plugin will go through three stages during its time loaded

- Initialization
- Active
- Shutdown

## Initialization

In this stage, the plugin loads its resources and performs the ncessary work to initialize. This stage occurs when Procyon is first loaded, or when the plugin is enabled.

## Active

In this stage, the plugin actively listens for events and UI changes, and performs the necessary logic to give itself functionality.

## Shutdown

In this stage, the plugin tidies up its resources and ceases functionality. This occurs when Procyon is shutting down, or when the plugin is manually disabled by the user.
