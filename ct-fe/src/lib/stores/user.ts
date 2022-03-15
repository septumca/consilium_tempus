import { writable } from 'svelte/store';
import type { User } from '../types/cttypes.type';

const store = writable<Array<User>>([]);

export default store;
