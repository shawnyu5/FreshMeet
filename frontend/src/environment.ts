interface Environment {
  api_url: string;
}

/**
 * Load custom environment variables
 */
export function load(): Environment {
  return {
    api_url: import.meta.env.VITE_API_URL,
  };
}
