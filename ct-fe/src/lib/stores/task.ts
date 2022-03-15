import { writable } from 'svelte/store';
import type { Task } from '../types/cttypes.type';

const store = writable<Array<Task>>([]);

export const addTask = (data: Task) => {
  store.update(s => [...s, data]);
}

export default store;
