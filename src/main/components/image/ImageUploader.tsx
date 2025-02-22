import type React from "react";
import { useEffect, useState } from "react";
import { useSocket } from "../../SocketContext";
import { Box, Button, Typography, Paper, Container } from "@mui/material";
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
import { base64ToFile } from "../../../utils/fileUtils";
import UploadControls from "./UploadControls";
import ImagePreview from "./ImagePreview";
import ImageEditorModal from "./ImageEditorModal";
import { useDragAndDrop } from "../../hooks/useDragandDropImage";
import { usePaste } from "../../hooks/usePasteImage";
import { useSocketListener } from "../../hooks/useSocketListener";

const ImageUploader: React.FC = () => {
	const { socket, socketUrl } = useSocket();
	const [selectedFile, setSelectedFile] = useState<File | null>(null);
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [uploading, setUploading] = useState<boolean>(false);
	const [displayTime, setDisplayTime] = useState<number>(6);
	const [position, setPosition] = useState<string>("center");
	const [isEditorOpen, setIsEditorOpen] = useState(false);

	useDragAndDrop(setSelectedFile, setPreviewUrl);
	usePaste(setPreviewUrl);
	useSocketListener(socket, socketUrl, displayTime, position);

	useEffect(() => {
		if (!selectedFile) {
			setPreviewUrl(null);
			trace("Selected file cleared, preview URL reset");
			return;
		}

		const objectUrl = URL.createObjectURL(selectedFile);
		setPreviewUrl(objectUrl);
		debug(`Preview URL generated for file: ${selectedFile.name}`);

		return () => URL.revokeObjectURL(objectUrl);
	}, [selectedFile]);

	const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (file) {
			setSelectedFile(file);
			setPreviewUrl(URL.createObjectURL(file));
			info(`File selected via input: ${file.name}`);
		}
	};

	const handleEditClick = () => {
		setIsEditorOpen(true);
		debug("Image editor opened");
	};

	const handleCloseEditor = () => {
		setIsEditorOpen(false);
		debug("Image editor closed");
	};

	const handleUpload = async () => {
		if (!previewUrl) {
			warn("No file selected for upload");
			alert("Please select a file first!");
			return;
		}

		setUploading(true);
		info("Starting image upload");

		let fileToUpload: File;
		if (previewUrl.startsWith("data:image")) {
			fileToUpload = base64ToFile(
				previewUrl,
				selectedFile?.name || "edited-image.png",
			);
			debug("Edited image converted to file for upload");
		} else {
			fileToUpload = selectedFile!;
		}

		const formData = new FormData();
		formData.append("image", fileToUpload);

		try {
			await fetch(`${socketUrl}/upload`, {
				method: "POST",
				body: formData,
			});
			info("Image uploaded successfully");
		} catch (err: unknown) {
			if (typeof err === "string") {
				error(`Error uploading image: ${err}`);
			} else if (err instanceof Error) {
				error(`Error uploading image: ${err.message}`);
			}
		} finally {
			setUploading(false);
			trace("Upload process completed");
		}
	};

	return (
		<Container maxWidth="sm">
			<Paper elevation={3} sx={{ padding: 4, marginTop: 4 }}>
				<Typography variant="h4" component="h1" align="center" gutterBottom>
					Upload Image
				</Typography>
				<Box
					component="form"
					sx={{ display: "flex", flexDirection: "column", gap: 3 }}
				>
					<Button variant="contained" component="label">
						Upload File
						<input
							type="file"
							accept="image/*"
							hidden
							onChange={handleFileChange}
						/>
					</Button>
					{selectedFile && previewUrl && (
						<>
							<ImagePreview
								imageUrl={previewUrl}
								fileName={selectedFile.name}
								onEditClick={handleEditClick}
							/>
							<ImageEditorModal
								isOpen={isEditorOpen}
								onClose={handleCloseEditor}
								imageUrl={previewUrl}
								onSave={(editedImageObject) => {
									if (editedImageObject.imageBase64) {
										setPreviewUrl(editedImageObject.imageBase64);
										handleCloseEditor();
										info("Image edited and saved");
									}
								}}
							/>
						</>
					)}
					<UploadControls
						displayTime={displayTime}
						position={position}
						onDisplayTimeChange={setDisplayTime}
						onPositionChange={setPosition}
						onUploadClick={handleUpload}
						uploading={uploading}
						selectedFile={selectedFile}
					/>
				</Box>
			</Paper>
		</Container>
	);
};

export default ImageUploader;
