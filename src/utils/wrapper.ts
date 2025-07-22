import { invoke } from "@tauri-apps/api/core";
import { OmaOperation, TumUpdateInfo } from '../types/oma';
import { CategoryIndex, Index, RecommendIndex } from '../types/home';
import { PackageDetail } from '../types/packages';

export async function fetchUpdateDetail(): Promise<OmaOperation> {
  return invoke<OmaOperation>('fetch_update_detail');
}

export async function fetchUpdateCount(): Promise<number> {
  return invoke<number>('fetch_update_count');
}

export async function fetchTumUpdate(): Promise<TumUpdateInfo[]> {
  return invoke<TumUpdateInfo[]>('fetch_tum_update');
}

export async function fetchRecommend(): Promise<RecommendIndex> {
  return invoke<RecommendIndex>('fetch_recommend');
}

export async function fetchIndex(): Promise<Index> {
  return invoke<Index>('fetch_index');
}

export async function fetchDetail(pkgName: string): Promise<PackageDetail> {
  return invoke<PackageDetail>('fetch_detail', { pkgName });
}

export async function fetchByCategory(category: string): Promise<CategoryIndex> {
  return invoke<CategoryIndex>('fetch_by_category', { category });
}

export async function getEndpointBaseUrl(): Promise<string> {
  return invoke<string>('get_endpoint_base_url');
}
