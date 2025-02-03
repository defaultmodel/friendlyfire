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
		const unlisten = listen<FileDropEventPayload>(
			"tauri://drag-drop",
			async (event) => {
				if (event.payload.paths as []) {
					const filePath = event.payload.paths[0] as string;
					try {
						const fileContent = await readFile(filePath);
						const file = new File([fileContent], filePath.split("/").pop()!, {
							type: "image/*",
						});
						setSelectedFile(file);
						setPreviewUrl(URL.createObjectURL(file));
					} catch (error) {
						console.error("Error reading file:", error);
					}
				}
			},
		);

		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	useEffect(() => {
		if (!socket) return;

		// Listen for new image events
		socket.on("new image", (url: string) => {
			// Emit a Tauri event to the "slave" window
			const fullUrl = `${socketUrl}${url}`;
			emit("new-image", { url: fullUrl, displayTime, position })
				.then(() => {
					console.log("Event emitted to slave window successfully");
				})
				.catch((err) => {
					console.error("Failed to emit event to slave window:", err);
				});
		});

		return () => {
			socket.off("new image");
		};
	}, [socket, socketUrl, displayTime, position]);

	// When selectedFile changes we update the preview
	useEffect(() => {
		if (!selectedFile) {
			setPreviewUrl(null);
			return;
		}

		const objectUrl = URL.createObjectURL(selectedFile);
		setPreviewUrl(objectUrl);

		// Clean up the object URL when the component unmounts or the file changes
		return () => URL.revokeObjectURL(objectUrl);
	}, [selectedFile]);

	const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (file) {
			setSelectedFile(file);
			setPreviewUrl(URL.createObjectURL(file)); // Create a preview URL
		}
	};

	const handleEditClick = () => {
		setIsEditorOpen(true);
	};

	const handleCloseEditor = () => {
		setIsEditorOpen(false);
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
			alert("Please select a file first!");
			return;
		}

		setUploading(true);

		let fileToUpload: File;
		if (previewUrl.startsWith("data:image")) {
			// If the image is edited, convert the base64 URL to a File object
			fileToUpload = base64ToFile(
				previewUrl,
				selectedFile?.name || "edited-image.png",
			);
		} else {
			// If the image is not edited, use the original file
			fileToUpload = selectedFile!;
		}

		const formData = new FormData();
		formData.append("image", fileToUpload);

		try {
			await fetch("http://localhost:3000/upload", {
				method: "POST",
				body: formData,
			});
		} catch (error) {
			console.error("Error uploading image:", error);
		} finally {
			setUploading(false);
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
