import { useEffect } from "react";
import { readImage } from "@tauri-apps/plugin-clipboard-manager";
import { warn, info, error, trace } from "@tauri-apps/plugin-log";

export const usePaste = (setPreviewUrl: (url: string) => void) => {
	useEffect(() => {
		const pasteHandler = async () => {
			try {
				const clipboardImage = await readImage();
				if (!clipboardImage) {
					warn("Clipboard does not contain an image.");
					return;
				}

				const blob = new Blob([await clipboardImage.rgba()]);
				const objectUrl = URL.createObjectURL(blob);
				setPreviewUrl(objectUrl);
				info("Image pasted from clipboard");

				// Cleanup function for the object URL
				return () => URL.revokeObjectURL(objectUrl);
				// biome-ignore lint/suspicious/noExplicitAny: <explanation>
			} catch (err: any) {
				error("Failed to process pasted image:", err);
			}
		};

		window.addEventListener("paste", pasteHandler);

		return () => {
			window.removeEventListener("paste", pasteHandler);
			trace("Paste event listener cleaned up");
		};
	}, [setPreviewUrl]);
};
