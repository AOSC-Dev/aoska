import { useConfigStore } from "../stores/cofig";

type AssetInput = string | string[];
type AssetOutput<T extends AssetInput> = T extends string[] ? string[] : string;

export function resolveAssetPath<T extends AssetInput>(path: T, packageName?: string): AssetOutput<T> {
  const store = useConfigStore();
  const base = store.endpoint || '';

  const isAbsolute = (value: string) => /^(https?:|data:|blob:|\/)/.test(value);

  if (Array.isArray(path)) {
    if (packageName) {
      return path.map(p => isAbsolute(p) ? p : new URL(`packages/${packageName}/${p}`, base).toString()) as AssetOutput<T>;
    } else {
      return path.map(p => isAbsolute(p) ? p : new URL(`${p}`, base).toString()) as AssetOutput<T>;
    }
  } else {
    let newPath = path as string;
    if (path == "") return "" as AssetOutput<T>;
    if (isAbsolute(newPath)) return newPath as AssetOutput<T>;
    if (packageName) newPath = `packages/${packageName}/${path}`;
    return new URL(newPath, base).toString() as AssetOutput<T>;
  }
}
