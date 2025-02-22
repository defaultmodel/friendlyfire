import type React from "react";
import { useEffect, useState } from "react";
import {
	List,
	ListItem,
	ListItemAvatar,
	Avatar,
	ListItemText,
	Typography,
	Paper,
} from "@mui/material";
import { useSocket } from "../context/SocketContext";

const UserList: React.FC = () => {
	const [users, setUsers] = useState<string[]>([]); // State to store the list of users
	const { socket } = useSocket();

	// Listen for the "user list" event from the server
	useEffect(() => {
		if (!socket) return;

		socket.on("user list", (usernames: string[]) => {
			setUsers(usernames); // Update the user list
		});

		// Cleanup the event listener on unmount
		return () => {
			socket.off("user list");
		};
	}, [socket]);

	return (
		<Paper
			elevation={3}
			style={{ padding: "16px", maxWidth: "300px", margin: "16px" }}
		>
			<Typography variant="h6" gutterBottom>
				Connected Users
			</Typography>
			<List>
				{users.map((username, index) => (
					// biome-ignore lint/suspicious/noArrayIndexKey: <explanation>
					<ListItem key={index}>
						<ListItemAvatar>
							<Avatar>{username[0].toUpperCase()}</Avatar>{" "}
							{/* Show the first letter of the username */}
						</ListItemAvatar>
						<ListItemText primary={username} />
					</ListItem>
				))}
			</List>
		</Paper>
	);
};

export default UserList;
