<script lang="ts">
	import { goto } from '$app/navigation';

	import { page } from '$app/stores';
	import { createDocument } from '$lib/api/mongodb';

	import JSONEditor from '$lib/components/JSONEditor.svelte';
	import { Button } from 'attractions';

	let document = {};

	const handleEditable = ({ path }: { path: string[] }) => {
		if (!path) return true;
		const totalPath = path.join('/');
		return totalPath !== '_id' && totalPath !== '_id/$oid';
	};

	const handleSave = async () => {
		const res = await createDocument(
			$page.params.project_id,
			$page.params.collection_name,
			document,
		);

		goto(
			`/user/projects/${$page.params.project_id}/mongodb/${$page.params.collection_name}/${res._id}`,
		);
	};
</script>

<div class="p-4">
	<div class="p-2 flex justify-end space-x-2">
		<Button small rectangle filled on:click={handleSave}>Save</Button>
	</div>
	<JSONEditor bind:content={document} editable={handleEditable} />
</div>
