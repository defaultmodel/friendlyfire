import { useState } from "react";
import {
	Container,
	Typography,
	Card,
	CardContent,
	Button,
	Grid,
	Dialog,
	DialogTitle,
	DialogContent,
	DialogActions,
} from "@mui/material";
import { useServerContext } from "../contexts/ServerContext";

const ServerSelectionPage = () => {
	const { servers } = useServerContext();
	const [selectedServerIndex, setSelectedServerIndex] = useState<number | null>(
		null,
	);
	const [openConfirmation, setOpenConfirmation] = useState(false);

	const handleSelectServer = (index: number) => {
		setSelectedServerIndex(index);
		setOpenConfirmation(true);
	};

	const handleConfirmSelection = () => {
		if (selectedServerIndex !== null && servers[selectedServerIndex]) {
			// Logic to connect to the selected server
			console.log(
				`Connecting to server: ${servers[selectedServerIndex].serverName}`,
			);
			setOpenConfirmation(false);
			// Additional logic to handle connection can be added here
		}
	};

	const handleCloseConfirmation = () => {
		setOpenConfirmation(false);
		setSelectedServerIndex(null);
	};

	return (
		<Container>
			<Typography variant="h4" gutterBottom>
				Select a Server to Connect
			</Typography>
			{servers.length > 0 ? (
				<Grid container spacing={3}>
					{servers.map((server, index) => (
						<Grid item key={index} xs={12} sm={6} md={4}>
							<Card
								variant="outlined"
								onClick={() => handleSelectServer(index)}
							>
								<CardContent>
									<Typography variant="h5" component="div">
										{server.serverName}
									</Typography>
									<Typography variant="body2" color="text.secondary">
										{server.socketUrl}
									</Typography>
								</CardContent>
							</Card>
						</Grid>
					))}
				</Grid>
			) : (
				<Typography variant="body1">
					No servers available. Please add a server first.
				</Typography>
			)}
			<Dialog open={openConfirmation} onClose={handleCloseConfirmation}>
				<DialogTitle>Confirm Selection</DialogTitle>
				<DialogContent>
					{selectedServerIndex !== null && servers[selectedServerIndex] ? (
						<Typography>
							Are you sure you want to connect to{" "}
							<strong>{servers[selectedServerIndex].serverName}</strong>?
						</Typography>
					) : (
						<Typography>No server selected.</Typography>
					)}
				</DialogContent>
				<DialogActions>
					<Button onClick={handleCloseConfirmation}>Cancel</Button>
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
		</Container>
	);
};

export default ServerSelectionPage;
