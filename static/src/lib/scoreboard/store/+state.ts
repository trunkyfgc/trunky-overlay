import key from './+key'
import type { Scoreboard } from '../model'
import { broadcastable } from '$lib/shared/broadcast-store'

const player = {
  name: "",
  score: 0
}

const initial_value: Scoreboard = {
  player1: { ...player },
  player2: { ...player },
  title: ""
}

export const scoreboardStore = { ...broadcastable(key, initial_value) }

