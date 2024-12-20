<script setup lang="ts">
import { Notivue, NotivueSwipe, Notification } from 'notivue'
</script>

<template>
  <Notivue v-slot="item">
    <NotivueSwipe :item="item">
      <Notification :item="item" />
    </NotivueSwipe>
  </Notivue>
</template>
