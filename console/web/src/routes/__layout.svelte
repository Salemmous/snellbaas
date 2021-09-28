<script lang="ts">
	import Footer from '$lib/Footer.svelte';
	import Header from '$lib/Header.svelte';
	import '../app.css';
</script>

<Header />
<main class="flex flex-grow flex-col w-full mx-auto">
	<slot />
</main>

<Footer />
