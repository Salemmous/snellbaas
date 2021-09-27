<script lang="ts">
	import { page } from '$app/stores';
	import type { IProject } from '$lib/models/project';
	import { fetchProject } from '$lib/stores/projects';
	import { Loading } from 'attractions';
	import { onMount } from 'svelte';

	let project: IProject;

	onMount(async () => {
		project = await fetchProject($page.params.project_id);
	});
</script>

{#if project}
	<h1>{project.name}</h1>
{:else}
	<Loading />
{/if}
