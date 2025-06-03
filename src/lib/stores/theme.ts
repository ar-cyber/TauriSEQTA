import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/core';

// Create a writable store with the default accent color
export const accentColor = writable('#3b82f6');

// Function to load the accent color from settings
export async function loadAccentColor() {
    try {
        const settings = await invoke<{ accent_color: string }>('get_settings');
        accentColor.set(settings.accent_color || '#3b82f6');
    } catch (e) {
        console.error('Failed to load accent color:', e);
    }
}

// Function to update the accent color
export async function updateAccentColor(color: string) {
    try {
        const settings = await invoke<{ accent_color: string }>('get_settings');
        await invoke('save_settings', {
            newSettings: {
                ...settings,
                accent_color: color
            }
        });
        accentColor.set(color);
    } catch (e) {
        console.error('Failed to update accent color:', e);
    }
} 