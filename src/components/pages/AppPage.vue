<template>
  <AppHeader
    app-name="微信"
    app-intro="用户超十亿的即时聊天软件"
    app-icon="/src/assets/icons/wechat.png"
    app-banner-img="/src/assets/icons/wechat_banner.png"
  />
  <div class="down-content">
    <div class="down-left">
      <div class="screen-shot">
        <div class="image-container">
          <ImageCarousel 
            :images="screenshots" 
            :autoplay="false" 
            :interval="4000"
          />
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
      </div>
      <div class="btns">
        <Button
          size="large"
          color="grey"
          width="261"
          height="37"
          :style="{ marginLeft: '15px' }"
        >
          发行方主页
        </Button>
        <Button
          size="large"
          color="yellow"
          width="261"
          height="37"
          :style="{ marginLeft: '15px' }"
        >
          报告使用问题
        </Button>
      </div>
    </div>
    <div class="down-right">
      <SoftwareFlags type="unofficial"></SoftwareFlags>
      <SoftwareFlags type="verified"></SoftwareFlags>
      <SoftwareFlags type="serviceRestricted"></SoftwareFlags>
      <SoftwareFlags type="nonNative"></SoftwareFlags>
      <SoftwareFlags type="windows"></SoftwareFlags>
      <SoftwareFlags type="telemetry"></SoftwareFlags>
    </div>
  </div>
</template>

<script setup>
import SoftwareFlags from "../share/SoftwareFlags.vue";
import ImageCarousel from "../share/ImageCarousel.vue";
import { ref } from 'vue';
import AppHeader from "../AppPage/AppHeader.vue";
import Button from '../share/Button.vue';

const screenshots = ref([
  '/src/assets/images/3.png',
  '/src/assets/images/2.png',
  '/src/assets/images/1.png'
]);
</script>

<style scoped>
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
  overflow-x: hidden;
  overflow-y: hidden;
  width: calc(100% - 2px);
  height: 400px;
  border: 1px solid #dadada;
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
  background-color: #f3f3f3;
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
  margin-top: 20px;
  justify-content: space-between;
}

.main-page {
  width: 200px;
  height: 36px;
  font-size: 18px;
  margin-top: 20px;
  border-radius: 5px;
  line-height: 36px;
  background-color: #f3f3f3;
  border: 1px solid black;
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
  background-color: rgb(250, 216, 185);
  border: 1px solid black;
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