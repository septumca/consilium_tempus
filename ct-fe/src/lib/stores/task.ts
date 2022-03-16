import { writable } from 'svelte/store';
import type { Task } from '../types/cttypes.type';

const store = writable<Array<Task>>([]);

export const addTask = (data: Task) => {
  store.update(s => [...s, data]);
}

export const removeTask = (id: string) => {
  store.update(s => s.filter(t => t._id !== id));
}

export default store;
