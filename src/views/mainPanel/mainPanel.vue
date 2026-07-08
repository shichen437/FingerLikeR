<template>
    <div class="flex flex-col items-center justify-center space-y-4">
        <h1 class="text-2xl font-bold">FingerLike R</h1>
        <div class="w-full max-w-lg mt-6">
            <Card>
                <CardContent class="pt-6">
                    <Form
                        :validation-schema="formSchema"
                        @submit="onSubmit"
                        v-slot="{ setFieldValue }"
                    >
                        <FormField v-slot="{ componentField }" name="amount">
                            <FormItem>
                                <FormLabel>点击次数</FormLabel>
                                <div
                                    class="flex w-full max-w-md items-center gap-2"
                                >
                                    <FormControl>
                                        <Input
                                            type="number"
                                            placeholder="请输入点击次数"
                                            v-bind="componentField"
                                            :disabled="
                                                taskState.status !== 'Idle'
                                            "
                                        />
                                    </FormControl>
                                    <Button
                                        type="submit"
                                        :disabled="taskState.status !== 'Idle'"
                                    >
                                        开始任务
                                    </Button>
                                </div>
                                <FormMessage />
                                <div
                                    class="flex justify-center gap-2 mt-4 items-center"
                                >
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        type="button"
                                        @click="setFieldValue('amount', 500)"
                                        :disabled="taskState.status !== 'Idle'"
                                    >
                                        500
                                    </Button>
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        type="button"
                                        @click="setFieldValue('amount', 1000)"
                                        :disabled="taskState.status !== 'Idle'"
                                    >
                                        1000
                                    </Button>
                                    <Button
                                        variant="outline"
                                        size="sm"
                                        type="button"
                                        @click="setFieldValue('amount', 3000)"
                                        :disabled="taskState.status !== 'Idle'"
                                    >
                                        3000
                                    </Button>
                                </div>
                            </FormItem>
                        </FormField>
                    </Form>
                </CardContent>
            </Card>
            <div
                class="text-center text-sm text-muted-foreground mt-4"
                v-if="taskState.status === 'Countdown'"
            >
                当前鼠标位置: X: {{ mousePosition.x }}, Y: {{ mousePosition.y }}
            </div>
            <TaskProgress v-show="taskState.status !== 'Idle'" />
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { toTypedSchema } from "@vee-validate/zod";
import * as z from "zod";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import {
    Form,
    FormControl,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "@/components/ui/form";
import TaskProgress from "./TaskProgress.vue";
import {
    taskState,
    setTaskState,
    TaskProgress as TaskProgressType,
} from "@/store/task";
import { getStoreValue } from "@/store/store";

const formSchema = toTypedSchema(
    z.object({
        amount: z
            .number({ invalid_type_error: "点击次数必须是数字" })
            .min(1, { message: "点击次数不能小于1" })
            .max(9999999, { message: "点击次数不能大于9999999" }),
    }),
);

const mousePosition = ref({ x: 0, y: 0 });
let unlisten: (() => void) | undefined;
let unlistenCancelReq: (() => void) | undefined;
let pollingInterval: number | undefined;
const countdownInterval = ref<number | undefined>(undefined);

async function updateMousePosition() {
    const [x, y] = (await invoke("get_mouse_location")) as [number, number];
    mousePosition.value = { x, y };
}

watch(
    () => taskState.status,
    (newStatus) => {
        if (newStatus === "Countdown") {
            if (!pollingInterval) {
                pollingInterval = window.setInterval(updateMousePosition, 100);
            }
        } else {
            if (pollingInterval) {
                clearInterval(pollingInterval);
                pollingInterval = undefined;
            }
        }
    },
);

function cancelCountdown() {
    if (countdownInterval.value) {
        clearInterval(countdownInterval.value);
        countdownInterval.value = undefined;
        setTaskState({ status: "Cancelled", progress: 0, total: 0 });
    }
}

async function onSubmit(values: any) {
    const amount = values.amount as number;
    await updateMousePosition();
    let countdownSeconds = await getStoreValue("task.countdown", 5);

    countdownSeconds = Math.round(countdownSeconds * 10) / 10;

    taskState.status = "Countdown";
    taskState.total = countdownSeconds;
    taskState.progress = countdownSeconds;

    const stepMs = 100;
    const stepSeconds = 0.1;

    countdownInterval.value = window.setInterval(() => {
        taskState.progress =
            Math.round((taskState.progress - stepSeconds) * 10) / 10;

        if (taskState.progress <= 0) {
            clearInterval(countdownInterval.value);
            countdownInterval.value = undefined;
            taskState.progress = 0;

            invoke("start_clicking_task", {
                count: amount,
            });
        }
    }, stepMs);
}

onMounted(async () => {
    unlisten = await listen<TaskProgressType>("task-progress", (event) => {
        setTaskState(event.payload);
    });
    unlistenCancelReq = await listen("cancel-task-request", () => {
        if (taskState.status === "Countdown") {
            cancelCountdown();
        }
    });
    const initialState = await invoke<TaskProgressType>("get_task_status");
    setTaskState(initialState);

    if (initialState.status === "Running") {
        if (!pollingInterval) {
            pollingInterval = window.setInterval(updateMousePosition, 100);
        }
    }
});

onUnmounted(() => {
    if (unlisten) {
        unlisten();
    }
    if (unlistenCancelReq) {
        unlistenCancelReq();
    }
    if (pollingInterval) {
        clearInterval(pollingInterval);
        pollingInterval = undefined;
    }
});
</script>
