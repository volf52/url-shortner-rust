<script lang="ts">
	import superagent, { SuperAgentRequest } from 'superagent';

	let url: string = '';
	let request: SuperAgentRequest = null;

	const click = () => {
		request = superagent.post(`/api/shorten?url=${url}`);
	};

	const getURL = (key: string) => {
		return `http://${window.location.host}/api/get?key=${key}`;
	};
</script>

<div class="box">
	{#if request === null}
		<div class="field has-addons">
			<div class="control">
				<input
					class="input"
					type="text"
					name="url"
					id="urlInput"
					placeholder="URL"
					bind:value={url}
				/>
			</div>
			<div class="control">
				<button class="button is-info" on:click={click}>Shorten</button>
			</div>
		</div>
	{:else}
		{#await request}
			<p>Loading...</p>
		{:then response}
			<div class="card">
				<header class="card-header">
					<p class="card-header-title">Done!</p>
				</header>
				<div class="card-content">
					<a class="content" target="_blank" href={getURL(response.text)}>{getURL(response.text)}</a
					>
				</div>
				<footer class="card-footer">
					<button class="card-footer-item button" on:click={() => (request = null)}>
						Go Back
					</button>
					<button
						class="card-footer-item button is-info"
						on:click={() => navigator.clipboard.writeText(getURL(response.text))}
					>
						Copy
					</button>
				</footer>
			</div>
		{:catch}
			<p>Something went wrong!!</p>
		{/await}
	{/if}
</div>
