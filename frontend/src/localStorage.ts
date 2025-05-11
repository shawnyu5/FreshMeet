/**
 * Append to a key in local storage. If the key already exist in the key, it will not be appended
 * @param key - the key to set
 * @param val - the value of the key
 * @returns the value set in local storage
 */
export function appendLocalStorage(key: string, val: string): string {
  const item = localStorage.getItem(key);
  if (item && !item.includes(val)) {
    val = `${item},${val}`;
  }
  localStorage.setItem(key, val);
  return val;
}

/**
 * Get a key in local storage. If the key is comma separated, it will be converted into an array
 * @param key - the key to retrieve
 */
export function getLocalStorage(key: string): Array<string> {
  const val = localStorage.getItem(key);
  if (!val) {
    return [];
  }

  return val.split(",");
}

/**
 * Get the value of local storage, remove the last item and return it
 * @param key - the key to get from local storag
 * @return the last value of the `key` in local storage. If the key is empty, null will be returned
 */
export function popLocalStorage(key: string): string | null {
  const val = localStorage.getItem(key);
  if (!val) {
    return null;
  }

  let arr = val.split(",");
  // The array will always be defined here. So no null checks are needed
  const lastItem = arr.pop() as string;

  let joined = arr.join(",");
  localStorage.setItem(key, joined);
  return lastItem;
}
