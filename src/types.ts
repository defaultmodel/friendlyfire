export interface FileDropEventPayload {
	paths: string[];
}

// Comes from type declaration of savedImageData from reac-filerobot-image-editor@4.9.1
export type savedImageData = {
	name: string;
	extension: string;
	mimeType: string;
	fullName?: string;
	height?: number;
	width?: number;
	imageBase64?: string;
	imageCanvas?: HTMLCanvasElement; // doesn't support quality
	quality?: number;
	cloudimageUrl?: string;
};
