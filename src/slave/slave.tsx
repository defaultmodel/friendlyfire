import ReactDOM from "react-dom/client";
import React, { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { Avatar } from "@mui/material";

interface Media {
	url: string;
	displayTime: number;
	username: string;
}

const SlaveApp: React.FC = () => {
	const [imageUrl, setImageUrl] = useState<string | null>(null);
	const [timer, setTimer] = useState<number | null>(null);
	const [username, setUsername] = useState<string | null>(null);

	useEffect(() => {
		// Listen for the "new-image-event" event
		const unlisten = listen("new-image", async (event) => {
			const { url, displayTime, username } = event.payload as Media;
			console.log(username);
			const window = getCurrentWindow();
			window.show();
			setImageUrl(url); // Update the image URL state
			setUsername(username);

			// Resize the window to match the image's aspect ratio
			const img = new Image();
			img.src = url;

			img.onload = async () => {
				const maxSize = 800; // Maximum size for width or height
				const aspectRatio = img.width / img.height;

				let newWidth: number;
				let newHeight: number;

				if (img.width > img.height) {
					// Landscape image
					newWidth = Math.min(img.width, maxSize);
					newHeight = newWidth / aspectRatio;
				} else {
					// Portrait or square image
					newHeight = Math.min(img.height, maxSize);
					newWidth = newHeight * aspectRatio;
				}

				// Get the current window and set its size
				await window.setSize(new LogicalSize(newWidth, newHeight));
			};

			// Clear the existing timer if any
			if (timer) {
				clearTimeout(timer);
			}

			// Set a new timer to hide the image after the display time
			const newTimer = setTimeout(() => {
				setImageUrl(null);
				window.hide();
			}, displayTime * 1000);

			setTimer(newTimer); // Store the new timer
		});

		// Cleanup the listener on unmount
		return () => {
			unlisten.then((fn) => fn());
			if (timer) {
				clearTimeout(timer); // Clear the timer when the component unmounts
			}
		};
	}, [timer]);

	console.log(username);

	return (
		<div style={{ position: "relative" }}>
			{username && (
				<div style={{ position: "absolute", top: 10, left: 10 }}>
					<Avatar>{username[0].toUpperCase()}</Avatar>
				</div>
			)}
			{imageUrl ? (
				<img
					src={imageUrl}
					alt="Uploaded"
					style={{ maxWidth: "100%", maxHeight: "100%" }}
				/>
			) : (
				<p>No image received yet.</p>
			)}
		</div>
	);
};

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
	<React.StrictMode>
		<SlaveApp />
	</React.StrictMode>,
);
