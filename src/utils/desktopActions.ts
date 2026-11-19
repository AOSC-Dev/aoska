export function externalPageUrl(value: string): string {
  const url = new URL(value);
  if (!['https:', 'http:'].includes(url.protocol)) throw new Error('仅支持 HTTP 或 HTTPS 链接。');
  return url.href;
}

export async function launchDesktopApplication(
  catalogName: string,
  preview: boolean,
  invokeCommand: (command: string, args: { catalogName: string; packageName?: string }) => Promise<unknown>,
  packageName?: string,
): Promise<void> {
  if (preview) throw new Error('浏览器预览无法启动本机应用，请在桌面版中打开。');
  await invokeCommand('app_launch', packageName ? { catalogName, packageName } : { catalogName });
}
