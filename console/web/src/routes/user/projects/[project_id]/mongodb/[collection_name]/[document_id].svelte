<script lang="ts">
	import { page } from '$app/stores';
	import {
		createDocument,
		deleteDocument,
		getDocument,
		setDocument,
		updateDocument,
	} from '$lib/api/mongodb';
	import type { IMongoDBDocument } from '$lib/models/mongodb';
	import { Button, Loading } from 'attractions';
	let document: IMongoDBDocument;
	import JSONEditor from '$lib/components/JSONEditor.svelte';
	import { goto } from '$app/navigation';

	$: fetchDocument($page.params.project_id, $page.params.collection_name, $page.params.document_id);

	const fetchDocument = async (projectId, collectionName, documentId) => {
		hasChanged = false;
		document = await getDocument(projectId, collectionName, documentId);
	};

	const handleEditable = ({ path }: { path: string[] }) => {
		if (!path) return true;
		const totalPath = path.join('/');
		return totalPath !== '_id' && totalPath !== '_id/$oid';
	};

	let hasChanged = false;

	const handleDelete = async () => {
		const sure = confirm('Are you sure?');
		await deleteDocument(
			$page.params.project_id,
			$page.params.collection_name,
			$page.params.document_id,
		);
		goto(`/user/projects/${$page.params.project_id}/mongodb/${$page.params.collection_name}`);
	};

	const handleDuplicate = async () => {
		const newDoc = {
			...document,
		};
		delete newDoc._id;
		const res = await createDocument($page.params.project_id, $page.params.collection_name, newDoc);
		goto(
			`/user/projects/${$page.params.project_id}/mongodb/${$page.params.collection_name}/${res._id}`,
		);
	};

	const handleSave = async () => {
		await setDocument(
			$page.params.project_id,
			$page.params.collection_name,
			$page.params.document_id,
			document,
			{ upsert: true },
		);
		hasChanged = false;
	};
</script>

<div class="p-4">
	{#if document}
		<div class="p-2 flex justify-end space-x-2">
			<Button small rectangle danger filled on:click={handleDelete}>Delete</Button>
			<Button small rectangle filled neutral on:click={handleDuplicate}>Duplicate</Button>
			<Button small rectangle filled disabled={!hasChanged} on:click={handleSave}>Save</Button>
		</div>
		<JSONEditor
			bind:content={document}
			editable={handleEditable}
			on:change={() => (hasChanged = true)}
		/>
	{:else}
		<Loading />
	{/if}
</div>
