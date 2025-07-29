export enum Category {
  Working = "working",
  Games = "games",
  Video = "video",
  Creating = "creating",
}

export interface PackageBrief {
  name: string;
  intro: string;
  icon: string; // PathBuf
}

export interface PackageFlags {
  unoffical: boolean;
  verified: boolean;
  non_native: boolean;
  windows_app: boolean;
  telemetry: boolean;
  service_limited: boolean;
}

export interface PackageInfo {
  publisher: string;
  source: string;
  version: string;
  inner_version: number; // i32
  update_date: string;
  install_size: number; // i64
  homepage: string;
}

/**
 * The detailed information of a package.
 * Will be serialized into repo-root/packages/$package_name/meta.json
 */
export interface PackageDetail {
  name: string;
  icon: string;
  title: string;
  intro: string;
  category: Category;
  screenshot: string[]; // Vec<PathBuf>
  package_flags: PackageFlags;
  package_info: PackageInfo;
  banner: string; // PathBuf
}
