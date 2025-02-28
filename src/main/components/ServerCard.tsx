import type React from "react";
import {
	Card,
	CardContent,
	CardActions,
	IconButton,
	Typography,
} from "@mui/material";
import EditIcon from "@mui/icons-material/Edit";
import DeleteIcon from "@mui/icons-material/Delete";
import { useServerContext } from "../contexts/ServerContext";

interface ServerCardProps {
	index: number;
	onSelect: (index: number) => void;
	onEdit: (index: number) => void;
	onDelete: (index: number) => void;
}

const ServerCard: React.FC<ServerCardProps> = ({
	index,
	onSelect,
	onEdit,
	onDelete,
}) => {
	const { servers } = useServerContext();
	const server = servers[index];

	return (
		<Card variant="outlined">
			<CardContent onClick={() => onSelect(index)}>
				<Typography variant="h5" component="div">
					{server.serverName}
				</Typography>
				<Typography variant="body2" color="text.secondary">
					{server.socketUrl}
				</Typography>
			</CardContent>
			<CardActions>
				<IconButton aria-label="edit" onClick={() => onEdit(index)}>
					<EditIcon />
				</IconButton>
				<IconButton aria-label="delete" onClick={() => onDelete(index)}>
					<DeleteIcon />
				</IconButton>
			</CardActions>
		</Card>
	);
};

export default ServerCard;
