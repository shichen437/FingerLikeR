<template>
    <div class="p-4 grid gap-4">
        <RadioGroup v-model="clickMode" class="grid grid-cols-3 gap-4">
            <Label
                for="bionic"
                class="border rounded-md p-4 flex flex-row items-center justify-center gap-2 cursor-pointer transition-colors"
                :class="[
                    'hover:border-primary',
                    clickMode === '1' ? 'border-primary' : '',
                ]"
            >
                <RadioGroupItem value="1" id="bionic" class="sr-only" />
                <HandHeart class="w-8 h-8" />
                <span class="font-semibold">仿生点赞</span>
            </Label>
            <Label
                for="normal"
                class="border rounded-md p-4 flex flex-row items-center justify-center gap-2 cursor-pointer transition-colors"
                :class="[
                    'hover:border-primary',
                    clickMode === '2' ? 'border-primary' : '',
                ]"
            >
                <RadioGroupItem value="2" id="normal" class="sr-only" />
                <MousePointerClick class="w-8 h-8" />
                <span class="font-semibold">普通点击</span>
            </Label>
            <Label
                for="custom"
                class="border rounded-md p-4 flex flex-row items-center justify-center gap-2 cursor-pointer transition-colors"
                :class="[
                    'hover:border-primary',
                    clickMode === '3' ? 'border-primary' : '',
                ]"
            >
                <RadioGroupItem value="3" id="custom" class="sr-only" />
                <Settings class="w-8 h-8" />
                <span class="font-semibold">自定义</span>
            </Label>
        </RadioGroup>
        <Card v-if="clickMode === '3'">
            <CardContent>
                <div class="grid gap-6">
                    <div class="grid gap-2">
                        <div class="flex items-center justify-between">
                            <Label for="base-interval-slider"
                                >基础间隔 ({{
                                    customParams.baseInterval[0]
                                }}ms)</Label
                            >
                        </div>
                        <Slider
                            id="base-interval-slider"
                            v-model="customParams.baseInterval"
                            :min="20"
                            :max="500"
                            :step="10"
                        />
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="max-interval-slider"
                                    >最大间隔 ({{
                                        customParams.maxInterval[0]
                                    }}ms)</Label
                                >
                            </div>
                            <Slider
                                id="max-interval-slider"
                                v-model="customParams.maxInterval"
                                :min="0"
                                :max="1000"
                                :step="50"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="step-slider"
                                    >步长 ({{ customParams.step[0] }})</Label
                                >
                            </div>
                            <Slider
                                id="step-slider"
                                v-model="customParams.step"
                                :min="0"
                                :max="1500"
                                :step="50"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="step-interval-slider"
                                    >步长间隔 ({{
                                        customParams.stepInterval[0]
                                    }}ms)</Label
                                >
                            </div>
                            <Slider
                                id="step-interval-slider"
                                v-model="customParams.stepInterval"
                                :min="0"
                                :max="100"
                                :step="5"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="random-interval-slider"
                                    >随机间隔 ({{
                                        customParams.randomInterval[0]
                                    }}ms)</Label
                                >
                            </div>
                            <Slider
                                id="random-interval-slider"
                                v-model="customParams.randomInterval"
                                :min="0"
                                :max="100"
                                :step="5"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="x-offset-slider"
                                    >X坐标随机偏移 ({{
                                        customParams.xOffset[0]
                                    }}px)</Label
                                >
                            </div>
                            <Slider
                                id="x-offset-slider"
                                v-model="customParams.xOffset"
                                :min="0"
                                :max="50"
                                :step="1"
                            />
                        </div>
                        <div class="grid gap-2">
                            <div class="flex items-center justify-between">
                                <Label for="y-offset-slider"
                                    >Y坐标随机偏移 ({{
                                        customParams.yOffset[0]
                                    }}px)</Label
                                >
                            </div>
                            <Slider
                                id="y-offset-slider"
                                v-model="customParams.yOffset"
                                :min="0"
                                :max="50"
                                :step="1"
                            />
                        </div>
                    </div>
                </div>
            </CardContent>
        </Card>
        <Card>
            <CardContent>
                <div class="grid gap-4">
                    <div class="flex items-center justify-between">
                        <Label for="countdown-slider"
                            >任务开始倒计时 ({{ countdown[0] }}秒)</Label
                        >
                    </div>
                    <Slider
                        id="countdown-slider"
                        v-model="countdown"
                        :min="3"
                        :max="30"
                        :step="1"
                    />
                </div>
            </CardContent>
        </Card>
        <Card>
            <CardContent>
                <div class="flex items-center justify-between">
                    <div class="flex flex-col">
                        <div class="flex items-center gap-2">
                            <Label>闲动模式</Label>
                            <span class="text-xs text-muted-foreground">
                                防网页休眠检测
                            </span>
                        </div>
                    </div>
                    <Switch :checked="idleMove" v-model="idleMove" />
                </div>
            </CardContent>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch, reactive } from "vue";
import { getStoreValue, setStoreValue } from "@/store/store";
import { Card, CardContent } from "@/components/ui/card";
import { Slider } from "@/components/ui/slider";
import { Label } from "@/components/ui/label";
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Switch } from "@/components/ui/switch";
import { HandHeart, MousePointerClick, Settings } from "@lucide/vue";
import { invoke } from "@tauri-apps/api/core";

const countdown = ref([7]);
const clickMode = ref("1");
const idleMove = ref(false);

const customParams = reactive({
    baseInterval: [150],
    maxInterval: [500],
    step: [200],
    stepInterval: [20],
    randomInterval: [25],
    xOffset: [20],
    yOffset: [20],
});

onMounted(async () => {
    const storedCountdown = await getStoreValue("task.countdown", 7);
    countdown.value = [storedCountdown];

    const storedClickMode = await getStoreValue("task.clickMode", 1);
    clickMode.value = String(storedClickMode);

    const storedIdleMove = await getStoreValue("task.idleMove", 0);
    idleMove.value = storedIdleMove === 1;

    const params = await getStoreValue("task.custom.taskParams", {
        baseInterval: 150,
        maxInterval: 500,
        step: 200,
        stepInterval: 20,
        randomInterval: 25,
        xOffset: 20,
        yOffset: 20,
    });
    customParams.baseInterval = [params.baseInterval];
    customParams.maxInterval = [params.maxInterval];
    customParams.step = [params.step];
    customParams.stepInterval = [params.stepInterval];
    customParams.randomInterval = [params.randomInterval];
    customParams.xOffset = [params.xOffset];
    customParams.yOffset = [params.yOffset];
});

watch(countdown, (newValue) => {
    if (newValue.length > 0) {
        setStoreValue("task.countdown", newValue[0]);
    }
});

watch(clickMode, (newValue) => {
    setStoreValue("task.clickMode", parseInt(newValue, 10));
});

watch(customParams, (newValue) => {
    setStoreValue("task.custom.taskParams", {
        baseInterval: newValue.baseInterval[0],
        maxInterval: newValue.maxInterval[0],
        step: newValue.step[0],
        stepInterval: newValue.stepInterval[0],
        randomInterval: newValue.randomInterval[0],
        xOffset: newValue.xOffset[0],
        yOffset: newValue.yOffset[0],
    });
});

watch(idleMove, async (value) => {
    await setStoreValue("task.idleMove", value ? 1 : 0);
    await invoke("toggle_idle_move_job");
});
</script>
