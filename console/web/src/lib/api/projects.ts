import type { IProject } from '$lib/models/project';
import { getClient } from './client';

export async function getUserProjects(): Promise<IProject[]> {
	const res = await getClient().get('/projects/info/list');
	return res.data;
}

export async function getProject(id: string): Promise<IProject> {
	const res = await getClient().get(`/projects/info/${id}`);
	return res.data;
}

export async function createProject(project: IProject): Promise<{ _id: string }> {
	const res = await getClient().post('/projects/edit/new', project);
	return res.data;
}
