<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

import { github } from '~build/git'

const version = getVersion()
</script>

<template>
  <div class="mx-auto -my-1">
    <div
      class="inline-flex items-center gap-1 text-xs text-muted-foreground"
      :class="{'cursor-pointer': github}"
      @click="github && openUrl(github)"
    >
      <span i="simple-icons-github" />
      <span>AnoGO v{{ version.name }}</span>
    </div>
  </div>
</template>
