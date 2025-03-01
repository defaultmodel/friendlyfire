import type React from "react";
import { Box, IconButton } from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";

interface ImagePreviewProps {
	imageUrl: string;
	onEditClick: () => void;
}

const ImagePreview: React.FC<ImagePreviewProps> = ({
	imageUrl,
	onEditClick,
}) => {
	return (
		<>
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
					src={imageUrl}
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
					onClick={onEditClick}
				>
					<EditIcon />
				</IconButton>
			</Box>
		</>
	);
};

export default ImagePreview;
