import type { PackageBrief, Category } from "./packages";

export interface CategoryIndex {
  category: Category;
  packages: PackageBrief[];
}

export interface RecommendIndex {
  date: string; // DateTime<Utc>
  packages: PackageBrief[];
}

export interface Index {
  version: number; // u8
  generated_at: string; // DateTime<Utc>
  packages: CategoryIndex[];
}
