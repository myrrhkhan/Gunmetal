<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	interface variableMap {
		[key: string]: string[];
	}

	// gathers environment variables to display
	async function getPath(): Promise<variableMap> {
		console.log("generating/regenerating");
		let vars: variableMap = await invoke('get_vars');
		return vars;
	}

	function whileAddingInput(key: String) {
		if (keyBeingEdited == "") {
			keyBeingEdited = key;
		} else {
			removeBox();
		}
	}

	function removeBox() {
		keyBeingEdited = "";
		varSubmission = "";
		varsPromise = getPath(); // reload on submission
		// TODO: find a way to only reload one of the thingies?
	}

	// adds a new environment variable
	async function addVar(variable: String, submission: String): Promise<String> {
		let message: String = "";
		await invoke('add_var', { key: variable, varSubmission: submission})
			.then((return_val) => { message = return_val as string })
			.catch((err_msg) => { message = err_msg });
		alert(message);
		// alert(message);
		removeBox();
		return message;
	}

	async function getShellLocation(): Promise<String> {
		return invoke('get_shell_location');
	}
	
	let shellLocationPromise = getShellLocation();

	let varsPromise = getPath(); // promise of map containing all environment variables

	let keyBeingEdited: String = ""; // key that's being edited
	let varSubmission: String; // environment variable being added

    // future colorscheme with Gunmetal name: https://coolors.co/2d3142-bfc0c0-ffffff-ef8354-4f5d75

</script>

<h1>Your Computer's Environment Variables:</h1>

{#await shellLocationPromise then location}
	<p>Reading Environment Variables from {location}</p>
{/await}

{#await varsPromise then allVars}
	{#each Object.keys(allVars) as key}
		{@const values = allVars[key]}
		<h3>{key}</h3>
		{#each values as value}
			<li>{value}</li>
		{/each}
		{#if key == keyBeingEdited}
			<button on:click={() => whileAddingInput(key)}>Cancel</button>
			<form>
				<input bind:value={varSubmission} type="text">
				<button on:click={() => addVar(key, varSubmission)}>Submit</button>
			</form>
		{:else}
			<button on:click={() => whileAddingInput(key)}>Add Variable</button>
		{/if}
	{/each}
{:catch error}
	<p>Could not get variables</p>
	<p>{error}</p>
{/await}

