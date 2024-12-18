import { writable, type Updater, type StartStopNotifier } from 'svelte/store';
import socket from './socket-singleton'

export function broadcastable<T>(name: string, value?: T, start: StartStopNotifier<T> = () => { }) {
  const {
    subscribe: _subscribe,
    set: _set,
    update: _update
  } = writable<T>(value, (set, update) => {
    socket.on('message-back', (message) => {
      if (message.from === socket.id || message.topic !== name) {
        return;
      }
      set(message.data)
    })

    const stop = start(set, update);

    return () => {
      socket?.close()
      if (stop) stop();
    };
  });

  function update(updater: Updater<T>) {
    _update((value) => {
      const newValue = updater(value);
      broadcastMessage(value)
      return newValue;
    });
  }

  function set(value: T) {
    broadcastMessage(value)
    _set(value);
  }

  function broadcastMessage(data: T) {
    socket?.emit('message', {
      data,
      topic: name,
      from: socket.id
    })
  }

  return {
    subscribe: _subscribe,
    set,
    update
  };
}
