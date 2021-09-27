<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import { currentUserToken } from '$lib/stores/auth';

	import { Button } from 'attractions';
</script>

<svelte:head>
	<title>SnellBaas</title>
</svelte:head>

<section class="flex flex-col justify-center items-center flex-grow">
	<h2>Welcome to</h2>
	<h1 class="w-full text-center text-8xl my-2">SnellBaas</h1>
	<small>Because I could not find a better name</small>

	<div class="space-x-4 mt-4 flex">
		{#if $currentUserToken}
			<Button class="mt-4" href="/user/projects" filled>My projects</Button>
		{:else}
			<Button class="mt-4" href="/auth/register" filled>Create an account</Button>
			<Button class="mt-4" href="/auth/login">Log in</Button>
		{/if}
	</div>
</section>
