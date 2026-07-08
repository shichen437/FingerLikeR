<template>
    <Tabs v-model="activeTab">
        <div class="flex items-center justify-center gap-6">
            <TabsList>
                <TabsTrigger value="mainPanel"> 主界面 </TabsTrigger>
                <TabsTrigger value="clickMode"> 点击模式 </TabsTrigger>
                <TabsTrigger value="about"> 关于 </TabsTrigger>
            </TabsList>
        </div>
        <div class="h-full rounded-lg p-4">
            <router-view />
        </div>
    </Tabs>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";

const route = useRoute();
const router = useRouter();

const activeTab = computed({
    get: () => (route.name as string | undefined) ?? "mainPanel",
    set: (val) => {
        router.push({ name: val });
    },
});
</script>
