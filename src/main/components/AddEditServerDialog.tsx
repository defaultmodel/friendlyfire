import type React from "react";
import { useState, useEffect } from "react";
import {
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
	TextField,
	Button,
} from "@mui/material";
import type { Server } from "../../types"; // Adjust the import according to your file structure
import { useServerContext } from "../contexts/ServerContext";

interface AddEditServerDialogProps {
	open: boolean;
	onClose: () => void;
	onSave: (server: Server) => void;
	title: string;
	index?: number;
}

const AddEditServerDialog: React.FC<AddEditServerDialogProps> = ({
	open,
	onClose,
	onSave,
	title,
	index,
}) => {
	const { servers } = useServerContext();
	const [newServer, setNewServer] = useState<Server>({
		serverName: "",
		socketUrl: "",
		username: "",
		apiKey: "",
	});

	useEffect(() => {
		if (index !== undefined && servers[index]) {
			setNewServer(servers[index]);
		}
	}, [index, servers]);

	const handleChange =
		(field: keyof Server) => (event: React.ChangeEvent<HTMLInputElement>) => {
			setNewServer((prevServer) => ({
				...prevServer,
				[field]: event.target.value,
			}));
		};

	const handleSave = () => {
		onSave(newServer);
	};

	return (
		<Dialog open={open} onClose={onClose}>
			<DialogTitle>{title}</DialogTitle>
			<DialogContent>
				<TextField
					autoFocus
					margin="dense"
					label="Server Name"
					fullWidth
					value={newServer.serverName}
					onChange={handleChange("serverName")}
				/>
				<TextField
					margin="dense"
					label="Socket URL"
					fullWidth
					value={newServer.socketUrl}
					onChange={handleChange("socketUrl")}
				/>
				<TextField
					margin="dense"
					label="Username"
					fullWidth
					value={newServer.username}
					onChange={handleChange("username")}
				/>
				<TextField
					margin="dense"
					label="API Key"
					fullWidth
					value={newServer.apiKey}
					onChange={handleChange("apiKey")}
				/>
			</DialogContent>
			<DialogActions>
				<Button onClick={onClose}>Cancel</Button>
				<Button variant="contained" color="primary" onClick={handleSave}>
					Save
				</Button>
			</DialogActions>
		</Dialog>
	);
};

export default AddEditServerDialog;
