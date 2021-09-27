import axios, { AxiosInstance } from 'axios';

const API_URL = import.meta.env.VITE_API_URL || '';

let _client: AxiosInstance = axios.create({ baseURL: `${API_URL}/api` });
setInterceptors();

export function getClient(): AxiosInstance {
	return _client;
}

export function setAuthToken(token: string | null) {
	_client = axios.create({
		baseURL: `${API_URL}/api`,
		headers: { Authorization: token ? `Bearer ${token}` : '' },
	});
	setInterceptors();
}

function setInterceptors() {
	_client.interceptors.response.use(
		(response) => {
			return response;
		},
		(error) => {
			return Promise.reject(error);
		},
	);
}
