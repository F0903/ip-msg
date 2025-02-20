export interface Deserializable<T> {
  deserialize(input: Object): T;
}
