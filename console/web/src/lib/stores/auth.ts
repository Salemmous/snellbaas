import * as auth from '$lib/api/auth';
import { setAuthToken } from '$lib/api/client';
import type { IRegisterUser, IUser } from '$lib/models/user';
import { savedWritable } from '$lib/utils/stores';

const CURRENT_USER_TOKEN_KEY = 'current_token_user';
const CURRENT_USER_KEY = 'current_user';

export const currentUserToken = savedWritable<string>(CURRENT_USER_TOKEN_KEY);
currentUserToken.subscribe(setAuthToken);
export const currentUser = savedWritable<IUser>(CURRENT_USER_KEY, null, (set) => {
	const unsubscribe = currentUserToken.subscribe((token) => {
		if (!token) {
			set(null);
			return;
		}
		getUserProfile();
	});
	return () => {
		unsubscribe();
	};
});

async function getUserProfile() {
	const res = await auth.getProfile();
	currentUser.set(res);
}

export async function loginWithCredentials(email: string, password: string) {
	const res = await auth.loginWithCredentials(email, password);
	const data = res.data;
	if (!data.success || !data.token) throw Error('Server denied request');
	currentUserToken.set(data.token);
	return data;
}

export async function register(info: IRegisterUser) {
	await auth.register(info);
	return loginWithCredentials(info.email, info.password);
}

export async function logout() {
	currentUserToken.set(null);
}
