export interface BufferResult<T> {
  (arg: T): void;
  flush: () => void;
}

export function buffer<T>(callback: (arg: T) => void) {
  const queue: T[] = [];
  let flushed = false;

  const result = ((arg: T) => {
    if (flushed) {
      callback(arg);
    } else {
      queue.push(arg);
    }
  }) as BufferResult<T>;

  result.flush = () => {
    flushed = true;
    let next: T | undefined;
    while ((next = queue.shift())) {
      callback(next);
    }
  };

  return result;
}
