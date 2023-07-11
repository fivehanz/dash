<script>
  // Your selected Skeleton theme:
  import '@skeletonlabs/skeleton/themes/theme-modern.css';
  // This contains the bulk of Skeletons required styles:
  import '@skeletonlabs/skeleton/styles/skeleton.css';
  // Finally, your application's global stylesheet (sometimes labeled 'app.css')
  import '../app.postcss';
  // authorizer
	import '@authorizerdev/authorizer-svelte/styles/default.css';
  import { AuthorizerProvider } from '@authorizerdev/authorizer-svelte';
  // skeleton ui components
  import { AppShell, AppBar, LightSwitch } from '@skeletonlabs/skeleton';
</script>

<AuthorizerProvider
    config={{
        authorizerURL: `https://d-auth.hanz.lol/`,
        redirectURL: typeof window != 'undefined' ? window.location.origin : ``,
        client_id: 'b221ab06-c89f-4c40-a1da-34fac30ca7fc',
    }}
>
<AppShell>
  <!-- header -->
	<svelte:fragment slot="header">
    <AppBar background="variant-glass-surface">
      <!-- left side -->
      <svelte:fragment slot="lead">
        <h2 class="h2">dash.hanz.lol</h2>
      </svelte:fragment>
      
      <!-- right side -->
      <svelte:fragment slot="trail">
        <div class="flex gap-4 align-baseline">
          <div>
            actions
          </div>
          <LightSwitch />
        </div>
      </svelte:fragment>
    </AppBar>
  </svelte:fragment>
  <!-- end header -->
  
  <!-- body -->
  <slot />
  <!-- end body -->
  
  <!-- footer -->
  <svelte:fragment slot="pageFooter">
    <div class="h-18 variant-soft-surface p-4">
      Page Footer
    </div>
  </svelte:fragment>
  <!-- end footer -->
</AppShell>
</AuthorizerProvider>
