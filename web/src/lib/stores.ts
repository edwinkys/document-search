import { writable } from "svelte/store"
import type { Writable } from "svelte/store"
import type { Alert } from "./types"

export const alerts: Writable<Alert[]> = writable([])
