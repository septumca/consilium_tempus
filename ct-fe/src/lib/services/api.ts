import type { CreateTask, Task, UpdateTaskData, User } from "../types/cttypes.type"

const API: string = "http://localhost:7005";
const TASKS_API: string = `${API}/tasks`;
const USER_API: string = `${API}/users`;
const REFDATA_API: string = `${API}/reference_data`;

type Options = {
  method: string,
  headers: {
    "Content-Type": string
  }
}

const GET_OPTIONS: Options = {
  method: "GET",
  headers: {
    "Content-Type": "application/json"
  }
}

const POST_OPTIONS: Options = {
  method: "POST",
  headers: {
    "Content-Type": "application/json"
  }
}

const PUT_OPTIONS: Options = {
  method: "PUT",
  headers: {
    "Content-Type": "application/json"
  }
}

const DELETE_OPTIONS: Options = {
  method: "DELETE",
  headers: {
    "Content-Type": "application/json"
  }
}

export const createUser = async (data: User) => {
  let r = await fetch(USER_API, { ...POST_OPTIONS, body: JSON.stringify(data) });
  return r.json();
}

export const readSingleUser = async (id: string) => {
  let r = await fetch(`${USER_API}/${id}`, GET_OPTIONS);
  return r.json();
}

export const readAllUsers = async () => {
  let r = await fetch(USER_API, GET_OPTIONS);
  return r.json();
}

export const updateUser = async (id: string, data: User) => {
  let _ = await fetch(`${USER_API}/${id}`, { ...PUT_OPTIONS, body: JSON.stringify(data) });
}

export const deleteUser = async (id: string) => {
  let _ = await fetch(`${USER_API}/${id}`, DELETE_OPTIONS);
}

export const createTask = async (data: CreateTask) => {
  let r = await fetch(TASKS_API, { ...POST_OPTIONS, body: JSON.stringify(data) });
  return r.json();
}

export const readSingleTask = async (id: string) => {
  let r = await fetch(`${TASKS_API}/${id}`, GET_OPTIONS);
  return r.json();
}

export const readAllTasks = async () => {
  let r = await fetch(TASKS_API, GET_OPTIONS);
  return r.json();
}

export const readUsersTasks = async (id: string) => {
  let r = await fetch(`${TASKS_API}/user/${id}`, GET_OPTIONS);
  return r.json();
}

export const updateTask = async (id: string, data: UpdateTaskData) => {
  let _ = await fetch(`${TASKS_API}/${id}`, { ...PUT_OPTIONS, body: JSON.stringify(data) });
}

export const deleteTask = async (id: string) => {
  let _ = await fetch(`${TASKS_API}/${id}`, DELETE_OPTIONS);
}

export const readRefData = async () => {
  let r = await fetch(REFDATA_API, GET_OPTIONS);
  return r.json();
}
