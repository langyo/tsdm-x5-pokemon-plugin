import { defineComponent } from 'vue'
import { RouterView } from 'vue-router'
import { useAppStore } from '@/stores'
import LoadingOverlay from '@/components/common/LoadingOverlay'

export default defineComponent({
  name: 'App',
  setup() {
    const app = useAppStore()

    return () => (
      <>
        {app.loading && <LoadingOverlay message={app.loadingMessage} />}
        <RouterView />
      </>
    )
  },
})
