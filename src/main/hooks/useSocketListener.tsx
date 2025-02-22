import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { debug, info, error, trace } from "@tauri-apps/plugin-log";
import type { Socket } from "socket.io-client";

export const useSocketListener = (
	socket: Socket | null,
	socketUrl: string | null,
	displayTime: number,
	position: string,
) => {
	useEffect(() => {
		if (!socket) return;

		trace("Setting up socket listener for new image events");
		socket.on("new image", (url: string) => {
			const fullUrl = `${socketUrl}${url}`;
			debug(`New image event received: ${fullUrl}`);
			emit("new-image", { url: fullUrl, displayTime, position })
				.then(() => {
					info("Event emitted to slave window successfully");
				})
				.catch((err) => {
					error("Failed to emit event to slave window:", err);
				});
		});

		return () => {
			socket.off("new image");
			trace("Socket listener for new image events cleaned up");
		};
	}, [socket, socketUrl, displayTime, position]);
};
