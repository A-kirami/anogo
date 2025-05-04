<script setup lang="ts">
import { type as getOsType } from '@tauri-apps/plugin-os'

const osType = getOsType()
</script>

<template>
  <header
    data-tauri-drag-region
    class="h-7 flex justify-end bg-background text-gray-500"
  >
    <WindowControls v-if="osType !== 'macos'" />
  </header>
</template>
