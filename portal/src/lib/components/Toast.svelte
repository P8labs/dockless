<script lang="ts" module>
  const t = createToaster({
    placement: "top-end",
  });
  export const toaster = t;
</script>

<script lang="ts">
  import { Toast, createToaster } from "@skeletonlabs/skeleton-svelte";
</script>

<Toast.Group toaster={t}>
  {#snippet children(toast)}
    <Toast {toast}>
      <Toast.Message>
        <Toast.Title>{toast.title}</Toast.Title>
        <Toast.Description>{toast.description}</Toast.Description>
      </Toast.Message>
      <Toast.CloseTrigger />
    </Toast>
  {/snippet}
</Toast.Group>
