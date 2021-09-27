export function getObjectFromLocalStorage<T>(key: string): T {
	if (typeof localStorage === 'undefined') return null;
	try {
		return JSON.parse(localStorage.getItem(key));
	} catch (_) {
		return null;
	}
}

export function setObjectFromLocalStorage<T>(key: string, object: T): void {
	if (typeof localStorage === 'undefined') return null;
	return localStorage.setItem(key, JSON.stringify(object));
}
