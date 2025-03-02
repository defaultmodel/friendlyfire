// utils/uploadUtils.ts
import { base64ToFile } from "./fileUtils";
import { info, debug } from "@tauri-apps/plugin-log";

export const getUploadUrl = (socketUrl: string) => {
	let uploadUrl = socketUrl;

	// Ensure the socket URL has a valid scheme for HTTP requests
	if (!/^https?:\/\//i.test(uploadUrl)) {
		uploadUrl = `https://${uploadUrl}`;
	}

	return uploadUrl;
};

export const uploadImage = async (uploadUrl: string, formData: FormData) => {
	try {
		const response = await fetch(`${uploadUrl}/upload`, {
			method: "POST",
			body: formData,
		});

		if (!response.ok) {
			throw new Error(`Upload failed with status: ${response.status}`);
		}

		info("Image uploaded successfully using HTTPS");
	} catch (err: unknown) {
		if (uploadUrl.startsWith("https://")) {
			// Fallback to http if https fails
			const httpUrl = uploadUrl.replace("https://", "http://");
			const fallbackResponse = await fetch(`${httpUrl}/upload`, {
				method: "POST",
				body: formData,
			});

			if (!fallbackResponse.ok) {
				throw new Error(
					`Upload failed with status: ${fallbackResponse.status}`,
				);
			}

			info("Image uploaded successfully using HTTP fallback");
		} else {
			throw new Error(`Error uploading image: ${(err as Error).message}`);
		}
	}
};

export const prepareFileForUpload = (
	previewUrl: string,
	selectedFile: File,
) => {
	let fileToUpload: File;
	if (previewUrl.startsWith("data:image")) {
		fileToUpload = base64ToFile(
			previewUrl,
			selectedFile?.name || "edited-image.png",
		);
		debug("Edited image converted to file for upload");
	} else {
		fileToUpload = selectedFile;
	}
	return fileToUpload;
};
