import type React from "react";
import { useEffect, useState } from "react";
import { Box, Button, Typography, Paper, Container } from "@mui/material";
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
import UploadControls from "../components/ImageUploadControls.tsx";
import ImagePreview from "../components/ImagePreview";
import ImageEditorModal from "../components/ImageEditorModal";
import { useDragAndDrop } from "../hooks/useDragDropImage.tsx";
import { usePaste } from "../hooks/usePasteImage";
import { useSocketListener } from "../hooks/useSocketListener";
import { useSocket } from "../contexts/SocketContext";
import Menu from "../components/Menu.tsx";
import { useSnackbar } from "../contexts/SnackbarContext.tsx";
import {
	getUploadUrl,
	prepareFileForUpload,
	uploadImage,
} from "../../utils/uploadUtils.ts";
import UserList from "../components/UserList.tsx";

const ImageUploaderPage: React.FC = () => {
	const { socket, socketUrl } = useSocket();
	const { showSnackbar } = useSnackbar();
	const [selectedFile, setSelectedFile] = useState<File | null>(null);
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [uploading, setUploading] = useState<boolean>(false);
	const [displayTime, setDisplayTime] = useState<number>(6);
	const [position, setPosition] = useState<string>("center");
	const [isEditorOpen, setIsEditorOpen] = useState(false);

	useDragAndDrop(setSelectedFile, setPreviewUrl);
	usePaste(setPreviewUrl);
	useSocketListener(socket, socketUrl);

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
		if (!previewUrl || !selectedFile) {
			warn("No file selected for upload");
			showSnackbar("Please select a file first!", "warning");
			return;
		}

		if (!socketUrl) {
			warn("No socketUrl, unable to upload");
			return;
		}

		setUploading(true);
		info("Starting image upload");

		const fileToUpload = prepareFileForUpload(previewUrl, selectedFile);

		const formData = new FormData();
		formData.append("image", fileToUpload);
		formData.append("displayTime", displayTime.toString(10));
		formData.append("position", position);
		formData.append("username", socket!.auth!.username);

		const uploadUrl = getUploadUrl(socketUrl);

		try {
			await uploadImage(uploadUrl, formData);
		} catch (err: unknown) {
			const errorMessage = err instanceof Error ? err.message : String(err);
			error(`Error uploading image: ${errorMessage}`);
			showSnackbar(`Error uploading image: ${errorMessage}`, "error");
		} finally {
			setUploading(false);
			trace("Upload process completed");
		}
	};

	return (
		<>
			<Menu />
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
		</>
	);
};

export default ImageUploaderPage;
