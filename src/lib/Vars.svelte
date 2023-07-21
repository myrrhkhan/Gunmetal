<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	interface variableMap {
		[key: string]: string[];
	}

	// gathers environment variables to display
	async function getPath(): Promise<variableMap> {
		let thing: variableMap = await invoke('get_vars');
		return thing;
	}

	function whileAddingInput(key: String) {
		console.log("box should now show");
		if (keyBeingEdited == "") {
			keyBeingEdited = key;
			boxLabel = "Cancel";
		} else {
			removeBox();
		}
		console.log(key == keyBeingEdited);
	}

	function removeBox() {
		keyBeingEdited = "";
		varSubmission = "";
		boxLabel = "Add Variable";
	}

	// adds a new environment variable
	async function addVar(variable: String, submission: String): Promise<String> {
		let message: String = "";
		console.log("About to receive message")
		await invoke('add_var', { key: variable, varSubmission: submission})
			.then((return_val) => { message = return_val as string })
			.catch((err_msg) => { message = err_msg });
		console.log("about to alert");
		alert(message);
		// alert(message);
		removeBox();
		return message;
	}
	
	let varsPromise = getPath(); // promise of map containing all environment variables

	let keyBeingEdited: String = ""; // key that's being edited
	let varSubmission: String; // environment variable being added
	let boxLabel: String = "Add Variable";

</script>

<h1>Your Computer's Environment Variables:</h1>

{#await varsPromise then allVars}
	{#each Object.keys(allVars) as key}
		{@const values = allVars[key]}
		<h3>{key}</h3>
		{#each values as value}
			<li>{value}</li>
		{/each}
		<button on:click={() => whileAddingInput(key)}>{boxLabel}</button>
		{#if key == keyBeingEdited}
			<form>
				<input bind:value={varSubmission} type="text">
				<button on:click={() => addVar(key, varSubmission)}>Submit</button>
			</form>
		{/if}
	{/each}
{/await}