/// <reference types="vite/client" />

declare module 'virtual:parity' {
  const data: import('./lib/model').ParityData
  export default data
}
