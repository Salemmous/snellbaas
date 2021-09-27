<script lang="ts">
	import { fetchUserProject, userProjects } from '$lib/stores/projects';

	import { Button, Loading } from 'attractions';
	import { onMount } from 'svelte';

	onMount(() => {
		fetchUserProject();
	});
</script>

<section class="flex flex-grow justify-center items-center">
	{#if $userProjects}
		<div class="grid grid-cols-3 gap-4">
			{#each $userProjects as project}
				<Button class="justify-center" href={`/user/projects/${project._id.$oid}`} outline>
					<h2>{project.name}</h2>
				</Button>
			{/each}
			<Button class="justify-center" href="/user/projects/new" outline>
				<h2>Create a new project</h2>
			</Button>
		</div>
	{:else}
		<Loading />
	{/if}
</section>
