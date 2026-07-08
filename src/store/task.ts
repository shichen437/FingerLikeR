import { reactive } from "vue";

export interface TaskProgress {
  status: "Idle" | "Running" | "Cancelled" | "Finished";
  progress: number;
  total: number;
}

type FrontendStatus = TaskProgress["status"] | "Countdown";

export const taskState = reactive<{
  status: FrontendStatus;
  progress: number;
  total: number;
}>({
  status: "Idle",
  progress: 0,
  total: 0,
});

export function setTaskState(newState: TaskProgress) {
  taskState.status = newState.status;
  taskState.progress = newState.progress;
  taskState.total = newState.total;
  if (newState.status === "Finished" || newState.status === "Cancelled") {
    setTimeout(() => {
      taskState.status = "Idle";
      taskState.progress = 0;
      taskState.total = 0;
    }, 2000);
  }
}
