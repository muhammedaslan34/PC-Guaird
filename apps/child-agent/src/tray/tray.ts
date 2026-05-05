export interface TrayDisplayState {
  connectionLabel: string;
  pairingCode: string | null;
  errorMessage: string | null;
}

export function renderTrayStatus(state: TrayDisplayState): string {
  const pairingSection =
    state.pairingCode != null
      ? `<section class="pairing">
          <p class="label">Pairing code</p>
          <p class="code">${state.pairingCode}</p>
        </section>`
      : "";

  const errorSection =
    state.errorMessage != null
      ? `<p class="error">${state.errorMessage}</p>`
      : "";

  return `
    <header>
      <span class="status-dot" data-status="${state.connectionLabel.toLowerCase()}"></span>
      <span class="status-label">${state.connectionLabel}</span>
    </header>
    ${pairingSection}
    ${errorSection}
  `;
}
