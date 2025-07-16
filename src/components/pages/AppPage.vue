<template>
  <div class="header">
    <img src="../../assets/icons/aosc_logo.png" alt="安同社区的logo">
    <div class="app-intro">
      <div class="name">微信</div>
      <div class="intro">用户超十亿的即时聊天软件</div>
    </div>
    <div class="install">安装</div>
  </div>
  <div class="down-content">
    <div class="down-left">
      <div class="screen-shot">
        <div class="image-container">
          <img src="../../assets/images/3.png" alt="截图1">
          <img src="../../assets/images/2.png" alt="截图2">
          <img src="../../assets/images/1.png" alt="截图3">
        </div>
      </div>
      <div class="info">
        <table>
          <tbody>
            <tr>
              <td class="l1">发行方：</td>
              <td class="l2">腾讯控股有限公司</td>
              <td class="l3">软件版本：</td>
              <td class="l4">4.0.0.11</td>
            </tr>
            <tr>
              <td></td>
              <td></td>
              <td class="l3">更新日期：</td>
              <td>2024年11月23日</td>
            </tr>
            <tr>
              <td class="l1">来源：</td>
              <td>官方安装包</td>
              <td class="l3">安装大小：</td>
              <td>711.19 MiB</td>
            </tr>
          </tbody>
        </table>
        <div class="btns">
          <div class="main-page">发行方主页</div>
          <div class="report-bug">报告使用问题</div>
        </div>
      </div>
    </div>
    <div class="down-right">
      <SoftwareFlags type="unofficial"></SoftwareFlags>
      <SoftwareFlags type="verified"></SoftwareFlags>
    </div>
  </div>
</template>

<script setup>

import SoftwareFlags from "../share/SoftwareFlags.vue";
</script>

<style scoped>
.header {
  display: flex;
  width: calc(100% - 40px);
  height: 100px;
  border-radius: 5px;
  margin:65px 20px 10px 20px;
  background-color: lightgray;
}

.header img {
  width: 60px;
  height: 60px;
  margin: 20px;
}

.app-intro {
  margin-top: 15px;
}

.name {
  font-size: 40px;
}

.intro {
  font-size: 18px;
}

.install {
  width: 120px;
  height: 36px;
  font-size: 24px;
  line-height: 36px;
  text-align: center;
  margin: auto 10px 10px auto;
  border: 1px solid black;
  border-radius: 5px;
  background-color: lightgreen;
}

.down-content {
  display: flex;
}

.down-left {
  width: calc(100% - 560px);
  margin: 10px 10px 20px 20px;
}

.screen-shot {
  width: 100%;
}

.image-container {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  overflow-x: auto;
  overflow-y: hidden;
  width: calc(100% - 2px);
  height: 300px;
  border: 1px solid black;
  border-radius: 5px;
}

.image-container img {
  width: auto;
  height: 100%;
  margin-right: 10px;
}

.info {
  width: calc(100% - 20px);
  margin-top: 20px;
  padding: 10px;
  border-radius: 5px;
  background-color: lightgray;
}

table {
  width: 100%;
}

.l1 {
  width: 65px;
  font-weight: bold;
}

.l3 {
  width: 85px;
  font-weight: bold;
}

.l2 {
  width: calc(50% - 65px);
}

.l4 {
  width: calc(50% - 85px);
}

.btns {
  display: flex;
}

.main-page {
  width: 200px;
  height: 36px;
  font-size: 18px;
  margin-top: 20px;
  border-radius: 5px;
  line-height: 36px;
  background-color: white;
  text-align: center;
}

.report-bug {
  width: 200px;
  height: 36px;
  margin-top: 20px;
  margin-left: calc(50% - 195px);
  font-size: 18px;
  border-radius: 5px;
  line-height: 36px;
  background-color: lightcoral;
  text-align: center;
}

.down-right {
  width: 500px;
  height: 400px;
  margin: 10px 20px 20px 10px;
  display: flex;
  flex-direction: column;
  gap: 10px;
}
</style>