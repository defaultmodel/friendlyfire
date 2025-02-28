import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ServerProvider } from "./contexts/ServerContext";
import { SocketProvider } from "./contexts/SocketContext";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<ServerProvider>
			<SocketProvider>
				<App />
			</SocketProvider>
		</ServerProvider>
	</React.StrictMode>,
);
