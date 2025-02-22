import type React from "react";
import { Box, Modal } from "@mui/material";
import FilerobotImageEditor, {
	TABS,
	TOOLS,
} from "react-filerobot-image-editor";
import type { savedImageData } from "../../../types";

interface ImageEditorModalProps {
	isOpen: boolean;
	onClose: () => void;
	imageUrl: string;
	onSave: (editedImageObject: savedImageData) => void;
}

const ImageEditorModal: React.FC<ImageEditorModalProps> = ({
	isOpen,
	onClose,
	imageUrl,
	onSave,
}) => {
	return (
		<Modal open={isOpen} onClose={onClose}>
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
				{imageUrl && (
					<FilerobotImageEditor
						source={imageUrl}
						onBeforeSave={() => {
							return false;
						}}
						onSave={onSave}
						onClose={onClose}
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
	);
};

export default ImageEditorModal;
