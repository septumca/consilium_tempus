import type { LoginResponse } from "../types/cttypes.type";

const LOCALSTORAGE_USER = "ct-user";

export const setLoggedUser = (user: LoginResponse) => {
  localStorage.setItem(LOCALSTORAGE_USER, JSON.stringify(user));
}

export const clearLoggedUser = () => {
  localStorage.setItem(LOCALSTORAGE_USER, null);
}

export const getLoggedUser = (): LoginResponse => JSON.parse(localStorage.getItem(LOCALSTORAGE_USER));

export const isUserLogged = (): boolean => localStorage.getItem(LOCALSTORAGE_USER) !== null;

export const getToken = (): string => getLoggedUser().token;
