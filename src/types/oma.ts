export interface OmaOperation {
  install: InstallEntry[];
  remove: RemoveEntry[];
  disk_size_delta: number; // i64 in Rust
  autoremovable: [number, number]; // tuple of two u64
  total_download_size: number; // u64
  suggest: [string, string][];
  recommend: [string, string][];
}

export interface InstallEntry {
  name: string;
  name_without_arch: string;
  old_version?: string; // Option<String>
  new_version: string;
  old_size?: number; // Option<u64>
  new_size: number;
  pkg_urls: PackageUrl[];
  sha256?: string;
  md5?: string;
  sha512?: string;
  arch: string;
  download_size: number;
  op: InstallOperation;
  automatic?: boolean; // default: false
  index: number;
}

export interface PackageUrl {
  download_url: string;
  index_url: string;
}

export interface RemoveEntry {
  name: string;
  version?: string; // Option<String>
  size: number;
  details: RemoveTag[];
  arch: string;
  index: number;
}

export enum RemoveTag {
  Purge = "Purge",
  AutoRemove = "AutoRemove",
  Resolver = "Resolver",
}

export enum InstallOperation {
  Default = 0,
  Install = 1,
  ReInstall = 2,
  Upgrade = 3,
  Downgrade = 4,
  Download = 5,
}

export interface TumUpdateInfo {
  manifest_name: string;
  name: Record<string, string>;
  is_security: boolean;
  package_count: number;
  package_names: string[];
  caution?: Record<string, string>;
}
