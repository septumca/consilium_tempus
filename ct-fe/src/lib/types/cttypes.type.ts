export type User = {
  _id: string,
  name: string,
}

export type RegistrationData = {
  name: string,
  password: string,
}

export type LoginRequest = {
  name: string,
  password: string,
}

export type LoginResponse = {
  _id: string,
  name: string,
  token: string,
}

export type CreateTask = {
  name: string,
  description?: string,
  creator: User,
  assignee?: User,
}

export type Task = {
  _id: string,
  name: string,
  description: string,
  status: number
  creator: User,
  created_on: Date,
  assignee: User | null,
}

export type UpdateTaskData = {
  name?: string,
  description?: string,
  status?: number
  assignee?: User | null,
}

export type RefData = {
  statuses: Array<{ name: string, code: number }>
}