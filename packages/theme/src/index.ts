import { extendTheme } from "@chakra-ui/react";

import borders from "./borders";
import * as components from "./components";
import styles from "./styles";

export const theme = extendTheme({
	borders,
	components,
	styles,
});
