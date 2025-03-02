import { useState } from "react";
import {
	Container,
	Typography,
	Card,
	CardActionArea,
	CardContent,
	Button,
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
} from "@mui/material";
import Grid from "@mui/material/Grid2";
import AddIcon from "@mui/icons-material/Add";
import { useServerContext } from "../contexts/ServerContext";
import type { Server } from "../../types";
import ServerCard from "../components/ServerCard";
import AddEditServerDialog from "../components/AddEditServerDialog";
import { useSocket } from "../contexts/SocketContext";
import ConnectionSpinner from "../components/ConnectionSpinner";

const ServerSelectionPage = () => {
	const { servers, addServer, removeServer, updateServer } = useServerContext();
	const { connectSocket } = useSocket();
	const [selectedServerIndex, setSelectedServerIndex] = useState<
		number | undefined
	>(undefined);
	const [openConfirmation, setOpenConfirmation] = useState(false);
	const [openAddServer, setOpenAddServer] = useState(false);
	const [openEditServer, setOpenEditServer] = useState(false);
	const [connecting, setConnecting] = useState(false);

	const handleConfirmSelection = async () => {
		if (selectedServerIndex !== undefined && servers[selectedServerIndex]) {
			console.log(
				`Connecting to server: ${servers[selectedServerIndex].serverName}`,
			);
			const socketUrl = servers[selectedServerIndex].socketUrl;
			const apiKey = servers[selectedServerIndex].apiKey;
			const username = servers[selectedServerIndex].username;

			setOpenConfirmation(false);
			setConnecting(true);
			try {
				await connectSocket(socketUrl, apiKey, username);
			} catch (error) {
				console.error("Connection failed:", error);
			} finally {
				setConnecting(false);
			}
		}
	};

	const handleEditServer = (updatedServer: Server) => {
		if (selectedServerIndex !== undefined) {
			updateServer(selectedServerIndex, updatedServer);
			(() => {
				setOpenEditServer(false);
				setSelectedServerIndex(undefined);
			})();
		}
	};

	return (
		<>
			<ConnectionSpinner open={connecting} />
			<Container>
				<Typography variant="h4" gutterBottom>
					Select a Server to Connect
				</Typography>
				<Grid container spacing={3}>
					{servers.map((server, index) => (
						<Grid key={server.serverName} size={{ xs: 12, sm: 6, md: 4 }}>
							<ServerCard
								index={index}
								onSelect={(index: number) => {
									setSelectedServerIndex(index);
									setOpenConfirmation(true);
								}}
								onEdit={(index: number) => {
									setSelectedServerIndex(index);
									setOpenEditServer(true);
								}}
								onDelete={(index: number) => {
									removeServer(index);
								}}
							/>
						</Grid>
					))}
					<Grid size={{ xs: 12, sm: 6, md: 4 }}>
						<Card variant="outlined">
							<CardActionArea
								onClick={() => {
									setOpenAddServer(true);
								}}
							>
								<CardContent
									style={{
										display: "flex",
										justifyContent: "center",
										alignItems: "center",
										height: "100%",
									}}
								>
									<AddIcon fontSize="large" />
								</CardContent>
							</CardActionArea>
						</Card>
					</Grid>
				</Grid>
				<Dialog
					open={openConfirmation}
					onClose={() => {
						setOpenConfirmation(false);
						setSelectedServerIndex(undefined);
					}}
				>
					<DialogTitle>Confirm Selection</DialogTitle>
					<DialogContent>
						{selectedServerIndex !== undefined &&
						servers[selectedServerIndex] ? (
							<Typography>
								Are you sure you want to connect to{" "}
								<strong>{servers[selectedServerIndex].serverName}</strong>?
							</Typography>
						) : (
							<Typography>No server selected.</Typography>
						)}
					</DialogContent>
					<DialogActions>
						<Button
							onClick={() => {
								setOpenConfirmation(false);
								setSelectedServerIndex(undefined);
							}}
						>
							Cancel
						</Button>
						<Button
							variant="contained"
							color="primary"
							onClick={handleConfirmSelection}
							disabled={selectedServerIndex === null}
						>
							Connect
						</Button>
					</DialogActions>
				</Dialog>
				<AddEditServerDialog
					open={openAddServer}
					onClose={() => {
						setOpenAddServer(false);
					}}
					onSave={(newServer: Server) => {
						addServer(newServer);
						(() => {
							setOpenAddServer(false);
						})();
					}}
					title="Add New Server"
				/>
				<AddEditServerDialog
					open={openEditServer}
					onClose={() => {
						setOpenEditServer(false);
						setSelectedServerIndex(undefined);
					}}
					onSave={handleEditServer}
					title="Edit Server"
					index={selectedServerIndex}
				/>
			</Container>
		</>
	);
};

export default ServerSelectionPage;
