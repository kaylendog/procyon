import clsx from "clsx";
import React from "react";

import Link from "@docusaurus/Link";
import useDocusaurusContext from "@docusaurus/useDocusaurusContext";
import HomepageFeatures from "@site/src/components/HomepageFeatures";
import Layout from "@theme/Layout";

import banner from "../../static/img/banner-light.png";
import styles from "./index.module.css";

function HomepageHeader() {
	return (
		<header className={clsx("hero hero--primary", styles.heroBanner)}>
			<div className="container">
				<img src={banner} width="800"></img>
				<p className="hero__subtitle">Pluggable companion app for Elite Dangerous.</p>
				<div className={styles.buttons}>
					<Link className="button button--secondary button--lg" to="/releases">
						Download
					</Link>
					<Link className="button button--primary button--lg" to="/docs/intro">
						Guides
					</Link>
				</div>
			</div>
		</header>
	);
}

export default function Home(): JSX.Element {
	return (
		<Layout title="Home" description="Pluggable companion app for Elite Dangerous.">
			<HomepageHeader />
			<main>
				<HomepageFeatures />
			</main>
		</Layout>
	);
}
