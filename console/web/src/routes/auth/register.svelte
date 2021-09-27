<script lang="ts">
	import { Button, Card, TextField } from 'attractions';
	import { createForm } from 'felte';
	import { validator } from '@felte/validator-yup';
	import * as yup from 'yup';
	import { currentUserToken, register } from '$lib/stores/auth';
	import { slide } from 'svelte/transition';
	import { goto } from '$app/navigation';

	const schema = yup.object({
		email: yup.string().email().required(),
		username: yup.string().required(),
		firstName: yup.string().required(),
		lastName: yup.string().required(),
		password: yup.string().required()
	});

	let error = null;

	if ($currentUserToken) goto('/user/projects');

	const { form, isSubmitting, isValid } = createForm({
		onSubmit: async (values: any) => {
			error = null;
			await register(values);
			goto('/user/projects');
		},
		onError: (e: any) => {
			error =
				e.response?.status === 400
					? 'Incorrect fields.'
					: 'Internal error. Please try again later.';
		},
		extend: validator, // OR `extend: [validator],`
		validateSchema: schema
	});
</script>

<section class="flex flex-grow justify-center items-center">
	<Card>
		<form use:form class="flex flex-col md:w-72 lg:w-96 space-y-8">
			<div>
				<label for="email">Email</label>
				<TextField name="email" type="email" />
			</div>
			<div>
				<label for="username">Username</label>
				<TextField name="username" />
			</div>
			<div>
				<label for="firstName">First name</label>
				<TextField name="firstName" />
			</div>
			<div>
				<label for="lastName">Last name</label>
				<TextField name="lastName" />
			</div>
			<div>
				<label for="password">Password</label>
				<TextField name="password" type="password" />
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
				<span class="px-4">Register</span>
			</Button>
		</form>
	</Card>
</section>
