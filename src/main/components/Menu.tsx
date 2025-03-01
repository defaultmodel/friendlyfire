import type React from "react";
import { useState } from "react";
import { Button, Menu as MuiMenu, MenuItem } from "@mui/material";
import { useSocket } from "../contexts/SocketContext";

const Menu: React.FC = () => {
	const { disconnectSocket, isConnected } = useSocket();
	const [anchorEl, setAnchorEl] = useState<null | HTMLElement>(null);

	const handleMenuOpen = (event: React.MouseEvent<HTMLButtonElement>) => {
		setAnchorEl(event.currentTarget);
	};

	const handleMenuClose = () => {
		setAnchorEl(null);
	};

	const handleDisconnect = () => {
		disconnectSocket();
		handleMenuClose();
	};

	return (
		<>
			<Button variant="contained" onClick={handleMenuOpen}>
				Menu
			</Button>
			<MuiMenu
				anchorEl={anchorEl}
				open={Boolean(anchorEl)}
				onClose={handleMenuClose}
			>
				{isConnected && (
					<MenuItem onClick={handleDisconnect}>Disconnect</MenuItem>
				)}
			</MuiMenu>
		</>
	);
};

export default Menu;
