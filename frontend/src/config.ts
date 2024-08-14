interface Config {
   apiUrl: string;
}

/**
   * Load configuration
 */
export function loadConfig(): Config {
   return {
      apiUrl: import.meta.env.VITE_API_URL,
   }
}
