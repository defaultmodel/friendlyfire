import type React from "react";
import { useEffect, useState } from "react";
import { useSocket } from "../SocketContext";
import { emit } from "@tauri-apps/api/event";
import {
	Box,
	Button,
	Typography,
	Slider,
	Select,
	MenuItem,
	InputLabel,
	FormControl,
	CircularProgress,
	Paper,
	Container,
	IconButton,
	Modal,
} from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import FilerobotImageEditor, {
	TABS,
	TOOLS,
} from "react-filerobot-image-editor";
import { listen } from "@tauri-apps/api/event";
import { readFile } from "@tauri-apps/plugin-fs";
import { readImage } from "@tauri-apps/plugin-clipboard-manager";
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";

const ImageUploader: React.FC = () => {
	const { socket, socketUrl } = useSocket();
	const [selectedFile, setSelectedFile] = useState<File | null>(null);
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [uploading, setUploading] = useState<boolean>(false);
	const [displayTime, setDisplayTime] = useState<number>(6);
	const [position, setPosition] = useState<string>("center");
	const [isEditorOpen, setIsEditorOpen] = useState(false);

	interface FileDropEventPayload {
		paths: string[];
	}

	useEffect(() => {
		trace("Setting up drag-and-drop listener");
		const unlisten = listen<FileDropEventPayload>(
			"tauri://drag-drop",
			async (event) => {
				if (event.payload.paths as []) {
					const filePath = event.payload.paths[0] as string;
					debug(`File dropped: ${filePath}`);
					try {
						const fileContent = await readFile(filePath);
						const fileName = filePath.split("/").pop() || ;
						const file = new File([fileContent], fileName, {
							type: "image/*",
						});
						setSelectedFile(file);
						setPreviewUrl(URL.createObjectURL(file));
						info(`File selected: ${file.name}`);
					} catch (err: unknown) {
						if (typeof err === "string") {
							error(`Error occured while reading chosen file: ${err}`);
						} else if (err instanceof Error) {
							error(`Error occured while reading chosen file: ${err.message}`);
						}
					}
				}
			},
		);

		return () => {
			unlisten.then((f) => f());
			trace("Drag-and-drop listener cleaned up");
		};
	}, []);

	useEffect(() => {
		trace("Setting up paste event listener");
		addEventListener("paste", async () => {
			const clipboardText = await readImage();
			const blob = new Blob([await clipboardText.rgba()]);
			const objectUrl = URL.createObjectURL(blob);
			setPreviewUrl(objectUrl);
			info("Image pasted from clipboard");

			return () => URL.revokeObjectURL(objectUrl);
		});
	}, []);

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

	const base64ToFile = (base64: string, filename: string): File => {
		const arr = base64.split(",");
		const mime = arr[0].match(/:(.*?);/)?.[1];
		const bstr = atob(arr[1]);
		let n = bstr.length;
		const u8arr = new Uint8Array(n);
		while (n--) {
			u8arr[n] = bstr.charCodeAt(n);
		}
		return new File([u8arr], filename, { type: mime });
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
			await fetch("http://localhost:3000/upload", {
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
					{selectedFile && (
						<>
							<Typography variant="body1">
								Selected File: {selectedFile.name}
							</Typography>
							{previewUrl && (
								<Box
									sx={{
										position: "relative",
										display: "flex",
										justifyContent: "center",
										marginTop: 2,
										"&:hover .edit-button": {
											display: "block",
										},
									}}
								>
									<img
										src={previewUrl}
										alt="Preview"
										style={{
											maxWidth: "100%",
											maxHeight: "300px",
											borderRadius: "8px",
										}}
									/>
									<IconButton
										className="edit-button"
										sx={{
											display: "none",
											position: "absolute",
											top: "50%",
											left: "50%",
											transform: "translate(-50%, -50%)",
											backgroundColor: "rgba(255, 255, 255, 0.8)",
											"&:hover": {
												backgroundColor: "rgba(255, 255, 255, 1)",
											},
										}}
										onClick={handleEditClick}
									>
										<EditIcon />
									</IconButton>
								</Box>
							)}
							<Modal open={isEditorOpen} onClose={handleCloseEditor}>
								<Box
									sx={{
										position: "absolute",
										top: "50%",
										left: "50%",
										transform: "translate(-50%, -50%)",
										width: "60%",
										height: "70%",
										bgcolor: "background.paper",
										boxShadow: 24,
										p: 4,
										borderRadius: "8px",
									}}
								>
									{previewUrl && (
										<FilerobotImageEditor
											source={previewUrl}
											onBeforeSave={() => {
												return false;
											}}
											onSave={(editedImageObject) => {
												setPreviewUrl(editedImageObject!.imageBase64);
												handleCloseEditor();
												info("Image edited and saved");
											}}
											onClose={handleCloseEditor}
											previewPixelRatio={4}
											savingPixelRatio={4}
											annotationsCommon={{
												fill: "#ff0000",
											}}
											tabsIds={[TABS.ADJUST, TABS.ANNOTATE, TABS.RESIZE]}
											defaultTabId={TABS.ANNOTATE}
											defaultToolId={TOOLS.TEXT}
										/>
									)}
								</Box>
							</Modal>
						</>
					)}
					<Box>
						<Typography gutterBottom>
							Display Time: {displayTime} seconds
						</Typography>
						<Slider
							value={displayTime}
							min={1}
							max={12}
							step={1}
							onChange={(_, value) => setDisplayTime(value as number)}
							valueLabelDisplay="auto"
						/>
					</Box>
					<FormControl fullWidth>
						<InputLabel id="position-label">Position</InputLabel>
						<Select
							labelId="position-label"
							id="position"
							value={position}
							label="Position"
							onChange={(e) => setPosition(e.target.value as string)}
						>
							<MenuItem value="top-left">Top Left</MenuItem>
							<MenuItem value="top-right">Top Right</MenuItem>
							<MenuItem value="bottom-left">Bottom Left</MenuItem>
							<MenuItem value="bottom-right">Bottom Right</MenuItem>
							<MenuItem value="center">Center</MenuItem>
						</Select>
					</FormControl>
					<Button
						variant="contained"
						color="primary"
						onClick={handleUpload}
						disabled={uploading || !selectedFile}
						fullWidth
					>
						{uploading ? <CircularProgress size={24} /> : "Upload"}
					</Button>
				</Box>
			</Paper>
		</Container>
	);
};

export default ImageUploader;
