export function randomChoice<T>(items: T[]): T {
  return items[Math.floor(Math.random() * items.length)]
}
