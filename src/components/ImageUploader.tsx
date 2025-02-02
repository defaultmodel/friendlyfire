// ImageUploader.tsx
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
} from "@mui/material";

const ImageUploader: React.FC = () => {
	const [selectedFile, setSelectedFile] = useState<File | null>(null);
	const [previewUrl, setPreviewUrl] = useState<string | null>(null);
	const [uploading, setUploading] = useState<boolean>(false);
	const [displayTime, setDisplayTime] = useState<number>(6);
	const [position, setPosition] = useState<string>("center");
	const { socket, socketUrl } = useSocket();

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
		if (event.target.files?.[0]) {
			setSelectedFile(event.target.files[0]);
		}
	};

	const handleUpload = async () => {
		if (!selectedFile) {
			alert("Please select a file first!");
			return;
		}

		setUploading(true);

		const formData = new FormData();
		formData.append("image", selectedFile);

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
								<img
									src={previewUrl}
									alt="Preview"
									style={{
										maxWidth: "100%",
										maxHeight: "200px",
										margin: "auto",
									}}
								/>
							)}
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
