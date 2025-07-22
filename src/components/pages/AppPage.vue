<template>
  <AppHeader
    :app-name="packageDetail?.title"
    :app-intro="packageDetail?.intro"
    :app-icon="resolveAssetPath(packageDetail?.icon || '', packageName)"
    :app-banner-img="resolveAssetPath(packageDetail?.banner || '', packageName)"
  />
  <div class="down-content">
    <div class="image-container">
      <ImageCarousel 
        :images="resolveAssetPath(packageDetail?.screenshot || [], packageName)" 
        :autoplay="false" 
        :interval="4000"
      />
    </div>
    <div class="down-right">
      <div class="flags">
        <template
          v-for="(v, k) in packageDetail?.package_flags"
          :key="k"
        >
          <SoftwareFlags 
            v-if="v"
            :type="k" />
        </template>
      </div>
      <div class="info">
        <table>
          <tbody>
            <tr>
              <td class="l1">发行方：</td>
              <td class="l2">{{ packageDetail?.package_info.publisher }}</td>
              <td class="l3">软件版本：</td>
              <td class="l4">{{ packageDetail?.package_info.version }}</td>
            </tr>
            <tr>
              <td></td>
              <td></td>
              <td class="l3">更新日期：</td>
              <td>{{ packageDetail?.package_info.update_date }}</td>
            </tr>
            <tr>
              <td class="l1">来源：</td>
              <td>{{ packageDetail?.package_info.source }}</td>
              <td class="l3">安装大小：</td>
              <td>{{ installSizeFormatted }}</td>
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

<script setup lang="ts">
import { filesize } from 'filesize';
import SoftwareFlags from "../share/SoftwareFlags.vue";
import ImageCarousel from "../share/ImageCarousel.vue";
import { computed, onMounted, ref } from 'vue';
import AppHeader from "./AppPage/AppHeader.vue";
import { useRoute} from "vue-router";
import { fetchDetail } from "../../utils/wrapper";
import { PackageDetail } from "../../types/packages";
import { resolveAssetPath } from '../../utils/url';

const route = useRoute();
const packageName = ref<string>(route.params.pkgName as string);
const packageDetail = ref<PackageDetail | null>(null);

onMounted(async () => {
  packageDetail.value = await fetchDetail(packageName.value);
});

const installSizeFormatted = computed(() => {
  return filesize(packageDetail.value?.package_info?.install_size ?? 0);
});

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