<script lang="ts">
    import { getContext } from 'svelte';
    import { Authorizer } from '@authorizerdev/authorizer-svelte';

    /**
	 * @type {{ token: string; user: any; loading: boolean; logout: Function; }}
	 */
	let state: { token: string; user: any; loading: boolean; logout: Function; };
    
    const store: any = getContext('authorizerContext');
    store.subscribe((/** @type {any} */ data: any) => {
		state = data;
	});

	const logoutHandler = async () => {
		await state.logout();
	};
</script>

<div class="px-4 max-w-lg flex flex-col items-center justify-center mx-auto gap-8 min-h-screen ">
    {#if state.user}
        <div class="flex gap-4">
            <h1 class="h1">Hey 👋,</h1>
            <span>{state.user.email}</span>

            {#if state.loading}
                <h3 class="h3">Processing....</h3>
            {:else}
                <!-- <h3 class="h3" on:click={logoutHandler}>Logout</h3> -->
                <h3 class="h3">Logout</h3>
            {/if}
        </div>
    {:else}
        <h1 class="h1">authenticate</h1>
        <Authorizer />
    {/if}
</div>