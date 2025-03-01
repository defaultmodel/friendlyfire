import type React from "react";
import {
	Box,
	Button,
	CircularProgress,
	FormControl,
	InputLabel,
	MenuItem,
	Select,
	Slider,
	Typography,
} from "@mui/material";

interface UploadControlsProps {
	displayTime: number;
	position: string;
	onDisplayTimeChange: (value: number) => void;
	onPositionChange: (value: string) => void;
	onUploadClick: () => void;
	uploading: boolean;
	selectedFile: File | null;
}

const UploadControls: React.FC<UploadControlsProps> = ({
	displayTime,
	position,
	onDisplayTimeChange,
	onPositionChange,
	onUploadClick,
	uploading,
	selectedFile,
}) => {
	return (
		<>
			<Box>
				<Typography gutterBottom>
					Display Time: {displayTime} seconds
				</Typography>
				<Slider
					value={displayTime}
					min={1}
					max={12}
					step={1}
					onChange={(_, value) => onDisplayTimeChange(value as number)}
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
					onChange={(e) => onPositionChange(e.target.value as string)}
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
				onClick={onUploadClick}
				disabled={uploading || !selectedFile}
				fullWidth
			>
				{uploading ? <CircularProgress size={24} /> : "Upload"}
			</Button>
		</>
	);
};

export default UploadControls;
