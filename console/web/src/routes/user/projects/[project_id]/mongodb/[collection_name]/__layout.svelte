<script lang="ts">
	import { goto } from '$app/navigation';

	import { page } from '$app/stores';
	import { getDocuments } from '$lib/api/mongodb';
	import type { IMongoDBDocument } from '$lib/models/mongodb';
	import { Button, Loading } from 'attractions';
	let documents: IMongoDBDocument[];

	const fetchDocuments = async (projectId, collectionName) => {
		documents = await getDocuments(
			projectId,
			collectionName,
			{},
			{
				projection: {
					_id: true,
				},
			},
		);
	};

	$: fetchDocuments($page.params.project_id, $page.params.collection_name);

	const handleSelectDocument = (document) => {
		goto(
			`/user/projects/${$page.params.project_id}/mongodb/${$page.params.collection_name}/${document._id.$oid}`,
		);
	};
</script>

<section class="flex">
	<div class="p-2">
		{#if documents}
			<ul>
				{#each documents as document}
					<li>
						<Button
							small
							selected={document._id.$oid === $page.params.document_id}
							rectangle
							on:click={() => handleSelectDocument(document)}
						>
							{document._id.$oid}
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
