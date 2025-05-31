import DOMPurify from 'dompurify';
import type { DraftJSContent, LinkPreview } from './types';

// HTML sanitization function using DOMPurify
export function sanitizeHtml(html: string): string {
  return DOMPurify.sanitize(html, {
    ALLOWED_TAGS: ['p', 'br', 'strong', 'em', 'u', 'h1', 'h2', 'h3', 'h4', 'h5', 'h6', 'ul', 'ol', 'li', 'a', 'img', 'blockquote', 'code', 'pre'],
    ALLOWED_ATTR: ['href', 'src', 'alt', 'title', 'class', 'target', 'rel'],
    ALLOWED_URI_REGEXP: /^(?:(?:(?:f|ht)tps?|mailto|tel|callto|cid|xmpp|xxx):|[^a-z]|[a-z+.\-]+(?:[^a-z+.\-:]|$))/i,
    ADD_ATTR: ['target', 'rel'],
    ADD_DATA_URI_TAGS: ['img']
  });
}

export function renderDraftJSText(content: DraftJSContent): string {
  return content.blocks
    .map(block => {
      let text = block.text;
      
      // Apply entity ranges (links, etc.)
      if (block.entityRanges.length > 0) {
        // Sort ranges by offset in reverse order to apply from end to start
        const sortedRanges = [...block.entityRanges].sort((a, b) => b.offset - a.offset);
        
        for (const range of sortedRanges) {
          const entity = content.entityMap[range.key];
          if (entity && entity.type === 'LINK') {
            const before = text.substring(0, range.offset);
            const linkText = text.substring(range.offset, range.offset + range.length);
            const after = text.substring(range.offset + range.length);
            text = `${before}<a href="${entity.data.href || entity.data.url}" class="text-indigo-400 hover:text-purple-300 underline transition-colors" target="_blank" rel="noopener noreferrer">${linkText}</a>${after}`;
          }
        }
      }
      
      // Apply block-level formatting
      switch (block.type) {
        case 'header-one':
          return `<h1 class="text-2xl font-bold mb-4 text-white">${text}</h1>`;
        case 'header-two':
          return `<h2 class="text-xl font-bold mb-3 text-white">${text}</h2>`;
        case 'header-three':
          return `<h3 class="text-lg font-bold mb-2 text-white">${text}</h3>`;
        default:
          return `<p class="mb-2 text-slate-300">${text}</p>`;
      }
    })
    .join('');
}

export function getFileIcon(mimetype: string): string {
  if (mimetype.includes('pdf')) return '📄';
  if (mimetype.includes('image')) return '🖼️';
  if (mimetype.includes('video')) return '🎥';
  if (mimetype.includes('audio')) return '🎵';
  if (mimetype.includes('text')) return '📝';
  if (mimetype.includes('word')) return '📝';
  if (mimetype.includes('excel') || mimetype.includes('spreadsheet')) return '📊';
  if (mimetype.includes('powerpoint') || mimetype.includes('presentation')) return '📽️';
  return '📎';
}

export function getFileColor(mimetype: string): string {
  if (mimetype.includes('pdf')) return 'border-red-500 bg-red-900/20 hover:bg-red-900/40';
  if (mimetype.includes('image')) return 'border-green-500 bg-green-900/20 hover:bg-green-900/40';
  if (mimetype.includes('video')) return 'border-purple-500 bg-purple-900/20 hover:bg-purple-900/40';
  if (mimetype.includes('audio')) return 'border-yellow-500 bg-yellow-900/20 hover:bg-yellow-900/40';
  if (mimetype.includes('word')) return 'border-blue-500 bg-blue-900/20 hover:bg-blue-900/40';
  if (mimetype.includes('excel') || mimetype.includes('spreadsheet')) return 'border-emerald-500 bg-emerald-900/20 hover:bg-emerald-900/40';
  if (mimetype.includes('powerpoint') || mimetype.includes('presentation')) return 'border-orange-500 bg-orange-900/20 hover:bg-orange-900/40';
  return 'border-indigo-500 bg-indigo-900/20 hover:bg-indigo-900/40';
}

export function formatFileSize(size: string): string {
  const bytes = parseInt(size);
  if (isNaN(bytes)) return size;
  
  const units = ['B', 'KB', 'MB', 'GB'];
  let unitIndex = 0;
  let fileSize = bytes;
  
  while (fileSize >= 1024 && unitIndex < units.length - 1) {
    fileSize /= 1024;
    unitIndex++;
  }
  
  return `${fileSize.toFixed(1)} ${units[unitIndex]}`;
}

// Link preview functionality
export async function fetchLinkPreview(url: string): Promise<LinkPreview | null> {
  try {
    // In a real app, you'd want to use a service like linkpreview.net or implement server-side scraping
    // For now, we'll create a mock implementation that extracts basic info from the URL
    const urlObj = new URL(url);
    
    // Mock implementation - in production you'd fetch and parse the actual page
    return {
      title: `Preview of ${urlObj.hostname}`,
      description: `Link to ${urlObj.pathname}`,
      image: '', // Would be extracted from og:image or similar
      url: url
    };
  } catch (error) {
    console.error('Failed to fetch link preview:', error);
    return null;
  }
}

export function isValidUrl(string: string): boolean {
  try {
    new URL(string);
    return true;
  } catch (_) {
    return false;
  }
}

export function formatDate(dateString: string): string {
  try {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-AU', {
      year: 'numeric',
      month: 'short',
      day: 'numeric'
    });
  } catch (error) {
    return dateString;
  }
}

export function formatTime(timeString: string): string {
  try {
    const time = new Date(`2000-01-01T${timeString}`);
    return time.toLocaleTimeString('en-AU', {
      hour: '2-digit',
      minute: '2-digit',
      hour12: false
    });
  } catch (error) {
    return timeString;
  }
} 