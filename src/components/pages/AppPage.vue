<template>
  <AppHeader
    app-name="微信"
    app-intro="用户超十亿的即时聊天软件"
    app-icon="/src/assets/icons/wechat.png"
    app-banner-img="/src/assets/icons/wechat_banner.png"
  />
  <div class="down-content">
    <div class="image-container">
      <ImageCarousel 
        :images="screenshots" 
        :autoplay="false" 
        :interval="4000"
      />
    </div>
    <div class="down-right">
      <div class="flags">
        <SoftwareFlags type="unofficial"></SoftwareFlags>
        <SoftwareFlags type="verified"></SoftwareFlags>
        <!-- <SoftwareFlags type="serviceRestricted"></SoftwareFlags> -->
        <!-- <SoftwareFlags type="nonNative"></SoftwareFlags> -->
        <!-- <SoftwareFlags type="windows"></SoftwareFlags> -->
        <!-- <SoftwareFlags type="telemetry"></SoftwareFlags> -->
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
        <!-- 这两个按钮比较特殊，先用div展示效果 -->
        <div class="main-page">发行方主页</div>
        <div class="report-bug">报告使用问题</div>
        <!-- <Button
          size="large"
          color="grey"
          width="261"
          height="37"
        >
          发行方主页
        </Button>
        <Button
          size="large"
          color="yellow"
          width="261"
          height="37"
        >
          报告使用问题
        </Button> -->
      </div>
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

.image-container {
  margin: 10px 10px 20px 20px;
  width: calc(100% - 2px);
  height: calc(100vh - 292px);
  border: 1px solid #dadada;
  border-radius: 5px;
}

.image-container img {
  width: auto;
  height: 100%;
  margin-right: 10px;
}

.down-right {
  width: 520px;
  height: calc(100vh - 292px);
  margin: 10px 20px 20px 10px;
  display: flex;
  flex-direction: column;
}

.flags{
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.info {
  width: calc(100% - 20px);
  margin-top: auto;
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
  margin-top: 10px;
}

.main-page {
  width: 250px;
  height: 36px;
  font-size: 18px;
  border-radius: 5px;
  line-height: 36px;
  background-color: #f3f3f3;
  border: 1px solid black;
  text-align: center;
}

.report-bug {
  width: 250px;
  height: 36px;
  margin-left: auto;
  font-size: 18px;
  border-radius: 5px;
  line-height: 36px;
  background-color: rgb(250, 216, 185);
  border: 1px solid black;
  text-align: center;
}

</style>