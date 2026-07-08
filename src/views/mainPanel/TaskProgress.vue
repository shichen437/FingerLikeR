<template>
    <Card class="w-full mt-4">
        <CardHeader>
            <CardDescription>
                {{ description }}
            </CardDescription>
        </CardHeader>
        <CardContent>
            <Progress
                :model-value="(taskState.progress / taskState.total) * 100"
            />
            <Separator class="my-4" />
            <div class="text-xs text-muted-foreground">
                你可以使用 <Kbd>⌘</Kbd>+<Kbd>⇧</Kbd>+<Kbd>J</Kbd>
                全局快捷键随时取消任务。
            </div>
        </CardContent>
    </Card>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { taskState } from "@/store/task";
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
} from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { Kbd } from "@/components/ui/kbd";
import { Progress } from "@/components/ui/progress";

const description = computed(() => {
    switch (taskState.status) {
        case "Countdown":
            return `将在 ${taskState.progress} 秒后开始执行任务`;
        case "Running":
            return `正在执行第 ${taskState.progress} / ${taskState.total} 次点击`;
        case "Cancelled":
            return "任务已取消。";
        case "Finished":
            return "任务已完成";
        default:
            return "";
    }
});
</script>
