<script setup lang="ts">
import { Notification, Notivue, NotivueSwipe } from 'notivue'
</script>

<template>
  <Notivue v-slot="item">
    <NotivueSwipe :item="item">
      <Notification :item="item" />
    </NotivueSwipe>
  </Notivue>
</template>
