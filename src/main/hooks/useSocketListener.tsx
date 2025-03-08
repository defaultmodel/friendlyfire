import { useEffect } from "react";
import { emit } from "@tauri-apps/api/event";
import { debug, info, error, trace } from "@tauri-apps/plugin-log";
import type { Socket } from "socket.io-client";
import { getUploadUrl } from "../../utils/uploadUtils";

export const useSocketListener = (
	socket: Socket | null,
	socketUrl: string | null,
) => {
	useEffect(() => {
		if (!socket || !socketUrl) return;

		// Ensure the socket URL has a valid scheme for HTTP requests
		const uploadUrl = getUploadUrl(socketUrl);

		trace("Setting up socket listener for new image events");
		socket.on(
			"new image",
			(
				url: string,
				displayTime: number,
				position: string,
				username: string,
			) => {
				debug(
					`New image event received. url: ${url} displayTime: ${displayTime} position: ${position} username: ${username}`,
				);
				emit("new-image", {
					url: `${uploadUrl}${url}`,
					displayTime,
					position,
					username,
				})
					.then(() => {
						info("Event emitted to slave window successfully");
					})
					.catch((err) => {
						error("Failed to emit event to slave window:", err);
					});
			},
		);

		return () => {
			socket.off("new image");
			trace("Socket listener for new image events cleaned up");
		};
	}, [socket, socketUrl]);
};
