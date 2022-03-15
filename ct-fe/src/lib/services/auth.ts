import type { User } from "../types/cttypes.type";

const LOCALSTORAGE_USER = "ct-user";

export const setLoggedUser = (user: User) => {
  localStorage.setItem(LOCALSTORAGE_USER, JSON.stringify(user));
}

export const getLoggedUser = (): User => {
  return JSON.parse(localStorage.getItem(LOCALSTORAGE_USER));
}

export const clearLoggedUser = () => {
  localStorage.setItem(LOCALSTORAGE_USER, null);
}

export const isUserLogged = (): boolean => {
  return localStorage.getItem(LOCALSTORAGE_USER) !== null;
}