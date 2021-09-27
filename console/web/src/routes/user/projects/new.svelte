<script lang="ts">
	import { Button, Card, TextField } from 'attractions';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import { slide } from 'svelte/transition';
	import { createProject } from '$lib/api/projects';
	import { goto } from '$app/navigation';

	const schema = yup.object({
		name: yup.string().required(),
	});

	let error = null;

	const { form, isSubmitting, isValid } = createForm({
		onSubmit: async (values: any) => {
			error = null;
			const res = await createProject(values);
			goto(`/user/projects/${res._id}`);
		},
		onError: (e: any) => {
			error =
				e.response?.status === 400
					? 'Incorrect fields.'
					: 'Internal error. Please try again later.';
		},
		extend: validator,
		validateSchema: schema,
	});
</script>

<section class="flex flex-grow justify-center items-center">
	<Card>
		<form use:form class="flex flex-col md:w-72 lg:w-96 space-y-8">
			<div>
				<label for="name">Name</label>
				<TextField name="name" autocomplete="off" />
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
				<span class="px-4">Create project</span>
			</Button>
		</form>
	</Card>
</section>
