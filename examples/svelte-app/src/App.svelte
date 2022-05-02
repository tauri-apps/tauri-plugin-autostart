<script lang="ts">
	import { enable, disable, isEnabled } from 'tauri-plugin-autostart-api'

	let response = ''

	isEnabled().then(flag => updateResponse(`Autostart is ${flag ? 'enabled' : 'disabled'}`)).catch(updateResponse)

	function updateResponse(returnValue) {
		response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	function _enable() {
		enable().then(() => updateResponse('Autostart enabled')).catch(updateResponse)
	}
	function _disable() {
		disable().then(() => updateResponse('Autostart disabled')).catch(updateResponse)
	}
</script>

<div>
	<button on:click="{_enable}">Enable</button>
	<button on:click="{_disable}">Disable</button>
	<div>{@html response}</div>
</div>
