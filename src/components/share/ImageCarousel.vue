<template>
  <div class="image-carousel" @mouseenter="handleMouseEnter" @mouseleave="handleMouseLeave">
    <!-- 主图显示区域 -->
    <div class="main-image-container">
      <button class="nav-button prev" :class="{ visible: isHovering }" @click="prev">
        &lt;
      </button>
      
      <div class="main-image">
        <div class="image-wrapper">
          <transition-group name="fade" tag="div" class="image-transition-container">
            <img 
              v-for="(image, index) in images" 
              v-show="currentIndex === index"
              :key="index"
              :src="image"
            />
          </transition-group>
        </div>
      </div>
      
      <button class="nav-button next" :class="{ visible: isHovering }" @click="next">
        &gt;
      </button>
      
      <div v-if="isHovering" class="indicators">
        <div 
          v-for="(_, index) in images" 
          :key="index" 
          class="indicator" 
          :class="{ active: index === currentIndex }"
          @click="showImage(index)"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, watch } from 'vue';

const props = defineProps<{
  images: string[];
  autoplay?: boolean;
  interval?: number;
}>();

const currentIndex = ref(0);
const isHovering = ref(false);
let timer: number | null = null;

const showImage = (index: number) => {
  currentIndex.value = index;
};

const prev = () => {
  currentIndex.value = (currentIndex.value - 1 + props.images.length) % props.images.length;
};

const next = () => {
  currentIndex.value = (currentIndex.value + 1) % props.images.length;
};

const handleMouseEnter = () => {
  isHovering.value = true;
  if (props.autoplay) {
    stopAutoplay();
  }
};

const handleMouseLeave = () => {
  isHovering.value = false;
  if (props.autoplay) {
    startAutoplay();
  }
};

const startAutoplay = () => {
  if (timer) return;
  timer = window.setInterval(() => {
    next();
  }, props.interval || 3000);
};

const stopAutoplay = () => {
  if (timer) {
    clearInterval(timer);
    timer = null;
  }
};

onMounted(() => {
  if (props.autoplay) {
    startAutoplay();
  }
});

onBeforeUnmount(() => {
  stopAutoplay();
});

watch(() => props.images, () => {
  currentIndex.value = 0;
}, { deep: true });
</script>

<style scoped>
.image-carousel {
  position: relative;
  width: 100%;
  height: 100%;
}

.main-image-container {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  overflow: hidden;
  border-radius: 5px;
}

.main-image {
  flex: 1;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  overflow: hidden;
  position: relative;
}

.image-wrapper {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.image-transition-container {
  position: relative;
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}

.image-transition-container img {
  position: absolute;
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  margin: auto;
}

.nav-button {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  width: 40px;
  height: 40px;
  background-color: rgba(0, 0, 0, 0.5);
  color: white;
  border: none;
  border-radius: 50%;
  font-size: 20px;
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.3s ease;
  z-index: 2;
}

.nav-button.visible,
.nav-button:hover {
  opacity: 0.8;
}

.prev {
  left: 10px;
}

.next {
  right: 10px;
}

.indicators {
  position: absolute;
  bottom: 15px;
  left: 0;
  right: 0;
  display: flex;
  justify-content: center;
  gap: 8px;
  z-index: 2;
}

.indicator {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background-color: rgba(255, 255, 255, 0.5);
  cursor: pointer;
  transition: all 0.3s ease;
}

.indicator.active {
  background-color: white;
  transform: scale(1.2);
}

.indicator:hover {
  background-color: rgba(255, 255, 255, 0.8);
}

.fade-enter-active {
  transition: all 0.3s ease;
  z-index: 1;
}

.fade-leave-active {
  transition: all 0.3s ease;
  position: absolute;
  z-index: 0;
}

.fade-enter-from {
  opacity: 0;
  transform: scale(1.05);
}

.fade-leave-to {
  opacity: 0;
  transform: scale(0.95);
}
</style>
