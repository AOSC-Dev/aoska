import type { Index } from '../types/home';
import type { PackageBrief } from '../types/packages';

export function normalizeSearchQuery(query: unknown): string {
  return String(query ?? '').trim().toLocaleLowerCase();
}

export function flattenCatalogPackages(index: Index): PackageBrief[] {
  const packages = new Map<string, PackageBrief>();
  for (const category of index.packages) {
    for (const pkg of category.packages) {
      if (!packages.has(pkg.name)) {
        packages.set(pkg.name, pkg);
      }
    }
  }
  return [...packages.values()];
}

export function searchCatalogPackages(index: Index, query: string): PackageBrief[] {
  const packages = flattenCatalogPackages(index);
  const normalizedQuery = normalizeSearchQuery(query);
  if (normalizedQuery.length === 0) return packages;

  return packages.filter((pkg) => {
    const fields = [pkg.name, pkg.intro];
    return fields.some((field) => field.toLocaleLowerCase().includes(normalizedQuery));
  });
}
