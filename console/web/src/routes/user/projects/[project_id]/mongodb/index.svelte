<script lang="ts">
	import { page } from '$app/stores';
	import { Button } from 'attractions';
</script>

<div class="p-4"><p>Select a collection to explore it</p></div>
<div class="flex">
	<Button href={`/user/projects/${$page.params.project_id}/mongodb/new`}>Create new</Button>
</div>
