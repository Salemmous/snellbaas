<script lang="ts">
	import 'jsoneditor/dist/jsoneditor.min.css';
	import { createEventDispatcher, onMount } from 'svelte';

	const dispatch = createEventDispatcher();

	let container;
	let editor;
	export let options = {};
	export let content;
	interface IFieldParams {
		path: string[];
		value: string;
		field: string;
	}
	export let editable: (params: IFieldParams) => boolean = null;
	export let className: (params: IFieldParams) => string = null;

	let insideChange = false;

	onMount(async () => {
		const { default: JSONEditor } = await import('jsoneditor');
		editor = new JSONEditor(container, {
			...options,
			onChange: (event) => {
				insideChange = true;
				content = editor.get();
				dispatch('change', event);
			},
			onChangeJSON: (event) => dispatch('change-json', event),
			onChangeTEXT: (event) => dispatch('change-text', event),
			onClassName: className,
			onEditable: editable,
		});
		editor.set(content);
	});

	const handleContentChanged = (content) => {
		if (editor && !insideChange) editor.set(content);
		insideChange = false;
	};

	$: handleContentChanged(content);
</script>

<div bind:this={container} />
