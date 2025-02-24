export interface Deserializable<T> {
  deserialize(input: T): T;
}
