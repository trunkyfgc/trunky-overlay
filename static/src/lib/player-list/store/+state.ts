
import { broadcastable } from '$lib/shared'
import key from './+key'


const initial_value: string[] = []

export const playerListStore = {
  ...broadcastable(key, initial_value)
}
