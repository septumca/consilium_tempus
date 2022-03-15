import { writable } from 'svelte/store';
import type { RefData } from '../types/cttypes.type';

const store = writable<RefData>({ statuses: [] });

export default store;
