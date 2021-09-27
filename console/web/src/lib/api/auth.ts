import type { IRegisterUser, IUser } from '$lib/models/user';
import { getClient } from './client';
export async function loginWithCredentials(email: string, password: string) {
	const res = await getClient().post('/auth/login', { email, password });
	return res;
}

export async function register(info: IRegisterUser) {
	const res = await getClient().post('/auth/signup', info);
	return res;
}

export async function getProfile(): Promise<IUser> {
	const res = await getClient().get('/auth/profile');
	return res.data;
}
