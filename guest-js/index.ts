import { invoke } from '@tauri-apps/api/core'

interface CustomAuthData {
	access_token: string,
	uuid?: number,
	expires_in: string,
	xts_token?: string,
}


export async function oauth(client_id: string, client_secret: string, bedrock_rel?: boolean, port?: number): Promise<CustomAuthData> {
	return await invoke<CustomAuthData>('plugin:minecraft-essentails|oauth', {
		client_id,
		client_secret,
		bedrock_rel,
		port
	}).then((r) => (r));
}
