import type React from "react";
import { useState, useRef, useEffect } from "react";
import {
	Container,
	Typography,
	TextField,
	Button,
	Stepper,
	Step,
	StepLabel,
} from "@mui/material";
import type { Server } from "../../types";
import { useServerContext } from "../contexts/ServerContext";

const GetStartedPage = () => {
	const { addServer } = useServerContext();
	const [activeStep, setActiveStep] = useState(0);
	const [serverDetails, setServerDetails] = useState<Server>({
		serverName: "",
		socketUrl: "",
		username: "",
		apiKey: "",
	});

	const inputRefs = [
		useRef<HTMLInputElement>(null),
		useRef<HTMLInputElement>(null),
		useRef<HTMLInputElement>(null),
		useRef<HTMLInputElement>(null),
	];

	// biome-ignore lint/correctness/useExhaustiveDependencies: <adding attributes of the object is redundant>
	useEffect(() => {
		if (
			activeStep >= 0 &&
			activeStep < inputRefs.length &&
			inputRefs[activeStep].current
		) {
			inputRefs[activeStep].current.focus();
		}
	}, [activeStep]);

	const steps = [
		"Enter Server Name",
		"Enter Socket URL",
		"Enter Username",
		"Enter API Key",
	];

	const handleNext = () => {
		setActiveStep((prevActiveStep) => prevActiveStep + 1);
	};

	const handleBack = () => {
		setActiveStep((prevActiveStep) => prevActiveStep - 1);
	};

	const handleChange =
		(field: keyof Server) => (event: React.ChangeEvent<HTMLInputElement>) => {
			setServerDetails((prevDetails) => ({
				...prevDetails,
				[field]: event.target.value,
			}));
		};

	const handleFinish = () => {
		addServer(serverDetails);
		setActiveStep(0);
		setServerDetails({
			serverName: "",
			socketUrl: "",
			username: "",
			apiKey: "",
		});
	};

	const getStepContent = (step: number) => {
		switch (step) {
			case 0:
				return (
					<TextField
						inputRef={inputRefs[0]}
						label="Server Name"
						value={serverDetails.serverName}
						onChange={handleChange("serverName")}
						fullWidth
					/>
				);
			case 1:
				return (
					<TextField
						inputRef={inputRefs[1]}
						label="Socket URL"
						value={serverDetails.socketUrl}
						onChange={handleChange("socketUrl")}
						fullWidth
					/>
				);
			case 2:
				return (
					<TextField
						inputRef={inputRefs[2]}
						label="Username"
						value={serverDetails.username}
						onChange={handleChange("username")}
						fullWidth
					/>
				);
			case 3:
				return (
					<TextField
						inputRef={inputRefs[3]}
						label="API Key"
						value={serverDetails.apiKey}
						onChange={handleChange("apiKey")}
						fullWidth
					/>
				);
			default:
				return "Unknown step";
		}
	};

	return (
		<Container>
			<Typography variant="h4" gutterBottom>
				Get Started
			</Typography>
			<Stepper activeStep={activeStep} alternativeLabel>
				{steps.map((label) => (
					<Step key={label}>
						<StepLabel>{label}</StepLabel>
					</Step>
				))}
			</Stepper>
			<div>
				{activeStep === steps.length ? (
					<div>
						<Typography variant="h5" gutterBottom>
							All steps completed
						</Typography>
						<Button variant="contained" color="primary" onClick={handleFinish}>
							Finish
						</Button>
					</div>
				) : (
					<div>
						{getStepContent(activeStep)}
						<div>
							<Button disabled={activeStep === 0} onClick={handleBack}>
								Back
							</Button>
							<Button variant="contained" color="primary" onClick={handleNext}>
								{activeStep === steps.length - 1 ? "Finish" : "Next"}
							</Button>
						</div>
					</div>
				)}
			</div>
		</Container>
	);
};

export default GetStartedPage;
