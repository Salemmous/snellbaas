import { writable } from 'svelte/store';
import type { StartStopNotifier, Writable } from 'svelte/store';
import { getObjectFromLocalStorage, setObjectFromLocalStorage } from './local_storage';

export function savedWritable<T>(
	key: string,
	defaultValue?: T,
	start?: StartStopNotifier<T>,
): Writable<T> {
	const store = writable<T>(getObjectFromLocalStorage(key) || defaultValue, start);
	store.subscribe((value) => setObjectFromLocalStorage(key, value));
	return store;
}
