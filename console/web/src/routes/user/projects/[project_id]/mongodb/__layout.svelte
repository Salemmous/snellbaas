<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { listCollections } from '$lib/api/mongodb';
	import type { IMongoDBCollection } from '$lib/models/mongodb';
	import { Button, Loading } from 'attractions';
	let collections: IMongoDBCollection[];

	const fetchCollections = async (projectId) => {
		collections = await listCollections(projectId);
	};

	$: fetchCollections($page.params.project_id);

	const handleSelectCollection = (collection) => {
		if (collection.name === $page.params.collection_name) {
			goto(`/user/projects/${$page.params.project_id}/mongodb/`);
			return;
		}
		goto(`/user/projects/${$page.params.project_id}/mongodb/${collection.name}`);
	};
</script>

<section class="flex">
	<div class="p-2">
		{#if collections}
			<ul>
				{#each collections as collection}
					<li>
						<Button
							small
							selected={$page.params.collection_name === collection.name}
							rectangle
							on:click={() => handleSelectCollection(collection)}
						>
							{collection.name}
						</Button>
					</li>
				{/each}
			</ul>
		{:else}
			<Loading />
		{/if}
	</div>
	<div class="flex-grow">
		<slot />
	</div>
</section>
