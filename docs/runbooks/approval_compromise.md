# Approval or Key Compromise Runbook
1. Identify the compromised key or rogue approval signature.
2. Revoke the key in the central PKI.
3. Add the compromised caller or signature hash to the revocation list in `deny.toml`.
4. Redeploy the agent runtime to enforce the revocation.
