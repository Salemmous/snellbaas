<script lang="ts">
	import { goto } from '$app/navigation';
	import * as yup from 'yup';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import { Button, TextField } from 'attractions';
	import { slide } from 'svelte/transition';
	import { createCollection } from '$lib/api/mongodb';
	import { page } from '$app/stores';

	const schema = yup.object({
		name: yup.string().required(),
	});

	let error = null;

	const { form, isSubmitting, isValid } = createForm({
		onSubmit: async (values: any) => {
			error = null;
			await createCollection($page.params.project_id, values.name);
			goto(`/user/projects/${$page.params.project_id}/mongodb/${values.name}`);
		},
		onError: (e: any) => {
			error =
				e.response?.status === 400
					? 'The email or password is incorrect.'
					: 'Internal error. Please try again later.';
		},
		extend: validator,
		validateSchema: schema,
	});
</script>

<form use:form class="flex flex-col md:w-72 lg:w-96 space-y-8 p-4">
	<div>
		<label for="name">Name</label>
		<TextField name="name" type="name" />
	</div>
	{#if error}
		<span transition:slide class="text-red-400">{error}</span>
	{/if}
	<Button
		disabled={$isSubmitting || !$isValid}
		filled
		class="mt-8 text-center justify-center self-center"
		type="submit"
	>
		<span class="px-4">Create</span>
	</Button>
</form>
