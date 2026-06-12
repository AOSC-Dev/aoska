import wechatIcon from '../assets/icons/wechat.png';
import wechatBanner from '../assets/icons/wechat_banner.png';
import firefoxShot from '../assets/images/1.png';
import steamShot from '../assets/images/2.png';
import fallbackShot from '../assets/images/3.png';
import { Category, type PackageBrief, type PackageDetail } from '../types/packages';
import type { CategoryIndex, Index, RecommendIndex } from '../types/home';

const packageDetails: PackageDetail[] = [
  {
    name: 'wechat',
    icon: wechatIcon,
    title: '微信',
    intro: '用户超十亿的即时聊天软件',
    category: Category.Working,
    screenshot: [fallbackShot],
    package_flags: {
      unoffical: true,
      verified: true,
      non_native: false,
      windows_app: false,
      telemetry: true,
      service_limited: true,
    },
    package_info: {
      publisher: '腾讯控股有限公司',
      source: '官方安装包',
      version: '4.0.1.11',
      inner_version: 1,
      update_date: '2026-06-09',
      install_size: 711_190_000,
      homepage: 'https://wx.qq.com',
    },
    banner: wechatBanner,
  },
  {
    name: 'firefox',
    icon: wechatIcon,
    title: 'Mozilla Firefox',
    intro: '快速、安全的网页浏览器',
    category: Category.Working,
    screenshot: [firefoxShot],
    package_flags: {
      unoffical: false,
      verified: true,
      non_native: false,
      windows_app: false,
      telemetry: false,
      service_limited: false,
    },
    package_info: {
      publisher: 'Mozilla Foundation',
      source: '安同 OS 软件仓库',
      version: '131.0.3',
      inner_version: 1,
      update_date: '2026-06-01',
      install_size: 85_000_000,
      homepage: 'https://www.mozilla.org/firefox/',
    },
    banner: firefoxShot,
  },
  {
    name: 'steam',
    icon: wechatIcon,
    title: 'Steam',
    intro: '数字游戏分发平台',
    category: Category.Games,
    screenshot: [steamShot],
    package_flags: {
      unoffical: false,
      verified: true,
      non_native: true,
      windows_app: false,
      telemetry: true,
      service_limited: false,
    },
    package_info: {
      publisher: 'Valve Corporation',
      source: '安同 OS 软件仓库',
      version: '1.0.0.78',
      inner_version: 1,
      update_date: '2026-05-30',
      install_size: 120_000_000,
      homepage: 'https://store.steampowered.com/',
    },
    banner: steamShot,
  },
  {
    name: 'vlc',
    icon: wechatIcon,
    title: 'VLC Media Player',
    intro: '自由、开源的跨平台多媒体播放器',
    category: Category.Video,
    screenshot: [fallbackShot],
    package_flags: {
      unoffical: false,
      verified: true,
      non_native: false,
      windows_app: false,
      telemetry: false,
      service_limited: false,
    },
    package_info: {
      publisher: 'VideoLAN',
      source: '安同 OS 软件仓库',
      version: '3.0.20',
      inner_version: 1,
      update_date: '2026-04-10',
      install_size: 95_000_000,
      homepage: 'https://www.videolan.org/vlc/',
    },
    banner: fallbackShot,
  },
];

function brief(pkg: PackageDetail): PackageBrief {
  return {
    name: pkg.name,
    intro: pkg.intro,
    icon: pkg.icon,
  };
}

export function stubRecommend(): RecommendIndex {
  return {
    date: '2026-06-12T00:00:00.000Z',
    packages: packageDetails.slice(0, 4).map(brief),
  };
}

export function stubIndex(): Index {
  const categories = Object.values(Category).map((category) => ({
    category,
    packages: packageDetails.filter((pkg) => pkg.category === category).map(brief),
  }));
  return {
    version: 1,
    generated_at: '2026-06-12T00:00:00.000Z',
    packages: categories,
  };
}

export function stubByCategory(category: string): CategoryIndex {
  const index = stubIndex();
  const found = index.packages.find((item) => item.category === category);
  if (!found) throw new Error(`Category ${category} not found`);
  return found;
}

export function stubDetail(name: string): PackageDetail {
  const found = packageDetails.find((pkg) => pkg.name === name);
  if (!found) throw new Error(`Package ${name} not found`);
  return structuredClone(found);
}
