<script lang="ts">
	import { page } from '$app/stores';
	import type { IProject } from '$lib/models/project';
	import { fetchProject } from '$lib/stores/projects';
	import { Button, Loading, Tab } from 'attractions';
	import { onMount } from 'svelte';

	let project: IProject;

	onMount(async () => {
		project = await fetchProject($page.params.project_id);
	});
</script>

{#if project}
	<div class="flex flex-grow flex-col md:flex-row">
		<nav class="hidden md:flex flex-col bg-gray-100 elevation-2">
			<Button neutral class="button-nav" href={`/user/projects/${$page.params.project_id}`}>
				<h1 class="p-4">{project.name}</h1>
			</Button>
			<ul>
				<li>
					<Button
						neutral
						class="button-nav"
						href={`/user/projects/${$page.params.project_id}/auth`}
					>
						Auth
					</Button>
				</li>
				<li>
					<Button
						neutral
						class="button-nav"
						href={`/user/projects/${$page.params.project_id}/mongodb`}
					>
						MongoDB
					</Button>
				</li>
				<li>
					<Button
						neutral
						class="button-nav"
						href={`/user/projects/${$page.params.project_id}/storage`}
					>
						Storage
					</Button>
				</li>
				<li>
					<Button
						neutral
						class="button-nav"
						href={`/user/projects/${$page.params.project_id}/functions`}
					>
						Functions
					</Button>
				</li>
				<li>
					<Button
						neutral
						class="button-nav"
						href={`/user/projects/${$page.params.project_id}/notifications`}
					>
						Notifications
					</Button>
				</li>
			</ul>
		</nav>
		<nav class="md:hidden flex overflow-x-scroll">
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}`}>
				{project.name}
			</Button>
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}/auth`}>
				Auth
			</Button>
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}/mongodb`}>
				MongoDB
			</Button>
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}/storage`}>
				Storage
			</Button>
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}/functions`}>
				Functions
			</Button>
			<Button class="button-nav" href={`/user/projects/${$page.params.project_id}/notifications`}>
				Notifications
			</Button>
		</nav>
		<div class="flex-grow">
			<slot />
		</div>
	</div>
{:else}
	<Loading />
{/if}

<style>
	nav :global(a.button-nav) {
		border-radius: 0 !important;
	}
</style>
