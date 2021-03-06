import type { CreateTask, LoginRequest, LoginResponse, RegistrationData, Task, UpdateTaskData, User } from "../types/cttypes.type"
import { getToken } from "./auth";

const API: string = "http://localhost:7005";
const TASKS_API: string = `${API}/tasks`;
const USER_API: string = `${API}/users`;
const REFDATA_API: string = `${API}/reference_data`;

type Options = {
  method: string,
  headers: {
    "Content-Type": string,
    "X-JWT-Token"?: string,
  },
  body?: string,
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

const fetchWIthToken = (url: string, options: Options) => {
  const token = getToken();
  return fetch(url, { ...options, headers: { ...options.headers, "X-JWT-Token": token }});
}

export const register = async (data: RegistrationData) => {
  let r = await fetch(`${API}/register`, { ...POST_OPTIONS, body: JSON.stringify(data) });
  return r.json();
}

export const login = async (data: LoginRequest): Promise<{ status:number, data: LoginResponse }> => {
  let r = await fetch(`${API}/authentificate`, { ...POST_OPTIONS, body: JSON.stringify(data) });
  return { status: r.status, data: await r.json() as LoginResponse };
}

export const readSingleUser = async (id: string) => {
  let r = await fetchWIthToken(`${USER_API}/${id}`, GET_OPTIONS);
  return r.json();
}

export const readAllUsers = async () => {
  let r = await fetchWIthToken(USER_API, GET_OPTIONS);
  return r.json();
}

export const updateUser = async (id: string, data: User) => {
  let _ = await fetchWIthToken(`${USER_API}/${id}`, { ...PUT_OPTIONS, body: JSON.stringify(data) });
}

export const deleteUser = async (id: string) => {
  let _ = await fetchWIthToken(`${USER_API}/${id}`, DELETE_OPTIONS);
}

export const createTask = async (data: CreateTask) => {
  let r = await fetchWIthToken(TASKS_API, { ...POST_OPTIONS, body: JSON.stringify(data) });
  return r.json();
}

export const readSingleTask = async (id: string) => {
  let r = await fetchWIthToken(`${TASKS_API}/${id}`, GET_OPTIONS);
  return r.json();
}

export const readAllTasks = async () => {
  let r = await fetchWIthToken(TASKS_API, GET_OPTIONS);
  return r.json();
}

export const readUsersTasks = async (id: string) => {
  let r = await fetchWIthToken(`${TASKS_API}/user/${id}`, GET_OPTIONS);
  return r.json();
}

export const updateTask = async (id: string, data: UpdateTaskData) => {
  let _ = await fetchWIthToken(`${TASKS_API}/${id}`, { ...PUT_OPTIONS, body: JSON.stringify(data) });
}

export const deleteTask = async (id: string) => {
  let _ = await fetchWIthToken(`${TASKS_API}/${id}`, DELETE_OPTIONS);
}

export const readRefData = async () => {
  let r = await fetchWIthToken(REFDATA_API, GET_OPTIONS);
  return r.json();
}
