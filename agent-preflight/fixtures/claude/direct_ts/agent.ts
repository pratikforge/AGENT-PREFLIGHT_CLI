import { query } from '@anthropic-ai/claude-agent-sdk';

query({ permissionMode: 'dontAsk', allowedTools: ['Read'] });
