import { savedWritable } from '$lib/utils/stores';
import { currentUserToken } from './auth';
import * as api from '$lib/api/projects';
import type { IProject } from '$lib/models/project';
import { get } from 'svelte/store';

const USER_PROJECTS = 'user_projects';

export const userProjects = savedWritable<IProject[]>(USER_PROJECTS, null, (set) => {
	const unsubscribe = currentUserToken.subscribe((token) => {
		if (!token) {
			set(null);
			return;
		}
	});
	return () => {
		unsubscribe();
	};
});

export async function fetchUserProject() {
	const res = await api.getUserProjects();
	userProjects.set(res);
	return res;
}

export async function fetchProject(id: string) {
	const cachedProjects = get(userProjects);
	const cachedProject = cachedProjects.find(({ _id: { $oid } }) => $oid === id);
	if (cachedProject) return cachedProject;
	const res = await api.getProject(id);
	cacheProject(res);
	return res;
}

function cacheProject(project: IProject) {
	userProjects.update((value) => {
		if (!value.some(({ _id: { $oid } }) => $oid === project._id.$oid)) return [...value, project];
		return value.map((existingProject) => {
			if (existingProject._id.$oid === project._id.$oid) return project;
			return existingProject;
		});
	});
}
