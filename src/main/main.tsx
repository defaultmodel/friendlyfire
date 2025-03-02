import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ServerProvider } from "./contexts/ServerContext";
import { SocketProvider } from "./contexts/SocketContext";
import { SnackbarProvider } from "./contexts/SnackbarContext";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<SnackbarProvider>
			<ServerProvider>
				<SocketProvider>
					<App />
				</SocketProvider>
			</ServerProvider>
		</SnackbarProvider>
	</React.StrictMode>,
);
