# 📦 macOS 打包指南

## 🎉 打包成功！

你的 YouTube Downloader 应用已成功打包为 macOS 可发布包。

## 📁 生成的文件

### 1. DMG 安装包 (推荐分发格式)
```
src-tauri/target/release/bundle/dmg/YouTube Downloader_1.0.0_aarch64.dmg
```
- **文件大小**: ~38MB
- **适用架构**: Apple Silicon (M1/M2/M3)
- **用途**: 用户可直接下载安装的 DMG 文件

### 2. .app 应用包
```
src-tauri/target/release/bundle/macos/YouTube Downloader.app
```
- **用途**: 可直接运行的 macOS 应用
- **分发方式**: 可压缩后分发

## 🚀 分发选项

### 选项 1: DMG 文件分发 (推荐)
- ✅ 用户体验最佳
- ✅ 包含安装界面
- ✅ 自动处理权限
- ✅ 符合 macOS 分发标准

### 选项 2: .app 文件分发
- 需要压缩为 ZIP 文件
- 用户需要手动拖拽到 Applications 文件夹

## 🔐 代码签名 (可选)

当前应用未进行代码签名，用户首次运行时需要：

1. 右键点击应用
2. 选择"打开"
3. 确认"打开"

### 添加代码签名 (推荐)

如果你有 Apple Developer 账户，可以添加代码签名：

```json
// tauri.conf.json
"macOS": {
  "signingIdentity": "Developer ID Application: Your Name (TEAM_ID)",
  "providerShortName": "TEAM_ID"
}
```

## 📋 打包命令总结

```bash
# 完整打包流程
cd yt-dlp-desktop

# 1. 安装依赖
npm install

# 2. 构建前端
npm run build

# 3. 打包应用
npm run tauri build
```

## 🎯 应用信息

- **应用名称**: YouTube Downloader
- **版本**: 1.0.0
- **架构**: Apple Silicon (aarch64)
- **最低系统要求**: macOS 10.13+
- **包含组件**: yt-dlp 二进制文件

## 📤 发布建议

1. **GitHub Releases**: 上传 DMG 文件到 GitHub Releases
2. **版本命名**: 使用语义化版本 (v1.0.0)
3. **发布说明**: 包含功能列表和使用说明
4. **校验和**: 提供 SHA256 校验和确保文件完整性

```bash
# 生成校验和
shasum -a 256 "YouTube Downloader_1.0.0_aarch64.dmg"
```

## 🔧 故障排除

### 应用无法打开
- 确保用户右键点击选择"打开"
- 检查 Gatekeeper 设置

### yt-dlp 无法执行
- 确保 yt-dlp 二进制文件有执行权限
- 检查 yt-dlp 版本兼容性

## 🎊 恭喜！

你的 YouTube Downloader 应用现在可以分发给 macOS 用户了！