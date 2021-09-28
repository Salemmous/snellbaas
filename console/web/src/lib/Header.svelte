<script lang="ts">
	import { page } from '$app/stores';
	import { Button, Loading } from 'attractions';
	import { currentUser, currentUserToken, logout } from './stores/auth';

	const handleLogout = () => {
		logout();
	};
</script>

<header class="w-full flex justify-between items-center p-4 bg-gray-200 elevation-2">
	<div />
	<nav class="flex w-full">
		<ul class="flex space-x-4">
			<li class:active={$page.path === '/'}><Button rectangle small href="/">SnellBaas</Button></li>
			<li class:active={$page.path === '/about'} class="hidden md:block">
				<Button rectangle small href="/about">About</Button>
			</li>
		</ul>
	</nav>
	<div>
		{#if $currentUserToken && !$currentUser}
			<Button small on:click={handleLogout}>
				<Loading />
			</Button>
		{:else if $currentUserToken && $currentUser}
			<Button small on:click={handleLogout}>{$currentUser.username}</Button>
		{/if}
	</div>
</header>
