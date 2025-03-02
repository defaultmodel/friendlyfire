import type React from "react";
import { createContext, useContext, useState, type ReactNode } from "react";
import { Snackbar, Alert, type AlertColor } from "@mui/material";

type SnackbarContextType = {
	showSnackbar: (message: string, severity?: AlertColor) => void;
};

const SnackbarContext = createContext<SnackbarContextType | undefined>(
	undefined,
);

export const useSnackbar = () => {
	const context = useContext(SnackbarContext);
	if (!context) {
		throw new Error("useSnackbar must be used within a SnackbarProvider");
	}
	return context;
};

type SnackbarProviderProps = {
	children: ReactNode;
};

export const SnackbarProvider: React.FC<SnackbarProviderProps> = ({
	children,
}) => {
	const [open, setOpen] = useState(false);
	const [message, setMessage] = useState("");
	const [severity, setSeverity] = useState<AlertColor>("success");

	const showSnackbar = (message: string, severity: AlertColor = "success") => {
		setMessage(message);
		setSeverity(severity);
		setOpen(true);
	};

	const handleClose = () => {
		setOpen(false);
	};

	return (
		<SnackbarContext.Provider value={{ showSnackbar }}>
			{children}
			<Snackbar
				open={open}
				autoHideDuration={6000}
				onClose={handleClose}
				anchorOrigin={{ vertical: "top", horizontal: "center" }}
			>
				<Alert onClose={handleClose} severity={severity} sx={{ width: "100%" }}>
					{message}
				</Alert>
			</Snackbar>
		</SnackbarContext.Provider>
	);
};
