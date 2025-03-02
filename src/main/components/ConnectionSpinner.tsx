import type React from "react";
import { Backdrop, CircularProgress, Typography, Box } from "@mui/material";

interface ConnectionSpinnerProps {
	open: boolean;
}

const ConnectionSpinner: React.FC<ConnectionSpinnerProps> = ({ open }) => {
	return (
		<Backdrop
			sx={{ color: "#fff", zIndex: (theme) => theme.zIndex.drawer + 1 }}
			open={open}
		>
			<Box
				sx={{
					display: "flex",
					flexDirection: "column",
					alignItems: "center",
				}}
			>
				<CircularProgress color="inherit" />
				<Typography variant="h6" sx={{ mt: 2 }}>
					Connecting to server...
				</Typography>
			</Box>
		</Backdrop>
	);
};

export default ConnectionSpinner;
