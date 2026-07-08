<script setup lang="ts">
import {
    Card,
    CardContent,
    CardHeader,
    CardDescription,
} from "@/components/ui/card";
import { ExternalLink } from "@lucide/vue";
import { ref } from "vue";

const appVersion = ref("0.1.0");
const projectGithubUrl = "https://github.com/shichen437/FingerLikeR";
const authorEmail = "shichen437@126.com";
</script>

<template>
    <div class="container mx-auto p-4 max-w-2xl">
        <h1 class="text-3xl font-bold text-center mb-8">FingerLike R</h1>

        <Card class="mb-6">
            <CardHeader>
                <CardDescription>
                    本项目基于 Tauri + Vue 开发，永久免费开源。
                </CardDescription>
            </CardHeader>
        </Card>

        <Card>
            <CardContent>
                <div
                    class="grid grid-cols-[max-content_1fr_max-content_1fr] gap-x-4 gap-y-2 items-center"
                >
                    <span>当前版本:</span>
                    <span>{{ appVersion }}</span>

                    <span>最新版本:</span>
                    <span>{{ appVersion }}</span>
                    <span>源码地址:</span>
                    <a
                        :href="projectGithubUrl"
                        target="_blank"
                        class="hover:underline flex items-center"
                    >
                        点此访问
                        <ExternalLink class="w-4 h-4 ml-1" />
                    </a>

                    <span>联系作者:</span>
                    <a class="hover:underline flex items-center">
                        {{ authorEmail }}
                    </a>
                </div>
            </CardContent>
        </Card>
    </div>
</template>
