import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { readFile } from "@tauri-apps/plugin-fs";
import { debug, info, error, trace } from "@tauri-apps/plugin-log";
import type { FileDropEventPayload } from "../../types";
import { useSnackbar } from "../contexts/SnackbarContext";

export const useDragAndDrop = (
	setSelectedFile: (file: File) => void,
	setPreviewUrl: (url: string) => void,
) => {
	useEffect(() => {
		const unlisten = listen<FileDropEventPayload>(
			"tauri://drag-drop",
			async (event) => {
				const { showSnackbar } = useSnackbar();

				if (event.payload.paths as []) {
					const filePath = event.payload.paths[0] as string;
					debug(`File dropped: ${filePath}`);
					try {
						const fileContent = await readFile(filePath);
						const fileName = filePath.split("/").pop() || "default";
						const file = new File([fileContent], fileName, { type: "image/*" });
						setSelectedFile(file);
						setPreviewUrl(URL.createObjectURL(file));
						info(`File selected: ${file.name}`);
					} catch (err: unknown) {
						const errorMessage = err instanceof Error ? err.message : err;
						error(`Error uploading image: ${errorMessage}`);
						showSnackbar(`Error uploading image: ${errorMessage}`, "error");
					}
				}
			},
		);

		return () => {
			unlisten.then((f) => {
				trace("Drag-and-drop listener cleaned up");
				return f();
			});
		};
	}, [setSelectedFile, setPreviewUrl]);
};
