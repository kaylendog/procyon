// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer/themes/github");
const darkCodeTheme = require("prism-react-renderer/themes/dracula");

/** @type {import('@docusaurus/types').Config} */
const config = {
	title: "Procyon",
	tagline: "Pluggable companion app for Elite Dangerous.",
	url: "https://your-docusaurus-test-site.com",
	baseUrl: "/",
	onBrokenLinks: "throw",
	onBrokenMarkdownLinks: "warn",
	favicon: "img/favicon.ico",

	organizationName: "kaylendog",
	projectName: "procyon",

	i18n: {
		defaultLocale: "en",
		locales: ["en"],
	},

	presets: [
		[
			"classic",
			/** @type {import('@docusaurus/preset-classic').Options} */
			({
				docs: {
					sidebarPath: require.resolve("./sidebars.js"),
				},
				blog: {
					path: "./releases",
					blogTitle: "Releases",
					routeBasePath: "/releases",
				},
				theme: {
					customCss: require.resolve("./src/css/custom.css"),
				},
			}),
		],
	],

	themeConfig:
		/** @type {import('@docusaurus/preset-classic').ThemeConfig} */
		({
			navbar: {
				title: "Procyon",
				logo: {
					alt: "My Site Logo",
					src: "img/logo.svg",
				},
				items: [
					{
						to: "/",
						label: "Home",
						position: "left",
					},
					{
						to: "/releases",
						label: "Releases",
						position: "left",
					},
					{
						type: "doc",
						docId: "intro",
						position: "right",
						label: "Guides",
					},
					{
						type: "doc",
						docId: "developers/introduction",
						position: "right",
						label: "Developers",
					},
					{
						href: "https://github.com/kaylendog/procyon",
						label: "GitHub",
						position: "right",
					},
				],
			},
			footer: {
				style: "dark",
				links: [
					{
						title: "Docs",
						items: [
							{
								label: "Installation",
								to: "/docs/intro",
							},
							{
								label: "Developers",
								to: "/docs/developers/",
							},
						],
					},
					{
						title: "Community",
						items: [
							{
								label: "Discord",
								href: "https://discordapp.com/invite/docusaurus",
							},
						],
					},
					{
						title: "More",
						items: [
							{
								label: "GitHub",
								href: "https://github.com/kaylendog/procyon",
							},
						],
					},
				],
				copyright: `Copyright © ${new Date().getFullYear()} Kaylen Dart`,
			},
			prism: {
				theme: lightCodeTheme,
				darkTheme: darkCodeTheme,
			},
		}),
};

module.exports = config;
