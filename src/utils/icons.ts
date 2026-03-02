// Local icon paths - no external dependencies
export const iconPaths = {
	// Media controls
	'play': '<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="1.5"/><path d="M10 8L16 12L10 16V8Z" fill="currentColor"/>',
	'pause': '<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="1.5"/><path d="M9 7H11V17H9V7ZM13 7H15V17H13V7Z" fill="currentColor"/>',
	'play-circle': '<circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M10 8L14 12L10 16V8Z" fill="currentColor"/>',
	'pause-circle': '<circle cx="12" cy="12" r="9" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M9 8H11V16H9V8ZM13 8H15V16H13V8Z" fill="currentColor"/>',
	'skip-previous': '<path d="M6 6V18M8 12L18 6V18L8 12Z" fill="currentColor"/><circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="1.5" fill="none"/>',
	'skip-next': '<path d="M18 6V18M16 12L6 6V18L16 12Z" fill="currentColor"/><circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="1.5" fill="none"/>',
	
	// Settings
	'settings': '<circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M12 1V3M12 21V23M23 12H21M3 12H1M20.66 3.34L19.07 4.93M4.93 19.07L3.34 20.66M20.66 20.66L19.07 19.07M4.93 4.93L3.34 3.34" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	'shuffle': '<path d="M16 3H21V8M21 3L14 10M21 8L14 15M3 17L9 11L15 17M3 21L9 15" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>',
	'repeat': '<path d="M17 1L21 5L17 9M3 17V7C3 5.89543 3.89543 5 5 5H21M7 23L3 19L7 15M21 7V17C21 18.1046 20.1046 19 19 19H3" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/>',
	
	// Navigation
	'playlist': '<path d="M4 6H20M4 10H20M4 14H14M4 18H14" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/><rect x="14" y="12" width="6" height="6" rx="1" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M16 14V16L17.5 17" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" fill="none"/>',
	'search': '<circle cx="11" cy="11" r="7" stroke="currentColor" stroke-width="1.5" fill="none"/><path d="M20 20L17 17" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	'menu': '<path d="M4 6H20M4 12H20M4 18H20" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	'close': '<path d="M6 6L18 18M6 18L18 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	'back': '<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
	
	// Music
	'vinyl': '<circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="1.5" fill="none"/><circle cx="12" cy="12" r="3" fill="currentColor"/><circle cx="12" cy="12" r="1" fill="currentColor"/>',
	'music': '<path d="M9 18V5L21 3V16" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/><circle cx="6" cy="18" r="3" stroke="currentColor" stroke-width="1.5" fill="none"/><circle cx="18" cy="16" r="3" stroke="currentColor" stroke-width="1.5" fill="none"/>',
	
	// Actions
	'plus': '<path d="M12 5V19M5 12H19" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	'more-vertical': '<circle cx="12" cy="6" r="1.5" fill="currentColor"/><circle cx="12" cy="12" r="1.5" fill="currentColor"/><circle cx="12" cy="18" r="1.5" fill="currentColor"/>',
	'more-horizontal': '<circle cx="6" cy="12" r="1.5" fill="currentColor"/><circle cx="12" cy="12" r="1.5" fill="currentColor"/><circle cx="18" cy="12" r="1.5" fill="currentColor"/>',
	'check': '<path d="M5 12L10 17L19 8" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
	'volume': '<path d="M11 5L6 9H2V15H6L11 19V5Z" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" fill="none"/><path d="M15.54 8.46C16.4774 9.39764 17.0039 10.6692 17.0039 11.995C17.0039 13.3208 16.4774 14.5924 15.54 15.53" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>',
	
	// UI
	'chevron-down': '<path d="M6 9L12 15L18 9" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
	'chevron-up': '<path d="M18 15L12 9L6 15" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
	'chevron-left': '<path d="M15 18L9 12L15 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
	'chevron-right': '<path d="M9 18L15 12L9 6" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>',
};

export type IconName = keyof typeof iconPaths;
